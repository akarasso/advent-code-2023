use std::{io::Error};


fn match_number(str: String) -> Result<char, Error> {
    let arr_of_string_digit: Vec<String> = vec![
        String::from("zero"),
        String::from("un"),
        String::from("deux"),
        String::from("trois"),
        String::from("quattre"),
        String::from("cinq"),
        String::from("six"),
        String::from("sept"),
        String::from("huit"),
        String::from("neuf"),
    ];
    for (index_of_string, c) in str.chars().enumerate() {
        if c.is_numeric() {
            return Ok(c);
        }
        for (index, string_digit) in arr_of_string_digit.iter().enumerate() {
            if str[0..index_of_string + 1].contains(string_digit) {
                let c = char::from_u32(index as u32 + '0' as u32).unwrap(); 
                return Ok(c);
            }
        }
    }
    
    return Err(Error::new(std::io::ErrorKind::NotFound, "No digit found"))
}

#[cfg(test)]
mod tests {
    use super::match_number;

    #[test]
    fn get_basic_number() {
        let result = match_number(String::from("0")).unwrap();
        assert_eq!(result, '0');
    }

    #[test]
    fn no_numeric_value() {
        let result = match_number(String::from("abc")).unwrap_err().kind();
        assert_eq!(result, std::io::ErrorKind::NotFound);
    }

    #[test]
    fn get_string_number() {
        let result = match_number(String::from("un")).unwrap();
        assert_eq!(result, '1');
    }
}
