use num_bigint::BigUint;

// Funcao para calcular o inverso modular de `a` em relação a `m`
fn mod_inv(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    use num_bigint::BigInt;
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

// Funcao para gerar as chaves RSA (publica e privada)
pub fn chaves_rsa(primo1: BigUint, primo2: BigUint) -> ((BigUint, BigUint), (BigUint, BigUint)) {
    let n = &primo1 * &primo2;
    let phi = (&primo1 - BigUint::from(1_u64)) * (&primo2 - BigUint::from(1_u64));
    
    let e = BigUint::from(65537_u64); // Expoente de criptografia (valor padrao para e)
    let d = mod_inv(&e, &phi).expect("Erro ao calcular o inverso modular"); // Expoente de descriptografia
  
    let chave_publica = (n.clone(), e);
    let chave_privada = (n, d);
  
    (chave_publica, chave_privada)
  }
  
  // Funcao para criptografar uma mensagem usando a chave publica
  pub fn criptografar_rsa(mensagem: BigUint, chave_publica: (BigUint, BigUint)) -> BigUint {
    let (n, e) = chave_publica;
    mensagem.modpow(&e, &n) // mensagem^e mod n
  }
  
  // Funcao para descriptografar uma mensagem criptografada usando a chave privada
  pub fn descriptografar_rsa(msg_cifrada: BigUint, chave_privada: (BigUint, BigUint)) -> BigUint {
    let (n, d) = chave_privada;
    msg_cifrada.modpow(&d, &n) // msg_cifrada^d mod n
  }