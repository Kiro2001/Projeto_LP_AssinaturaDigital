mod assinatura;
mod pdf_manipulation;
mod hash;
mod aux_fn;
mod primos;
mod rsa;

use std::io;

use num_bigint::BigUint;
use num_traits::Num;

fn main() {
  let mut path_to_pdf = String::new();
  let mut operacao = String::new();
  let pdf_content: Vec<u8>;

  loop{
    println!("Insira o caminho para seu arquivo: ");
    io::stdin().read_line(&mut path_to_pdf).expect("Erro ao ler caminho");
    path_to_pdf = path_to_pdf.trim().to_string();
    match pdf_manipulation::get_pdf_content(&path_to_pdf){
      Ok(content) => {
        pdf_content = content;
        break;
      },
      Err(e) => {
        println!("Erro ao ler PDF, tente novamente!");
        path_to_pdf = String::new();
      }
    }


  }

  println!("Deseja assinar documento, verificar assinatura ou modificar? [A|V|M]: ");
  io::stdin().read_line(&mut operacao).expect("Erro ao ler opção");

   if operacao.trim() == "A" {
    let seed_do_tempo = primos::seed_BigUint();
    let mr_prime = primos::gera_primo();
    let mr_prime2 = primos::gera_primo();

    let chave_privada;
    let chave_publica;

    (chave_publica,chave_privada) = rsa::chaves_rsa(mr_prime, mr_prime2);

    let hashed_pdf = hash::hash_content(&pdf_content);
    let res_assinatura = assinatura::assinar(&hashed_pdf, chave_privada);
    pdf_manipulation::attach_signature_to_pdf(&path_to_pdf, &res_assinatura);

    println!("Documento assinado");
    println!("Chave publica para verificação: ({}, {})", chave_publica.0, chave_publica.1);
  }else if operacao.trim() == "V"{
    let mut modulo_chave_publica = String::new();
    let mut exp = String::new(); 
    println!("Insira o modulo da chave pública: ");
    io::stdin().read_line(&mut modulo_chave_publica).expect("Erro ao ler modulo");
    modulo_chave_publica = modulo_chave_publica.trim().to_string();

    println!("Insira o expoente da chave pública: ");
    io::stdin().read_line(&mut exp).expect("Erro ao ler expoente");
    exp = exp.trim().to_string();
  
    let chave_publica = (BigUint::from_str_radix(&modulo_chave_publica, 10).expect("Erro ao obter modulo"), BigUint::from_str_radix(&exp, 10).expect("Erro ao obter expoente"));

    let signature = pdf_manipulation::extract_signature_from_pdf(&path_to_pdf);
    let hashed_signed_pdf = hash::hash_content(&pdf_content);
    let signature_em_bytes = aux_fn::decodificar_base64(signature);
  
    let eh_a_mesma_assinatura = assinatura::verificar_assinatura(&signature_em_bytes, &hashed_signed_pdf, chave_publica);

    if eh_a_mesma_assinatura{
      println!("Documento valido");
    }else{
      println!("Documento invalido");
    }
  }else{
    pdf_manipulation::modifica_pdf(&path_to_pdf);
  }
  
}
