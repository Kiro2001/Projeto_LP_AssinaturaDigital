use num_bigint::BigUint;
use num_bigint::RandBigInt;
use num_traits:: One;
use num_traits::Zero;
use std::time::{SystemTime, UNIX_EPOCH};

static mut add_in_seed : u128 = 0;

pub fn seed_BigUint() -> BigUint {
  let tempo = SystemTime::now().duration_since(UNIX_EPOCH).expect("Houston, we have a problem.");
  unsafe { add_in_seed += 1 };
  return BigUint::from( tempo.as_millis() + unsafe{add_in_seed}); 
}

fn random_BigUint() -> BigUint{
  let modulus = BigUint::from(2_u32).pow(32);
  let multiplier = BigUint::from(1664525_u32);
  let increment = BigUint::from(1013904223_u32);
  let seed = seed_BigUint();
  let result = ( seed * multiplier + increment) % modulus;
  return result; 
}

fn random_BigUint_1024() -> BigUint {
  let mut res = BigUint::zero();
  for i in 0..32{
    let aux = random_BigUint() << (32*i);
    res = res | aux;
  }
  res
}

fn random_BigUint_within_range(n: BigUint, m: BigUint) -> BigUint {
  if n>=m{
      panic!("n has to be lesser than m!");
  }
  let res = &n + random_BigUint() % (m - &n + BigUint::one());
  return res;
}

fn random_BigUint_1024_within_range(n : BigUint, m : BigUint) -> BigUint {
  if n>=m{
    panic!("n has to be lesser than m!");
  }
  let res = &n + random_BigUint_1024() % (m - &n + BigUint::one());
  return res;
}

fn fast_modular_exponentiation(mut base: BigUint, mut exponent : BigUint, modulus : BigUint) -> BigUint{
  let mut res = BigUint::one();
  base = base % &modulus;
  while !exponent.is_zero() {
    if ( &exponent & BigUint::one() ) == BigUint::one(){
      res = (res * &base) % &modulus;
    }
    exponent = exponent >> 1;
    base = (&base * &base) % &modulus;
  }
  return res;
}

fn miller_rabin_test(n: &BigUint, d: &BigUint) -> bool {
  let random_number = random_BigUint_within_range(BigUint::from(2_u32), (n - BigUint::from(2_u32)));
  let mut x = fast_modular_exponentiation(random_number, d.clone(), n.clone());
  if x == BigUint::one() || x == n - BigUint::one() {
      return true;
  }
  let mut d = d.clone();
  while d != n - BigUint::one() {
      x = (&x * &x) % n;
      d = d * 2_u32;
      if x == BigUint::one() {
          return false;
      }
      if x == n - BigUint::one() {
          return true;
      }
  }
  false
}

fn iterate_miller_rabin(n : BigUint, k: usize) -> bool {

  if n <= BigUint::one(){
    return false;
  }
  if n == BigUint::from(2_u32) || n == BigUint::from(3_u32){
    return true;
  }

  let mut d = n.clone() - BigUint::one();
  while(d.clone() % BigUint::from(2_u32) == BigUint::zero()){
    d = d / BigUint::from(2_u32);
  }

  for _i in 0..k{
    if !miller_rabin_test(&n,&d){
      return false;
    }
  }

  return true;

}

pub fn gera_primo() -> BigUint{
  let mut n = BigUint::one();
  loop {
    n = random_BigUint_1024();
    if iterate_miller_rabin(n.clone(), 3){
      break;
    }
  }
  return n;
}

fn geraprimo() {
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

// Funcao para calcular o inverso modular de `a` em relação a `m`
fn mod_inv(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    use num_bigint::BigInt;
    use num_integer::Integer;
    use num_traits::{One, Zero};
  
    let mut t = BigInt::zero();
    let mut new_t = BigInt::one();
    let mut r = BigInt::from(m.clone());
    let mut new_r = BigInt::from(a.clone());
  
    // Algoritmo de Euclides Estendido
    while new_r != BigInt::zero() {
      let quotient = &r / &new_r;
  
      let temp_t = t.clone();
      t = new_t.clone();
      new_t = temp_t - &quotient * &new_t;
  
      let temp_r = r.clone();
      r = new_r.clone();
      new_r = temp_r - &quotient * &new_r;
    }
  
    if r != BigInt::one() {
      None
    } else {
      let result = (t % &BigInt::from(m.clone()) + &BigInt::from(m.clone())) % &BigInt::from(m.clone());
      Some(result.to_biguint().unwrap())
    }
  }