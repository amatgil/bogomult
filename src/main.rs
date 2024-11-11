use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::hint::black_box;
use std::ops::Mul;
use std::time::Instant;

fn main() {
    const BITS: u32 = 20;
    const ITERACIONS: u128 = 100000000;

    println!("nº bits,it nº,temps bogo (ns),temps mult(ns),x,y");
    for n in 0..ITERACIONS {
        eprintln!("Calculant {BITS} bits (iter {n})");
        let x = rand_num_amb_bits(BITS);
        let y = rand_num_amb_bits(BITS);
        let t_bogo = time(x, y, bogomult as fn(u128, u128) -> u128);
        let t_mult = time(x, y, u128::mul as fn(u128, u128) -> u128);
        println!("{},{},{},{},{},{}", BITS, n, t_bogo, t_mult, x, y);
    }
}

/// Multiplica dos u128 amb complexitat extraordinaria
fn bogomult(x: u128, y: u128) -> u128 {
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

/// Retorna els millisegons que es tarda en executar `f(x, y)` TRIALS cops
fn time(x: u128, y: u128, f: impl Fn(u128, u128) -> u128) -> u128 {
    let start = Instant::now();
    let _ = black_box(f(x, y));
    let end = Instant::now();

    (end - start).as_nanos()
}

/// Retorna número aleatori que ocupa el número demanat de bits
fn rand_num_amb_bits(bits: u32) -> u128 {
    let mut small_rng = SmallRng::from_entropy();
    small_rng.gen_range(2u128.pow(bits - 1)..2u128.pow(bits))
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
            assert!(2u128.pow(bits) <= g && g <= 2u128.pow(bits + 1) - 1)
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
