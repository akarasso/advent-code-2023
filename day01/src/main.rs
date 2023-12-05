use std::{fs::File, io::{BufReader, BufRead}};

fn find_first_digit(str: String) -> char {
    for c in str.chars() {
        if c.is_numeric() {
            return c
        }
    }
    panic!("Input not correct: {}", str)
}

fn find_last_digit(str: String) -> char {
    for c in str.chars().rev() {
        if c.is_numeric() {
            return c
        }
    }
    panic!("Input not correct: {}", str)
}

fn main() {
    let mut total = 0;
    let data_result = File::open("input");
    // Reading a file returns a Result enum
    // Result can be a file or an error
    let data_file = match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };
    let reader = BufReader::new(data_file);
    for line_result in reader.lines() {
        let line_read = match line_result {
            Ok(line) => line,
            Err(error) => panic!("Problem opening the data file: {:?}", error),
        };
        let first_digit = find_first_digit(line_read.clone());
        let last_digit = find_last_digit(line_read.clone());
        let mut stringify_number = String::new();
        stringify_number.push(first_digit);
        stringify_number.push(last_digit);
        let number = stringify_number.parse::<i32>().unwrap();
        total += number;
    }
    println!("{}", total);
}

#[cfg(test)]
mod findfirstdigit {
    use crate::find_first_digit;

    #[test]
    fn get_number() {
        assert_eq!(find_first_digit(String::from("zfzeifzifn2")), '2');
    }
}

#[cfg(test)]
mod findlastdigit {
    use crate::find_last_digit;

    #[test]
    fn get_number() {
        assert_eq!(find_last_digit(String::from("zfzeifzifn2")), '2');
    }
}
