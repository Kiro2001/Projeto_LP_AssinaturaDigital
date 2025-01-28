use num_bigint::BigUint;
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