use num_bigint::BigUint;
use base64::{DecodeError,decode,encode};

pub fn codificar_base64(assinatura: &BigUint) -> String {
    let bytes = assinatura.to_bytes_be();
    base64::encode(bytes)
}

pub fn decodificar_base64(assinatura: &str) -> Result<Vec<u8>, DecodeError>{
    return decode(assinatura);
}

/*
pub fn fast_mod_exp(base : BigUint, exponent : BigUint, modulus : BigUint) -> BigUint{
	let mut result = BigUint::from(1u32);
	let mut exponent = exponent.clone();
	let mut base = base.clone();
	let modulus = modulus.clone();
	
	while exponent > BigUint::from(0u32) {
		if exponent & BigUint::from(2u32) == BigUint::from(1u32) {
			result = (result*base) % modulus
		}
		exponent = exponent/ BigUint::from(2u32);
		base = base*base;
	}
	return (result % modulus);
}*/