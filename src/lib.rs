#![deny(missing_docs)]

//! Interpolation algorithms.
//!
//! Interpolation is used in animation,
//! to describe smooth shapes and to make transitions.
//! Any object that fullfill certain mathematical
//! properties can be interpolated.
//! A common technique is using one or more 'numbers'
//! controlling the mixture of states.
//! The choice of interpolation algorithm depends often
//! on the circumstances where it used.

pub use ease::{ Ease, EaseFunction };
pub use lerp::{lerp, Lerp};

mod ease;
mod lerp;

/// Performs quadratic beziér interpolation.
/// This is done by nesting linear interpolations.
/// For more information, see:
///
/// [Beziér Curve at Wikipedia](http://en.wikipedia.org/wiki/B%C3%A9zier_curve)
#[inline(always)]
pub fn quad_bez<T: Lerp>(
    x0: &T,
    x1: &T,
    x2: &T,
    t: &T::Scalar
) -> T {
    let x_0_1 = lerp(x0, x1, t);
    let x_1_2 = lerp(x1, x2, t);
    lerp(&x_0_1, &x_1_2, t)
}

/// Performs cubic beziér interpolation.
/// This is done by interpolation between two quadratic beziér.
/// For more information, see:
///
/// [Beziér Curve at Wikipedia](http://en.wikipedia.org/wiki/B%C3%A9zier_curve)
#[inline(always)]
pub fn cub_bez<T: Lerp>(
    x0: &T,
    x1: &T,
    x2: &T,
    x3: &T,
    t: &T::Scalar
) -> T {
    let x_0_2 = quad_bez(x0, x1, x2, t);
    let x_1_3 = quad_bez(x1, x2, x3, t);
    lerp(&x_0_2, &x_1_3, t)
}
