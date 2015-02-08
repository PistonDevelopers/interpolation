//! A trait to allow interpolation over spatial structures.

use std::num::Float;

/// Used for interpolation over spatial structures.
pub trait Spatial {
    /// The scalar type.
    type Scalar;

    /// Add
    fn add(&self, other: &Self) -> Self;
    /// Subtract
    fn sub(&self, other: &Self) -> Self;
    /// Scales with a scalar.
    fn scale(&self, scalar: &<Self as Spatial>::Scalar) -> Self;
}

impl<T> Spatial for T
    where
        T: Float
{
    type Scalar = T;

    #[inline(always)]
    fn add(&self, other: &Self) -> Self {
        *self + *other
    }

    #[inline(always)]
    fn sub(&self, other: &Self) -> Self {
        *self - *other
    }

    #[inline(always)]
    fn scale(&self, other: &<Self as Spatial>::Scalar) -> Self {
        *self * *other
    }
}

impl<T> Spatial for [T; 2]
    where
        T: Spatial
{
    type Scalar = <T as Spatial>::Scalar;

    #[inline(always)]
    fn add(&self, other: &Self) -> Self {
        [
            self[0].add(&other[0]),
            self[1].add(&other[1])
        ]
    }

    #[inline(always)]
    fn sub(&self, other: &Self) -> Self {
        [
            self[0].sub(&other[0]),
            self[1].sub(&other[1])
        ]
    }

    #[inline(always)]
    fn scale(&self, scalar: &<Self as Spatial>::Scalar) -> Self {
        [
            self[0].scale(scalar),
            self[1].scale(scalar),
        ]
    }
}

impl<T> Spatial for [T; 3]
    where
        T: Spatial
{
    type Scalar = <T as Spatial>::Scalar;

    #[inline(always)]
    fn add(&self, other: &Self) -> Self {
        [
            self[0].add(&other[0]),
            self[1].add(&other[1]),
            self[2].add(&other[2])
        ]
    }

    #[inline(always)]
    fn sub(&self, other: &Self) -> Self {
        [
            self[0].sub(&other[0]),
            self[1].sub(&other[1]),
            self[2].sub(&other[2])
        ]
    }

    #[inline(always)]
    fn scale(&self, scalar: &<Self as Spatial>::Scalar) -> Self {
        [
            self[0].scale(scalar),
            self[1].scale(scalar),
            self[2].scale(scalar)
        ]
    }
}

impl<T> Spatial for [T; 4]
    where
        T: Spatial
{
    type Scalar = <T as Spatial>::Scalar;

    #[inline(always)]
    fn add(&self, other: &Self) -> Self {
        [
            self[0].add(&other[0]),
            self[1].add(&other[1]),
            self[2].add(&other[2]),
            self[3].add(&other[3])
        ]
    }

    #[inline(always)]
    fn sub(&self, other: &Self) -> Self {
        [
            self[0].sub(&other[0]),
            self[1].sub(&other[1]),
            self[2].sub(&other[2]),
            self[3].sub(&other[3])
        ]
    }

    #[inline(always)]
    fn scale(&self, scalar: &<Self as Spatial>::Scalar) -> Self {
        [
            self[0].scale(scalar),
            self[1].scale(scalar),
            self[2].scale(scalar),
            self[3].scale(scalar)
        ]
    }
}
