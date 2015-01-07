
//! A module contains implementation of ease functions.

use std::f64::consts::{
    PI,
    PI_2,
};
use std::num::{
    Float,
    FromPrimitive,
};

#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq)]
pub enum EaseFunction {
    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,

    CubicIn,
    CubicOut,
    CubicInOut,

    QuarticIn,
    QuarticOut,
    QuarticInOut,

    QuinticIn,
    QuinticOut,
    QuinticInOut,

    SineIn,
    SineOut,
    SineInOut,

    CircularIn,
    CircularOut,
    CircularInOut,

    ExponentialIn,
    ExponentialOut,
    ExponentialInOut,

    ElasticIn,
    ElasticOut,
    ElasticInOut,

    BackIn,
    BackOut,
    BackInOut,

    BounceIn,
    BounceOut,
    BounceInOut,
}

impl EaseFunction {
    /// Calculate the eased value, normalized
    pub fn calc<T>(self, p: T) -> T
        where
            T: Float + FromPrimitive
    {
        match self {
            EaseFunction::QuadraticIn => quadratic_in(p),
            EaseFunction::QuadraticOut => quadratic_out(p),
            EaseFunction::QuadraticInOut => quadratic_in_out(p),

            EaseFunction::CubicIn => cubic_in(p),
            EaseFunction::CubicOut => cubic_out(p),
            EaseFunction::CubicInOut => cubic_in_out(p),

            EaseFunction::QuarticIn => quartic_in(p),
            EaseFunction::QuarticOut => quartic_out(p),
            EaseFunction::QuarticInOut => quartic_in_out(p),

            EaseFunction::QuinticIn => quintic_in(p),
            EaseFunction::QuinticOut => quintic_out(p),
            EaseFunction::QuinticInOut => quintic_in_out(p),

            EaseFunction::SineIn => sine_in(p),
            EaseFunction::SineOut => sine_out(p),
            EaseFunction::SineInOut => sine_in_out(p),

            EaseFunction::CircularIn => circular_in(p),
            EaseFunction::CircularOut => circular_out(p),
            EaseFunction::CircularInOut => circular_in_out(p),

            EaseFunction::ExponentialIn => exponential_in(p),
            EaseFunction::ExponentialOut => exponential_out(p),
            EaseFunction::ExponentialInOut => exponential_in_out(p),

            EaseFunction::ElasticIn => elastic_in(p),
            EaseFunction::ElasticOut => elastic_out(p),
            EaseFunction::ElasticInOut => elastic_in_out(p),

            EaseFunction::BackIn => back_in(p),
            EaseFunction::BackOut => back_out(p),
            EaseFunction::BackInOut => back_in_out(p),

            EaseFunction::BounceIn => bounce_in(p),
            EaseFunction::BounceOut => bounce_out(p),
            EaseFunction::BounceInOut => bounce_in_out(p),
        }
    }
}


/// Applies EaseQuadraticIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quadratic_in<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    p * p
}

/// Applies EaseQuadraticOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quadratic_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _2: T = FromPrimitive::from_f64(2.0).unwrap();
    -(p * (p - _2))
}

/// Applies EaseQuadraticInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quadratic_in_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _05: T = FromPrimitive::from_f64(0.5).unwrap();
    let _2: T = FromPrimitive::from_f64(2.0).unwrap();
    let _4: T = FromPrimitive::from_f64(4.0).unwrap();
    let _1: T = Float::one();
    if p < _05 {
        p * p * _2
    } else {
        (-_2 * p * p) + (_4 * p) - _1
    }
}


/// Applies EaseCubicIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn cubic_in<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    p * p * p
}

/// Applies EaseCubicOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn cubic_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _1: T = Float::one();
    let f = p - _1;
    f * f * f + _1
}

/// Applies EaseCubicInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn cubic_in_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _05: T = FromPrimitive::from_f64(0.5).unwrap();
    let _4: T = FromPrimitive::from_f64(4.0).unwrap();
    let _2: T = FromPrimitive::from_f64(2.0).unwrap();
    let _1: T = Float::one();
    if p < _05 {
        p * p * p * _4
    } else {
        let f = (_2 * p) - _2;
        f * f * f * _05 + _1
    }
}


/// Applies EaseQuarticIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quartic_in<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    p * p * p * p
}

/// Applies EaseQuarticOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quartic_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _1: T = Float::one();
    let f = p - _1;
    f * f * f * (_1 - p) + _1
}

/// Applies EaseQuarticInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quartic_in_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _8: T = FromPrimitive::from_f64(8.0).unwrap();
    let _1: T = Float::one();
    let _05: T = FromPrimitive::from_f64(0.5).unwrap();
    if p < _05 {
        _8 * p * p * p * p
    } else {
        let f = p - _1;
        -_8 * f * f * f * f + _1
    }
}


/// Applies EaseQuinticIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quintic_in<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    p * p * p * p * p
}

/// Applies EaseQuinticOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quintic_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _1: T = Float::one();
    let f = p - _1;
    f * f * f * f * f + _1
}

/// Applies EaseQuinticInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quintic_in_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _05: T = FromPrimitive::from_f64(0.5).unwrap();
    let _16: T = FromPrimitive::from_f64(16.0).unwrap();
    let _2: T = FromPrimitive::from_f64(2.0).unwrap();
    let _1: T = Float::one();
    if p < _05  {
        p * p * p * p * p * _16
    } else {
        let f = (_2 * p) - _2;
        f * f * f * f * f * _05 + _1
    }
}


/// Applies EaseSineIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn sine_in<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _1: T = Float::one();
    let _pi_2: T = FromPrimitive::from_f64(PI_2).unwrap();
    ((p - _1) * _pi_2).sin() + _1
}

/// Applies EaseSineOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn sine_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _pi_2: T = FromPrimitive::from_f64(PI_2).unwrap();
    (p * _pi_2).sin()
}

/// Applies EaseSineInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn sine_in_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _05: T = FromPrimitive::from_f64(0.5).unwrap();
    let _1: T = Float::one();
    let _pi: T = FromPrimitive::from_f64(PI).unwrap();
    _05 * (_1 - (p * _pi).cos())
}


/// Applies EaseCircularIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn circular_in<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _1: T = Float::one();
    _1 - (_1 - (p * p)).sqrt()
}

/// Applies EaseCircularOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn circular_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _2: T = FromPrimitive::from_f64(2.0).unwrap();
    ((_2 - p) * p).sqrt()
}

/// Applies EaseCircularInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn circular_in_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _05: T = FromPrimitive::from_f64(0.5).unwrap();
    let _1: T = Float::one();
    let _4: T = FromPrimitive::from_f64(4.0).unwrap();
    let _2: T = FromPrimitive::from_f64(2.0).unwrap();
    let _3: T = FromPrimitive::from_f64(3.0).unwrap();
    if p < _05 {
        _05 * (_1 - (_1 - _4 * (p * p)).sqrt())
    } else {
        _05 * ((-((_2 * p) - _3) * ((_2 * p) - _1)).sqrt() + _1)
    }
}


/// Applies EaseExponentialIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn exponential_in<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _0: T = Float::zero();
    let _2: T = FromPrimitive::from_f64(2.0).unwrap();
    let _10: T = FromPrimitive::from_f64(10.0).unwrap();
    let _1: T = Float::one();
    if p == _0 {
        p
    } else {
        _2.powf(_10 * (p - _1))
    }
}

/// Applies EaseExponentialOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn exponential_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _1: T = Float::one();
    let _2: T = FromPrimitive::from_f64(2.0).unwrap();
    let _10: T = FromPrimitive::from_f64(10.0).unwrap();
    if p == _1 {
        p
    } else {
        _1 - _2.powf(-_10 * p)
    }
}

/// Applies EaseExponentialInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn exponential_in_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _0: T = Float::one();
    let _1: T = Float::one();
    if p == _0 || p == _1 {
        return p;
    }

    let _05: T = FromPrimitive::from_f64(0.5).unwrap();
    let _2: T = FromPrimitive::from_f64(2.0).unwrap();
    let _20: T = FromPrimitive::from_f64(20.0).unwrap();
    let _10: T = FromPrimitive::from_f64(10.0).unwrap();
    if p < _05  {
        _05 * _2.powf((_20 * p) - _10)
    } else {
        -_05 * _2.powf((-_20 * p) + _10) + _1
    }
}


/// Applies EaseElasticIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn elastic_in<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _13: T = FromPrimitive::from_f64(13.0).unwrap();
    let _pi_2: T = FromPrimitive::from_f64(PI_2).unwrap();
    let _2: T = FromPrimitive::from_f64(2.0).unwrap();
    let _10: T = FromPrimitive::from_f64(10.0).unwrap();
    let _1: T = Float::one();
    (_13 * _pi_2 * p).sin() * _2.powf(_10 * (p - _1))
}

/// Applies EaseElasticOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn elastic_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _13: T = FromPrimitive::from_f64(13.0).unwrap();
    let _10: T = FromPrimitive::from_f64(10.0).unwrap();
    let _2: T = FromPrimitive::from_f64(2.0).unwrap();
    let _pi_2: T = FromPrimitive::from_f64(PI_2).unwrap();
    let _1: T = Float::one();
    (-_13 * _pi_2 * (p + _1)).sin() * _2.powf(-_10 * p) + _1
}

/// Applies EaseElasticInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn elastic_in_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _05: T = FromPrimitive::from_f64(0.5).unwrap();
    let _13: T = FromPrimitive::from_f64(13.0).unwrap();
    let _pi_2: T = FromPrimitive::from_f64(PI_2).unwrap();
    let _2: T = FromPrimitive::from_f64(2.0).unwrap();
    let _10: T = FromPrimitive::from_f64(10.0).unwrap();
    let _1: T = Float::one();
    if p < _05 {
        _05 * (_13 * _pi_2 * (_2 * p)).sin() * _2.powf(_10 * ((_2 * p) - _1))
    } else {
        _05 * ((-_13 * _pi_2 * ((_2 * p - _1) + _1)).sin() * _2.powf(-_10 * (_2 * p - _1)) + _2)
    }
}


/// Applies EaseBackIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn back_in<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _pi = FromPrimitive::from_f64(PI).unwrap();
    p * p * p - p * (p * _pi).sin()
}

/// Applies EaseBackOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn back_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _1: T = Float::one();
    let _pi: T = FromPrimitive::from_f64(PI).unwrap();
    let f = _1 - p;
    _1 - (f * f * f - f * (f * _pi).sin())
}

/// Applies EaseBackInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn back_in_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _05: T = FromPrimitive::from_f64(0.5).unwrap();
    let _2: T = FromPrimitive::from_f64(2.0).unwrap();
    let _pi: T = FromPrimitive::from_f64(PI).unwrap();
    let _1: T = Float::one();
    if p < _05 {
        let f = _2 * p;
        _05 * (f * f * f - f * (f * _pi).sin())
    } else {
        let f = _1 - (_2 * p - _1);
        _05 * (_1 - (f * f * f - f * (f * _pi).sin())) + _05
    }
}


/// Applies EaseBounceIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn bounce_in<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _1: T = Float::one();
    _1 - bounce_out(_1 - p)
}

/// Applies EaseBounceOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn bounce_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _4: T = FromPrimitive::from_f64(4.0).unwrap();
    let _11: T = FromPrimitive::from_f64(11.0).unwrap();
    let _121: T = FromPrimitive::from_f64(121.0).unwrap();
    let _16: T = FromPrimitive::from_f64(16.0).unwrap();
    let _8: T = FromPrimitive::from_f64(8.0).unwrap();
    let _11: T = FromPrimitive::from_f64(11.0).unwrap();
    let _363: T = FromPrimitive::from_f64(363.0).unwrap();
    let _40: T = FromPrimitive::from_f64(40.0).unwrap();
    let _99: T = FromPrimitive::from_f64(99.0).unwrap();
    let _10: T = FromPrimitive::from_f64(10.0).unwrap();
    let _17: T = FromPrimitive::from_f64(17.0).unwrap();
    let _5: T = FromPrimitive::from_f64(5.0).unwrap();
    let _9: T = FromPrimitive::from_f64(9.0).unwrap();
    let _4356: T = FromPrimitive::from_f64(4356.0).unwrap();
    let _361: T = FromPrimitive::from_f64(361.0).unwrap();
    let _35442: T = FromPrimitive::from_f64(35442.0).unwrap();
    let _1805: T = FromPrimitive::from_f64(1805.0).unwrap();
    let _16061: T = FromPrimitive::from_f64(16061.0).unwrap();
    let _54: T = FromPrimitive::from_f64(54.0).unwrap();
    let _513: T = FromPrimitive::from_f64(513.0).unwrap();
    let _25: T = FromPrimitive::from_f64(25.0).unwrap();
    let _268: T = FromPrimitive::from_f64(268.0).unwrap();
    if p < _4 / _11 {
        (_121 * p * p) / _16
    } else if p < _8 / _11 {
        (_363 / _40 * p * p) - (_99 / _10 * p) + _17 / _5
    } else if p < _9 / _10 {
        (_4356 / _361 * p * p) - (_35442 / _1805 * p) + _16061 / _1805
    } else {
        (_54 / _5 * p * p) - (_513 / _25 * p) + _268 / _25
    }
}

/// Applies EaseBounceInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn bounce_in_out<T>(mut p: T) -> T
    where
        T: Float + FromPrimitive
{
    p = normalized(p);
    let _05: T = FromPrimitive::from_f64(0.5).unwrap();
    let _2: T = FromPrimitive::from_f64(2.0).unwrap();
    let _1: T = FromPrimitive::from_f64(1.0).unwrap();
    if p < _05 {
        _05 * bounce_in(p * _2)
    } else {
        _05 * bounce_out(p * _2 - _1) + _05
    }
}

fn normalized<T>(p: T) -> T
    where
        T: Float + FromPrimitive
{
    let _1 = Float::one();
    let _0 = Float::zero();
    if p > _1 {
        _1
    } else if p < _0 {
        _0
    } else {
        p
    }
}
