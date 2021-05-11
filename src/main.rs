use argh::FromArgs;
use rand::Rng;

///number sequence generator for a puzzle
#[derive(FromArgs, PartialEq, Debug)]
struct Args {
    ///start/seed number of the test
    #[argh(option, short = 's')]
    seed : i32,
    ///iteration depth
    #[argh(option, short = 'd', default = "16")]
    depth : i32
}

fn parse_digits(mut value : i32, digits : &mut [i32]) {
    let mut digit_place = 0;
    while value > 0 {
        let scaled = value / 10;
        digits[digit_place] = value - scaled * 10;
        value = scaled;
        digit_place += 1;
    }
}

fn from_digits_k(digits : &[i32; 4], k : usize) -> i32 {
    let mut value : i32 = 0;
    let mut place : i32 = 1;
    //println!("from_digits_{}", k);
    for x in 0..(4 / (2 * k)) {
        //println!("x:{}", x);
        for y in (2 * x + k)..(2 * x + 2 * k) {
            //println!("y:{}", y);
            value += place * digits[y];
            place *= 10;
        }
        for y in (2 * x)..(2 * x + k) {
            //println!("y:{}", y);
            value += place * digits[y];
            place *= 10;
        }
    }
    //println!("{}", value);
    value
}

///for puzzle:
/// https://www.quantamagazine.org/the-puzzling-power-of-simple-arithmetic-20210420/
fn main() {
    let args : Args = argh::from_env();

    let seed = if args.seed >= 1000 {
        args.seed
    }
    else {
        rand::thread_rng().gen_range(1000..10000)
    };
    let depth = if args.depth > 0 {
        args.depth
    }
    else {
        4
    };

    let mut digits : [i32; 4] =  [0; 4];
    parse_digits(seed, &mut digits);

    println!("{}", seed);
    for _k in 0..depth {
        let value = (from_digits_k(&digits, 1) - from_digits_k(&digits, 2)).abs();
        println!("{}", value);
        parse_digits(value, &mut digits);
    }
}
