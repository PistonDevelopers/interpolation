//! A trait to allow interpolation over spatial structures.

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

/// Implementation of spatial for floats.
macro_rules! impl_spatial_for_float {
    ($float: ident) => (
        impl Spatial for $float {
            type Scalar = $float;

            #[inline(always)]
            fn add(&self, other: &$float) -> $float {
                *self + *other
            }

            #[inline(always)]
            fn sub(&self, other: &$float) -> $float {
                *self - *other
            }

            #[inline(always)]
            fn scale(&self, other: &$float) -> $float {
                *self * *other
            }
        }
    )
}

impl_spatial_for_float!(f32);
impl_spatial_for_float!(f64);

/// Implementation of spatial for signed integers.
/// `scale` will cast the int to the Scalar before multiplying and rounding to the nearest value.
macro_rules! impl_spatial_for_int {
    ($int: ident, $scalar: ident) => (
        impl Spatial for $int {
            type Scalar = $scalar;

            #[inline(always)]
            fn add(&self, other: &$int) -> $int {
                *self + *other
            }

            #[inline(always)]
            fn sub(&self, other: &$int) -> $int {
                *self - *other
            }

            #[inline(always)]
            fn scale(&self, other: &$scalar) -> $int {
                ((*self as $scalar) * *other).round() as $int
            }
        }
    )
}

impl_spatial_for_int!(i8,  f32);
impl_spatial_for_int!(i16, f32);
impl_spatial_for_int!(i32, f32);
impl_spatial_for_int!(i64, f64);

/// Implementation of spatial for unsigned integers.
/// `scale` will cast the uint to the Scalar before multiplying and rounding to the nearest value.
macro_rules! impl_spatial_for_uint {
    ($uint: ident, $scalar: ident) => (
        impl Spatial for $uint {
            type Scalar = $scalar;

            #[inline(always)]
            fn add(&self, other: &$uint) -> $uint {
                *self + *other
            }

            #[inline(always)]
            fn sub(&self, other: &$uint) -> $uint {
                // Find the absolute difference to avoid underflow.
                if *self >= *other {
                    *self - *other
                } else {
                    *other - *self
                }
            }

            #[inline(always)]
            fn scale(&self, other: &$scalar) -> $uint {
                ((*self as $scalar) * *other).round() as $uint
            }
        }
    )
}

impl_spatial_for_uint!(u8,  f32);
impl_spatial_for_uint!(u16, f32);
impl_spatial_for_uint!(u32, f32);
impl_spatial_for_uint!(u64, f64);

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
