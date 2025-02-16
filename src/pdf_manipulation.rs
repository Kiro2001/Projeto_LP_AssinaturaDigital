use lopdf::Document;
use lopdf::dictionary;
use lopdf::{Object, Stream};
use lopdf::content::{Content, Operation};

pub fn get_pdf_content(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut doc = Document::load(file_path)?; // Propaga o erro em vez de usar expect()

    // Remover metadados para que não afetem o hash
    doc.trailer.remove(b"Info");
    doc.trailer.remove(b"Metadata");
    doc.trailer.remove(b"Signature"); // Se houver assinatura

    // Converter para bytes sem os metadados
    let mut raw_pdf = Vec::new();
    doc.save_to(&mut raw_pdf)?; // Propaga o erro

    Ok(raw_pdf)
}

pub fn extract_signature_from_pdf(pdf_path: &str) -> String{
    let doc = Document::load(pdf_path).expect("Falha ao carregar o PDF");
    let u8_assinatura = "Signature".as_bytes();

    // Pegamos a assinatura armazenada nos metadados
    if let Ok(signature_base64) = doc.trailer.get(u8_assinatura) {
        let signed_str = signature_base64.as_name_str().unwrap().to_string();
        return signed_str;
    } else {
        panic!("Assinatura não encontrada no PDF!");
    }
}

pub fn attach_signature_to_pdf(pdf_path: &str, signature_base64: &str) {
    let mut doc = lopdf::Document::load(pdf_path).expect("Falha ao carregar o PDF");

    // Adicione a assinatura ao metadado do PDF
    doc.trailer.set(b"Signature", signature_base64);

    // Salve o PDF modificado
    doc.save("documento_assinado.pdf").expect("Erro ao salvar PDF assinado");
}


pub fn modifica_pdf(pdf_path: &str){
    let mut doc = lopdf::Document::load(pdf_path).expect("Falha ao carregar o PDF");
    
    let pages_id = doc.new_object_id();

    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Courier",
    });


    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! {
            "F1" => font_id,
        },
    });

    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 48.into()]),
            Operation::new("Td", vec![100.into(), 600.into()]),
            Operation::new("Tj", vec![Object::string_literal("Hello World!")]),
            Operation::new("ET", vec![]),
        ],
    };

    let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));


    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    });


    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => 1,
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    };

    doc.objects.insert(pages_id, Object::Dictionary(pages));

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });

    doc.trailer.set("Root", catalog_id);
    doc.compress();


    doc.save(pdf_path).unwrap();
}