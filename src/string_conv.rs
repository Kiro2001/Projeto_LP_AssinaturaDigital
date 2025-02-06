pub fn string_to_num(text : String) -> Vec<u8> {
    let data = text;
    let mut lcharn: Vec<u8> = Vec::new();

    for i in data.chars() {
        let asc: char = i;
        lcharn.push(asc as u8);
    }
    let ind=lcharn.len();
    lcharn.remove(ind - 1);
    lcharn.remove(ind - 2);
    return lcharn;
}


pub fn codificar_b64(text : Vec <u8>) -> String {
    let conjcharstring : String = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    let conjchar: Vec<char> = conjcharstring.chars().collect();
    let mut strb64: String = String::new();
    let ogtext=text;
    const LOW_6_BITS_MASK: u8 = 0b0011_1111;
    
    for i in (0.. ogtext.len()).step_by(3){
        let mut char: u32 = 0;
        let mut cont: u32= 0;

        for j in i .. std::cmp::min(i + 3,ogtext.len()){
            let valat: u32= u32::from(ogtext[j]);
            
            char = char << 8;
            char = char | valat;
            cont = cont + 1;
        }
        let numbits = cont * 8;
        let padding = numbits % 3;
    

        if padding != 0{
            char = char << (padding * 8);
        }
        char = char << 8;
        
        char = char.reverse_bits();
        
        for _k in 0..(4 - padding){
            let mut charf= char as u8;
            charf = charf & LOW_6_BITS_MASK;
            charf = charf.reverse_bits();
            charf = charf >> 2;
            strb64.push(conjchar[charf as usize]);
            char = char >> 6;
            
        }
        
        for _k in 0..padding{
            strb64.push('=');
        }
    }

    return strb64;
}

pub fn decodificar_b64(text:String) -> Vec <u8> {
    let conjcharstring : String = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    let conjchar: Vec<char> = conjcharstring.chars().collect();
    let textocodif:Vec<char> = text.chars().collect();
    let mut textoclaro: Vec<u8> = Vec::new();
    let mut padding: u32 = 0;
    let ind: usize = textocodif.len();
    
    for i in 1..ind{
        if textocodif[ind - i] != '=' {
            break;
        }
        padding=padding + 1;
    }


    for i in (0..ind - (padding as usize)).step_by(4){
        let mut chars: u32 = 0;
        let mut cont: u32=0;
        for j in i .. std::cmp::min(i + 4,ind - (padding as usize)){
            let valat: u32= conjchar.iter().position(|&c| c == textocodif[j]).unwrap() as u32;
            chars = chars << 6;
            chars = chars | valat;
            cont = cont + 1;
        }
        chars = chars << 8;

        if cont != 4{
            chars = chars << (padding * 6);
            let arrchars = chars.to_be_bytes();
            for k in 0..(3 - padding){
                textoclaro.push(arrchars[k as usize]);
            }
        }
        else{
            let arrchars = chars.to_be_bytes();
            for k in 0..(3){
                textoclaro.push(arrchars[k as usize]);
            } 
        }
        


            
    }
    
    return textoclaro;

}
