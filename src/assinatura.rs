use super::{criptografar_rsa, descriptografar_rsa};
use super::aux_fn::codificar_base64;
use num_bigint::BigUint;

pub fn assinar(hashed_content: &Vec<u8>, chave_publica: (BigUint, BigUint)) -> String{
    let mensagem = BigUint::from_bytes_be(&hashed_content);
    let assinatura = criptografar_rsa(mensagem, chave_publica);
    codificar_base64(assinatura.to_bytes_be())
}

pub fn verificar_assinatura(assinatura_bytes: &Vec<u8>, hashed_content: &[u8], chave_privada: (BigUint, BigUint)) -> bool{
    let assinatura_inteiro = BigUint::from_bytes_be(assinatura_bytes);

    return descriptografar_rsa(assinatura_inteiro, chave_privada) == BigUint::from_bytes_be(&hashed_content);
}