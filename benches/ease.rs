#![feature(test)]

extern crate test;
extern crate interpolation;

use interpolation::Ease;

macro_rules! bench_ease {
    ($name: ident, $T: ident, $f: ident) => (
        #[bench]
        fn $name(bencher: &mut test::Bencher) {
            let values = (0..11).map(|x| x as $T / 10.0).collect::<Vec<$T>>();
            bencher.iter(|| {
                for x in values.iter().map(|&x| Ease::$f(x)) {
                    test::black_box(x);
                }
            })
        }
    )
}

bench_ease!(bench_quadratic_in_f32, f32, quadratic_in);
bench_ease!(bench_quadratic_in_f64, f64, quadratic_in);

bench_ease!(bench_quadratic_out_f32, f32, quadratic_out);
bench_ease!(bench_quadratic_out_f64, f64, quadratic_out);

bench_ease!(bench_quadratic_in_out_f32, f32, quadratic_in_out);
bench_ease!(bench_quadratic_in_out_f64, f64, quadratic_in_out);

bench_ease!(bench_cubic_in_f32, f32, cubic_in);
bench_ease!(bench_cubic_in_f64, f64, cubic_in);

bench_ease!(bench_cubic_out_f32, f32, cubic_out);
bench_ease!(bench_cubic_out_f64, f64, cubic_out);

bench_ease!(bench_cubic_in_out_f32, f32, cubic_in_out);
bench_ease!(bench_cubic_in_out_f64, f64, cubic_in_out);

bench_ease!(bench_quartic_in_f32, f32, quartic_in);
bench_ease!(bench_quartic_in_f64, f64, quartic_in);

bench_ease!(bench_quartic_out_f32, f32, quartic_out);
bench_ease!(bench_quartic_out_f64, f64, quartic_out);

bench_ease!(bench_quartic_in_out_f32, f32, quartic_in_out);
bench_ease!(bench_quartic_in_out_f64, f64, quartic_in_out);

bench_ease!(bench_quintic_in_f32, f32, quintic_in);
bench_ease!(bench_quintic_in_f64, f64, quintic_in);

bench_ease!(bench_quintic_out_f32, f32, quintic_out);
bench_ease!(bench_quintic_out_f64, f64, quintic_out);

bench_ease!(bench_quintic_in_out_f32, f32, quintic_in_out);
bench_ease!(bench_quintic_in_out_f64, f64, quintic_in_out);

bench_ease!(bench_sine_in_f32, f32, sine_in);
bench_ease!(bench_sine_in_f64, f64, sine_in);

bench_ease!(bench_sine_out_f32, f32, sine_out);
bench_ease!(bench_sine_out_f64, f64, sine_out);

bench_ease!(bench_sine_in_out_f32, f32, sine_in_out);
bench_ease!(bench_sine_in_out_f64, f64, sine_in_out);

bench_ease!(bench_circular_in_f32, f32, circular_in);
bench_ease!(bench_circular_in_f64, f64, circular_in);

bench_ease!(bench_circular_out_f32, f32, circular_out);
bench_ease!(bench_circular_out_f64, f64, circular_out);

bench_ease!(bench_circular_in_out_f32, f32, circular_in_out);
bench_ease!(bench_circular_in_out_f64, f64, circular_in_out);

bench_ease!(bench_exponential_in_f32, f32, exponential_in);
bench_ease!(bench_exponential_in_f64, f64, exponential_in);

bench_ease!(bench_exponential_out_f32, f32, exponential_out);
bench_ease!(bench_exponential_out_f64, f64, exponential_out);

bench_ease!(bench_exponential_in_out_f32, f32, exponential_in_out);
bench_ease!(bench_exponential_in_out_f64, f64, exponential_in_out);

bench_ease!(bench_elastic_in_f32, f32, elastic_in);
bench_ease!(bench_elastic_in_f64, f64, elastic_in);

bench_ease!(bench_elastic_out_f32, f32, elastic_out);
bench_ease!(bench_elastic_out_f64, f64, elastic_out);

bench_ease!(bench_elastic_in_out_f32, f32, elastic_in_out);
bench_ease!(bench_elastic_in_out_f64, f64, elastic_in_out);

bench_ease!(bench_back_in_f32, f32, back_in);
bench_ease!(bench_back_in_f64, f64, back_in);

bench_ease!(bench_back_out_f32, f32, back_out);
bench_ease!(bench_back_out_f64, f64, back_out);

bench_ease!(bench_back_in_out_f32, f32, back_in_out);
bench_ease!(bench_back_in_out_f64, f64, back_in_out);

bench_ease!(bench_bounce_in_f32, f32, bounce_in);
bench_ease!(bench_bounce_in_f64, f64, bounce_in);

bench_ease!(bench_bounce_out_f32, f32, bounce_out);
bench_ease!(bench_bounce_out_f64, f64, bounce_out);

bench_ease!(bench_bounce_in_out_f32, f32, bounce_in_out);
bench_ease!(bench_bounce_in_out_f64, f64, bounce_in_out);

