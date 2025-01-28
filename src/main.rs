use num_bigint::BigUint;
use num_bigint::{ToBigInt, RandBigInt};
use num_traits::One;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn randomBigUint() -> BigUint {
  let tempo = SystemTime::now().duration_since(UNIX_EPOCH).expect("Houston, we have a problem.");
  return BigUint::from( tempo.as_secs() );
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

// Funcao para gerar as chaves RSA (publica e privada)
fn chaves_rsa(primo1: BigUint, primo2: BigUint) -> ((BigUint, BigUint), (BigUint, BigUint)) {
  let n = &primo1 * &primo2;
  let phi = (&primo1 - BigUint::from(1_u64)) * (&primo2 - BigUint::from(1_u64));
  
  let e = BigUint::from(65537_u64); // Expoente de criptografia (valor padrao para e)
  let d = mod_inv(&e, &phi).expect("Erro ao calcular o inverso modular"); // Expoente de descriptografia

  let chave_publica = (n.clone(), e);
  let chave_privada = (n, d);

  (chave_publica, chave_privada)
}

// Funcao para criptografar uma mensagem usando a chave publica
fn criptografar_rsa(mensagem: BigUint, chave_publica: (BigUint, BigUint)) -> BigUint {
  let (n, e) = chave_publica;
  mensagem.modpow(&e, &n) // mensagem^e mod n
}

// Funcao para descriptografar uma mensagem criptografada usando a chave privada
fn descriptografar_rsa(msg_cifrada: BigUint, chave_privada: (BigUint, BigUint)) -> BigUint {
  let (n, d) = chave_privada;
  msg_cifrada.modpow(&d, &n) // msg_cifrada^d mod n
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

fn main() {
  //Seed
  let seed_do_tempo = randomBigUint();
  println!("seed do tempo: {}", seed_do_tempo);
  // Numeros primos grandes como exemplo
  let primo1 = BigUint::from(7919_u64);
  let primo2 = BigUint::from(6841_u64);
  println!("Numero primo 1: {:?}", primo1);
  println!("Numero primo 2: {:?}", primo2);

  let (chave_publica, chave_privada) = chaves_rsa(primo1, primo2);
  println!("Chave Pública: {:?}", chave_publica);
  println!("Chave Privada: {:?}", chave_privada);

  // Mensagem como exemplo
  let mensagem = BigUint::from(42_u64);

  let msg_cifrada = criptografar_rsa(mensagem, chave_publica);
  println!("Mensagem criptografada: {}", msg_cifrada);

  let mensagem_decriptada = descriptografar_rsa(msg_cifrada, chave_privada);
  println!("Mensagem decriptada: {}", mensagem_decriptada);
}
