use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::hint::black_box;
use std::ops::Mul;
use std::time::Instant;

fn main() {
    const ITERACIONS: u64 = 100_000;
    const Z: u32 = 25;

    println!("it nº,temps bogo (log_2; ms),temps mult(log_2; ms),x,y");
    for n in 0..ITERACIONS {
        let x = rand_num_amb_bits(Z);
        let y = rand_num_amb_bits(Z);
        eprintln!("{n}/{ITERACIONS}");
        let t_bogo = time(x, y, bogomult as fn(u64, u64) -> u64).log2();
        let t_mult = time(x, y, u64::mul as fn(u64, u64) -> u64).log2();
        println!("{},{:.5},{:.5},{},{}", n, t_bogo, t_mult, x, y);
    }
}

/// Multiplica dos naturals amb complexitat extraordinaria
fn bogomult(x: u64, y: u64) -> u64 {
    if x == 0 {
        return 0;
    }
    if x == 1 {
        return y;
    }

    let mut n = 0;
    while n < bogomult(x - 1, y) + y {
        n += 1
    }
    n
}

/// Retorna els nanosegons que es tarda en executar `f(x, y)` cops
fn time(x: u64, y: u64, f: impl Fn(u64, u64) -> u64) -> f64 {
    let start = Instant::now();
    let _ = black_box(f(x, y));
    let end = Instant::now();

    (end - start).as_nanos() as f64
}

/// Retorna número aleatori que ocupa el número demanat de bits
fn rand_num_amb_bits(bits: u32) -> u64 {
    let mut small_rng = SmallRng::from_entropy();
    small_rng.gen_range(2u64.pow(bits - 1)..2u64.pow(bits))
}

/*
_____ _____ ____ _____ ____
|_   _| ____/ ___|_   _/ ___|
  | | |  _| \___ \ | | \___ \
  | | | |___ ___) || |  ___) |
  |_| |_____|____/ |_| |____/
*/

#[test]
fn mult_zero() {
    for i in 0..10 {
        assert_eq!(0, bogomult(i, 0));
    }
    for i in 0..10 {
        assert_eq!(0, bogomult(0, i));
    }
}

#[test]
fn mult_one() {
    for i in 0..10 {
        assert_eq!(i, bogomult(i, 1));
    }
    for i in 0..10 {
        assert_eq!(i, bogomult(1, i));
    }
}

#[test]
fn small_numbers() {
    let xs = [5, 4, 7, 8, 3, 2, 8, 42];
    let ys = [6, 34, 8, 4, 1, 6, 8, 9, 6, 3, 5, 7];
    for x in xs {
        for y in ys {
            assert_eq!(x * y, bogomult(x, y));
        }
    }
}

// 4 bits -> 1000 - 1111
#[test]
fn random_number() {
    for bits in 0..20 {
        for _ in 0..1000 {
            let g = rand_num_amb_bits(bits);
            assert!(2u64.pow(bits) <= g && g <= 2u64.pow(bits + 1) - 1)
        }
    }
}

#[test]
fn equivalence() {
    const TRIALS: usize = 100000;
    let mut small_rng = SmallRng::from_entropy();

    for _ in 0..TRIALS {
        let x = small_rng.gen();
        let y = small_rng.gen();
        assert_eq!(x * y, bogomult(x, y));
    }
}
