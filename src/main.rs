use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::ops::Mul;
use std::time::Instant;

fn main() {
    const ITERS_PER_BITS: u128 = 200;
    const MAX_BITS: u32 = 32;

    println!("it nº,nº bits,algoritme,temps(us),{:010},{:010}", 'x', 'y');
    for n in 0..ITERS_PER_BITS {
        for (algo, algo_name) in [(bogomult as fn(u128, u128) -> u128, "bogomult"), (u128::mul as fn(u128, u128) -> u128, "*")] {
            for bits in 0..MAX_BITS {
                eprintln!("Calculant {bits} bits (iter {n})");
                let x = rand_with_bits(bits);
                let y = rand_with_bits(bits);
                let t = time(x, y, algo);
                println!("{:05},{:04},{:>8},{:07},{:010},{:010}", n, bits, algo_name, t, x, y);
            }
        }
    }

}

/// Multiplica dos u128 amb complexitat extraordinaria
fn bogomult(x: u128, y: u128) -> u128 {
    if x == 0 { return 0 }
    if x == 1 { return y }

    let mut n = 0;
    while n < bogomult(x - 1, y) + y {
        n += 1
    }
    n
}

/// Retorna els millisegons que es tarda en executar `f(x, y)`
fn time(x: u128, y: u128, f: impl Fn(u128, u128) -> u128) -> u128 {
    let start = Instant::now();
    let _ = f(x, y);
    let end = Instant::now();

    (end-start).as_micros()
}

/// Retorna número aleatori que ocupa el número demanat de bits
fn rand_with_bits(bits: u32) -> u128 {
    let mut small_rng = SmallRng::from_entropy();
    small_rng.gen_range(2u128.pow(bits)..2u128.pow(bits + 1))
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
            let g = rand_with_bits(bits);
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
        assert_eq!(x*y, bogomult(x, y));
    }
}
