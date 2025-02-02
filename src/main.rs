mod assinatura;

use sha3::{Digest, Sha3_256};
use base64;
use num_bigint::BigUint;
use num_bigint::{ToBigInt, RandBigInt};
use num_traits::{Num, One};
use num_traits::Zero;
use std::time::{SystemTime, UNIX_EPOCH};

fn seed_BigUint() -> BigUint {
  let tempo = SystemTime::now().duration_since(UNIX_EPOCH).expect("Houston, we have a problem.");
  return BigUint::from( tempo.as_secs() );
}

fn random_BigUint() -> BigUint{
  let modulus = BigUint::from(2_u32).pow(32);
  let multiplier = BigUint::from(1664525_u32);
  let increment = BigUint::from(1013904223_u32);
  let seed = seed_BigUint();
  let result = ( seed * multiplier + increment) % modulus;
  return result; 
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

/*def power(x, y, p) :
    res = 1     # Initialize result

    # Update x if it is more
    # than or equal to p
    x = x % p 
    
    if (x == 0) :
        return 0

    while (y > 0) :
        
        # If y is odd, multiply
        # x with result
        if ((y & 1) == 1) :
            res = (res * x) % p

        # y must be even now
        y = y >> 1      # y = y/2
        x = (x * x) % p
        
    return res*/

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
  
  /*
  let mut plaintext = "ola mundo";
  let big_int_chave_publica = BigUint::from_str_radix("3024309595713703550698328938426547750510840110938483057719575811129937029926494570183450198868757660108580326658974290247228261806106642702998274230160058231365816090767792512935089465870096780650873974295129125090296970508135929388876051172056916117469028829714113294710923714445659937549580085599831961458943367588175851446408177265065247829355804966847284109830128910203968234898743274495855231593970882374387709288378376479706249612458571409088141421694216408530267633459002673666677586408971582985911524380847298442321644376010893067789664872159028285694766156421350519060396343219088940759227101136668162033287", 10);

  let exp_chave_publica = BigUint::from_str_radix("65537", 10);

  let chave_privada = (BigUint::from_str_radix("3024309595713703550698328938426547750510840110938483057719575811129937029926494570183450198868757660108580326658974290247228261806106642702998274230160058231365816090767792512935089465870096780650873974295129125090296970508135929388876051172056916117469028829714113294710923714445659937549580085599831961458943367588175851446408177265065247829355804966847284109830128910203968234898743274495855231593970882374387709288378376479706249612458571409088141421694216408530267633459002673666677586408971582985911524380847298442321644376010893067789664872159028285694766156421350519060396343219088940759227101136668162033287",10).unwrap(), BigUint::from_str_radix("1455509848763384404116392160869611709398697040436863056342724577854484396003960590783163750591566372545046799260733754805496537919441073248627013251956580201649590388923919978798017255031642012275649114595460087194608492100601421317035255352210921941210165834841583341142823822837947420392317400550899490308874564261227800011250452012411108859014569640319249925934531446367109750219438961898058824858731485591154297950340155890382429169161906358931179657197056718723619308946020960957632154677288685321080485292513731404426600907600807401755688407328918424857290096373480836436095680163062662523446498348475609753921", 10).unwrap());

  let res_assinatura = assinatura::assinar(plaintext, (big_int_chave_publica.unwrap(), exp_chave_publica.unwrap())).unwrap();
let res_assinatura_str = res_assinatura.as_str(); // Agora res_assinatura vive tempo suficiente
  plaintext = "ola pessoal";

  println!("{}", res_assinatura);

  let eh_a_mesma_assinatura = assinatura::verificar_assinatura(res_assinatura_str, plaintext, chave_privada);

  println!("{}", eh_a_mesma_assinatura);
  */


  /*
  // exemplo Seed
  let seed_do_tempo = seed_BigUint();
  println!("seed do tempo: {}", seed_do_tempo);
  // exemplo randon big u int
  let randombiguint = random_BigUint();
  println!("random big u int: {}", randombiguint);
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
  */
}
