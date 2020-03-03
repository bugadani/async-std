use core::future::Future;
use core::pin::Pin;

use crate::stream::Stream;

/// Trait to represent types that can be created by summing up a stream.
///
/// This trait is used to implement the [`sum`] method on streams. Types which
/// implement the trait can be generated by the [`sum`] method. Like
/// [`FromStream`] this trait should rarely be called directly and instead
/// interacted with through [`Stream::sum`].
///
/// [`sum`]: trait.Sum.html#tymethod.sum
/// [`FromStream`]: trait.FromStream.html
/// [`Stream::sum`]: trait.Stream.html#method.sum
pub trait Sum<A = Self>: Sized {
    /// Method which takes a stream and generates `Self` from the elements by
    /// "summing up" the items.
    fn sum<'a, S>(stream: S) -> Pin<Box<dyn Future<Output = Self> + 'a>>
    where
        S: Stream<Item = A> + 'a;
}

use crate::stream::stream::StreamExt;
use core::num::Wrapping;
use core::ops::Add;

macro_rules! integer_sum {
    (@impls $zero: expr, $($a:ty)*) => ($(
        impl Sum for $a {
            fn sum<'a, S>(stream: S) -> Pin<Box<dyn Future<Output = Self>+ 'a>>
            where
                S: Stream<Item = $a> + 'a,
            {
                Box::pin(async move { stream.fold($zero, Add::add).await } )
            }
        }
        impl<'a> Sum<&'a $a> for $a {
            fn sum<'b, S>(stream: S) -> Pin<Box<dyn Future<Output = Self> + 'b>>
            where
                S: Stream<Item = &'a $a> + 'b,
            {
                Box::pin(async move { stream.fold($zero, Add::add).await } )
            }
        }
    )*);
    ($($a:ty)*) => (
        integer_sum!(@impls 0, $($a)*);
        integer_sum!(@impls Wrapping(0), $(Wrapping<$a>)*);
    );
}

macro_rules! float_sum {
    ($($a:ty)*) => ($(
        impl Sum for $a {
            fn sum<'a, S>(stream: S) -> Pin<Box<dyn Future<Output = Self> + 'a>>
                where S: Stream<Item = $a> + 'a,
            {
                Box::pin(async move { stream.fold(0.0, |a, b| a + b).await } )
            }
        }
        impl<'a> Sum<&'a $a> for $a {
            fn sum<'b, S>(stream: S) -> Pin<Box<dyn Future<Output = Self> + 'b>>
                where S: Stream<Item = &'a $a> + 'b,
            {
                Box::pin(async move { stream.fold(0.0, |a, b| a + b).await } )
            }
        }
    )*);
    ($($a:ty)*) => (
        float_sum!(@impls 0.0, $($a)*);
        float_sum!(@impls Wrapping(0.0), $(Wrapping<$a>)*);
    );
}

integer_sum! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
float_sum! { f32 f64 }
