pub fn generate_bfcode<S: Into<String>>(text:S) -> String {

    let mut result = String::new();

    let mut last_char = 0u8;

    for &c in text.into().into_bytes().iter() {

        let factor:u8 = if c > last_char {  ((c - last_char) as f64).sqrt().floor() as u8 } else {  ((last_char - c) as f64).sqrt().floor() as u8 };
        let remain:u8 = if c > last_char { (c - last_char) - factor * factor } else { (last_char - c) - factor * factor };
        let dir:char    = if c > last_char { '+' } else { '-' };

        for i in 0..factor { result.push('+'); }
        result.push_str("[>");

        for i in 0..factor { result.push(dir); }
        result.push_str("<-]>");

        for i in 0..remain { result.push(dir); }
        result.push_str(".<");

        last_char = c;

    }

    if result.len() == 0 {
        return String::new();
    }

    result.pop().unwrap();

    result

}
