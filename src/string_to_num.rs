pub fn string_to_num(text : String) -> Vec<u8> {
    let data = text;
    let mut lcharn: Vec<u8> = Vec::new();

    for i in data.chars() {
        let asc: char = i;
        lcharn.push(asc as u8);
    }

    return lcharn;
}