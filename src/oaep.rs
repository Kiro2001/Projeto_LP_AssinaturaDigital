use super::{criptografar_rsa, descriptografar_rsa};
use num_bigint::BigUint;
use rand::Rng;
use sha1::{Digest, Sha1};


pub fn mgf1(seed: &[u8], k: usize) -> Vec<u8> {
    let hash_size = 20; // tamanho de hash padrão para sha1
    let mut result = Vec::new();
    let iterations = (k + hash_size - 1) / hash_size;

    for i in 0..iterations {
        let mut input = seed.to_vec();
        input.extend(&i.to_be_bytes());
        let mut hasher = Sha1::new();
        hasher.update(&input);
        let hash_result = hasher.finalize();
        result.extend(&hash_result);
    }

    result.truncate(k);
    result
}
fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

fn codificar_oaep(mensagem_em_bytes: &[u8], k: usize, rotulo: &[u8]) -> Vec<u8> {
    let hash_len = Sha1::output_size();
    let hash_rotulo = Sha1::digest(rotulo);

    let mut db = Vec::new();
    db.extend(&hash_rotulo);
    db.extend(vec![0u8; k - mensagem_em_bytes.len() - 2 * hash_len - 2]);
    db.push(0x01);
    db.extend(mensagem_em_bytes);

    let mut rng = rand::thread_rng();
    let seed: Vec<u8> = (0..hash_len).map(|_| rng.gen()).collect();
    let db_mask = mgf1(&seed, k - hash_len - 1);
    let masked_db = xor_bytes(&db, &db_mask);

    let seed_mask = mgf1(&masked_db, hash_len);
    let masked_seed = xor_bytes(&seed, &seed_mask);

    let mut result = Vec::new();
    result.push(0x00);
    result.extend(masked_seed);
    result.extend(masked_db);
    result
}

fn decodificar_oaep(texto_cifrado: &[u8], k: usize, rotulo: &[u8]) -> Vec<u8> {
    let hash_len = Sha1::output_size();
    let masked_seed = &texto_cifrado[0..hash_len];
    let masked_db = &texto_cifrado[hash_len..];

    let seed_mask = mgf1(masked_db, hash_len);
    let seed = xor_bytes(masked_seed, &seed_mask);

    let db_mask = mgf1(&seed, k - hash_len - 1);
    let db = xor_bytes(masked_db, &db_mask);
    let mut i = hash_len;
    while i < db.len() {
        if db[i] == 0 {
            i += 1;
        } else if db[i] == 1 {
            break;
        } else {
            panic!("Decodificação OAEP falhou");
        }
    }

    db[i + 1..].to_vec()
}

pub fn cifrar(mensagem_em_bytes: &[u8], chave: (BigUint, BigUint)) -> Vec<u8> {
    let (n, e) = chave;
    let k = ((n.bits() + 7) / 8) as usize;
    let mensagem_codificada = codificar_oaep(mensagem_em_bytes, k, b"");
    let mensagem_cifrada = criptografar_rsa(BigUint::from_bytes_be(&mensagem_codificada), (n, e));

    mensagem_cifrada.to_bytes_be().to_vec()
}

pub fn decifrar(texto_cifrado: &[u8], chave: (BigUint, BigUint)) -> Vec<u8> {
    let (n, d) = chave;
    let k = ((n.bits() + 7) / 8) as usize;
    let mensagem_decodificada = descriptografar_rsa(BigUint::from_bytes_be(texto_cifrado), (n, d));

    decodificar_oaep(&mensagem_decodificada.to_bytes_be(), k, b"")
}
