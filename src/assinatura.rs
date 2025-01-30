use super::{criptografar_rsa, descriptografar_rsa};
use sha3::{Digest, Sha3_256};
use base64::{DecodeError,decode,encode};
use num_bigint::{BigUint, ToBigUint};
use num_traits::FromPrimitive;


pub fn assinar(plaintext: &str, chave_publica: (BigUint, BigUint)) -> Option<String> {
    let mut hasher = Sha3_256::new();
    hasher.update(plaintext.as_bytes());
    let hashed = hasher.finalize();
    
    let mensagem = BigUint::from_bytes_be(&hashed);
    let assinatura = criptografar_rsa(mensagem, chave_publica);
    Some(codificar_base64(&assinatura))
}

pub fn verificar_assinatura(assinatura: &str, plaintext: &str, chave_privada: (BigUint, BigUint)) -> bool{
    let mut hasher = Sha3_256::new();
    hasher.update(plaintext.as_bytes());
    let hashed = hasher.finalize();
    let assinatura_bytes = decodificar_base64(assinatura);

    match &assinatura_bytes {
        Ok(bytes) => {
            let assinatura_em_inteiro = BigUint::from_bytes_be(bytes);
            if(descriptografar_rsa(assinatura_em_inteiro, chave_privada) == BigUint::from_bytes_be(&hashed)){
                return true;
            }
            return false;
        },
        Err(e) => {
            println!("Erro ao verificar assinatura");
            return false;
        },
    }
}

pub fn codificar_base64(assinatura: &BigUint) -> String {
    let bytes = assinatura.to_bytes_be();
    base64::encode(bytes)
}

pub fn decodificar_base64(assinatura: &str) -> Result<Vec<u8>, DecodeError>{
    return decode(assinatura);
}