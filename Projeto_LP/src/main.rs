use num_bigint::BigUint;
use num_bigint::{ToBigInt, RandBigInt};
fn geraprimo(){
    let mut rng = rand::thread_rng();
    let num1: Vec<u32> = vec![2];
    let num2: Vec<u32> = vec![2];
    let mut max = BigUint::new(num1);
    let mut min = BigUint::new(num2);
    max = max.pow(1024);
    min = min.pow(1023);
    let ale = rng.gen_biguint_range(&min, &max);

    println!("{}", ale);

}
fn main() {
    geraprimo();
}
