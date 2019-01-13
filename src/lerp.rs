//! Linear interpolation

/// Performs linear interpolation.
/// A linear interpolation consists of two states 'a' and 'b'.
/// The 't' variable is a factor between 0 and 1 that
/// gives weight to 'a' or 'b'.
/// When 't' is zero then 'a' has full weight.
/// When 't' is one then 'b' has full weight.
#[inline(always)]
pub fn lerp<T: Lerp>(a: &T, b: &T, t: &T::Scalar) -> T {
    a.lerp(b, t)
}

/// Describes a type that can linearly interpolate between two points.
pub trait Lerp {
    /// The scaling type for linear interpolation.
    type Scalar;

    /// Given `self` and another point `other`, return a point on a line running between the two
    /// that is `scalar` fraction of the distance between the two points.
    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self;
}

/// Implementation of `Lerp` for floats.
macro_rules! impl_lerp_for_float {
    ($float: ident) => (
        impl Lerp for $float {
            type Scalar = $float;

            #[inline(always)]
            fn lerp(&self, other: &$float, scalar: &$float) -> $float {
                self + (other - self) * scalar
            }
        }
    )
}

impl_lerp_for_float!(f32);
impl_lerp_for_float!(f64);

/// Implementation of `Lerp` for signed integers.
/// This will cast the int to the Scalar before multiplying and rounding to the nearest value.
macro_rules! impl_lerp_for_int {
    ($int: ident, $scalar: ident) => (
        impl Lerp for $int {
            type Scalar = $scalar;

            #[inline(always)]
            fn lerp(&self, other: &$int, scalar: &$scalar) -> $int {
                self + ((other - self) as $scalar * scalar).round() as $int
            }
        }
    )
}

impl_lerp_for_int!(i8,  f32);
impl_lerp_for_int!(i16, f32);
impl_lerp_for_int!(i32, f32);
impl_lerp_for_int!(i64, f64);

/// Implementation of `Lerp` for unsigned integers.
/// Will cast the uint to the Scalar before multiplying and rounding to the nearest value.
macro_rules! impl_lerp_for_uint {
    ($uint: ident, $scalar: ident) => (
        impl Lerp for $uint {
            type Scalar = $scalar;

            #[inline(always)]
            fn lerp(&self, other: &$uint, scalar: &$scalar) -> $uint {
                if self <= other {
                    self + ((other - self) as $scalar * scalar).round() as $uint
                } else {
                    self - ((self - other) as $scalar * scalar).round() as $uint
                }
            }
        }
    )
}

impl_lerp_for_uint!(u8,  f32);
impl_lerp_for_uint!(u16, f32);
impl_lerp_for_uint!(u32, f32);
impl_lerp_for_uint!(u64, f64);

/// Transitive impl of `Lerp` for arrays, given a length and index list
macro_rules! impl_lerp_for_array {
    ($len:expr; $($i:expr),*) => {
        impl<T> Lerp for [T; $len] where T: Lerp {
            type Scalar = T::Scalar;
            
            #[inline(always)]
            fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
                [
                    $(self[$i].lerp(&other[$i], scalar)),*
                ]
            }
        }
    }
}

impl_lerp_for_array!(1; 0);
impl_lerp_for_array!(2; 0, 1);
impl_lerp_for_array!(3; 0, 1, 2);
impl_lerp_for_array!(4; 0, 1, 2, 3);
impl_lerp_for_array!(5; 0, 1, 2, 3, 4);

#[test]
fn lerp_f32() {
    for x in 0 ..= 10 {
        let w = x as f32 / 10f32;
        assert_eq!(lerp(&0f32, &10f32, &w), x as f32);
    }
    
    for x in (0 ..= 10).rev() {
        let w = (10 - x) as f32 / 10f32;
        assert_eq!(lerp(&10f32, &0f32, &w), x as f32);
    }

    for x in -10 .. 0 {
        let w = (10 + x) as f32 / 10f32;
        assert_eq!(lerp(&-10f32, &0f32, &w), x as f32);
    }

    for x in (-10 .. 0).rev() {
        let w = (0 - x) as f32 / 10f32;
        assert_eq!(lerp(&0f32, &-10f32, &w), x as f32);
    }
}

#[test]
fn lerp_f64() {
    for x in 0 ..= 10 {
        let w = x as f64 / 10f64;
        assert_eq!(lerp(&0f64, &10f64, &w), x as f64);
    }

    for x in (0 ..= 10).rev() {
        let w = (10 - x) as f64 / 10f64;
        assert_eq!(lerp(&10f64, &0f64, &w), x as f64);
    }

    for x in -10 .. 0 {
        let w = (10 + x) as f64 / 10f64;
        assert_eq!(lerp(&-10f64, &0f64, &w), x as f64);
    }

    for x in (-10 .. 0).rev() {
        let w = (0 - x) as f64 / 10f64;
        assert_eq!(lerp(&0f64, &-10f64, &w), x as f64);
    }
}

#[test]
fn lerp_i8() {
    for x in 0 ..= 10 {
        let w = x as f32 / 10f32;
        assert_eq!(lerp(&0i8, &10i8, &w), x);
    }

    for x in (0 ..= 10).rev() {
        let w = (10 - x) as f32 / 10f32;
        assert_eq!(lerp(&10i8, &0i8, &w), x);
    }
    
    for x in -10 .. 0 {
        let w = (10 + x) as f32 / 10f32;
        assert_eq!(lerp(&-10i8, &0i8, &w), x);
    }

    for x in (-10 .. 0).rev() {
        let w = (0 - x) as f32 / 10f32;
        assert_eq!(lerp(&0i8, &-10i8, &w), x);
    }
}

#[test]
fn lerp_i16() {
    for x in 0 ..= 10 {
        let w = x as f32 / 10f32;
        assert_eq!(lerp(&0i16, &10i16, &w), x);
    }

    for x in (0 ..= 10).rev() {
        let w = (10 - x) as f32 / 10f32;
        assert_eq!(lerp(&10i16, &0i16, &w), x);
    }

    for x in -10 .. 0 {
        let w = (10 + x) as f32 / 10f32;
        assert_eq!(lerp(&-10i16, &0i16, &w), x);
    }

    for x in (-10 .. 0).rev() {
        let w = (0 - x) as f32 / 10f32;
        assert_eq!(lerp(&0i16, &-10i16, &w), x);
    }
}

#[test]
fn lerp_i32() {
    for x in 0 ..= 10 {
        let w = x as f32 / 10f32;
        assert_eq!(lerp(&0i32, &10i32, &w), x);
    }

    for x in (0 ..= 10).rev() {
        let w = (10 - x) as f32 / 10f32;
        assert_eq!(lerp(&10i32, &0i32, &w), x);
    }

    for x in -10 .. 0 {
        let w = (10 + x) as f32 / 10f32;
        assert_eq!(lerp(&-10i32, &0i32, &w), x);
    }

    for x in (-10 .. 0).rev() {
        let w = (0 - x) as f32 / 10f32;
        assert_eq!(lerp(&0i32, &-10i32, &w), x);
    }
}

#[test]
fn lerp_i64() {
    for x in 0 ..= 10 {
        let w = x as f64 / 10f64;
        assert_eq!(lerp(&0i64, &10i64, &w), x);
    }

    for x in (0 ..= 10).rev() {
        let w = (10 - x) as f64 / 10f64;
        assert_eq!(lerp(&10i64, &0i64, &w), x);
    }

    for x in -10 .. 0 {
        let w = (10 + x) as f64 / 10f64;
        assert_eq!(lerp(&-10i64, &0i64, &w), x);
    }

    for x in (-10 .. 0).rev() {
        let w = (0 - x) as f64 / 10f64;
        assert_eq!(lerp(&0i64, &-10i64, &w), x);
    }
}

#[test]
fn lerp_u8() {
    for x in 0 ..= 10 {
        let w = x as f32 / 10f32;
        assert_eq!(lerp(&0u8, &10u8, &w), x);
    }

    for x in (0 ..= 10).rev() {
        let w = (10 - x) as f32 / 10f32;
        assert_eq!(lerp(&10u8, &0u8, &w), x);
    }
}

#[test]
fn lerp_u16() {
    for x in 0 ..= 10 {
        let w = x as f32 / 10f32;
        assert_eq!(lerp(&0u16, &10u16, &w), x);
    }

    for x in (0 ..= 10).rev() {
        let w = (10 - x) as f32 / 10f32;
        assert_eq!(lerp(&10u16, &0u16, &w), x);
    }
}

#[test]
fn lerp_u32() {
    for x in 0 ..= 10 {
        let w = x as f32 / 10f32;
        assert_eq!(lerp(&0u32, &10u32, &w), x);
    }

    for x in (0 ..= 10).rev() {
        let w = (10 - x) as f32 / 10f32;
        assert_eq!(lerp(&10u32, &0u32, &w), x);
    }
}

#[test]
fn lerp_u64() {
    for x in 0 ..= 10 {
        let w = x as f64 / 10f64;
        assert_eq!(lerp(&0u64, &10u64, &w), x);
    }

    for x in (0 ..= 10).rev() {
        let w = (10 - x) as f64 / 10f64;
        assert_eq!(lerp(&10u64, &0u64, &w), x);
    }
}

#[test]
fn lerp_array_2() {
    for x in 0 ..= 10 {
        let w = x as f32 / 10f32;
        let pt = lerp(&[0, 0], &[10, 10], &w);
        // slope should be 1/1
        assert_eq!(pt, [x, x]);
    }

    for x in (0 ..= 10).rev() {
        let w = (10 - x) as f32 / 10f32;
        let pt = lerp(&[10, 10], &[0, 0], &w);
        // slope should be -1/1
        assert_eq!(pt, [x, x]);
    }
}

#[test]
fn lerp_array_3() {
    for x in 0 ..= 10 {
        let w = x as f32 / 10f32;
        let pt = lerp(&[0, 0, 0], &[10, 10, 10], &w);
        // slope should be 1/1/1
        assert_eq!(pt, [x, x, x]);
    }

    for x in (0 ..= 10).rev() {
        let w = (10 - x) as f32 / 10f32;
        let pt = lerp(&[10, 10, 10], &[0, 0, 0], &w);
        // slope should be -(1/1/1)
        assert_eq!(pt, [x, x, x]);
    }
}

#[test]
fn lerp_array_4() {
    for x in 0 ..= 10 {
        let w = x as f32 / 10f32;
        let pt = lerp(&[0, 0, 0, 0], &[10, 10, 10, 10], &w);
        // slope should be 1/1/1/1
        assert_eq!(pt, [x, x, x, x]);
    }

    for x in (0 ..= 10).rev() {
        let w = (10 - x) as f32 / 10f32;
        let pt = lerp(&[10, 10, 10, 10], &[0, 0, 0, 0], &w);
        // slope should be -(1/1/1/1)
        assert_eq!(pt, [x, x, x, x]);
    }
}

#[test]
fn lerp_array_5() {
    for x in 0 ..= 10 {
        let w = x as f32 / 10f32;
        let pt = lerp(&[0, 0, 0, 0, 0], &[10, 10, 10, 10, 10], &w);
        // slope should be 1/1/1/1/1
        assert_eq!(pt, [x, x, x, x, x]);
    }

    for x in (0 ..= 10).rev() {
        let w = (10 - x) as f32 / 10f32;
        let pt = lerp(&[10, 10, 10, 10, 10], &[0, 0, 0, 0, 0], &w);
        // slope should be -(1/1/1/1/1)
        assert_eq!(pt, [x, x, x, x, x]);
    }
}
