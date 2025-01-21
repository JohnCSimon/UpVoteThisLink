use std::collections::HashMap;

use url::{Url, form_urlencoded};

pub fn parse_url_local(url_string: &str) -> Result<Url, String> {
    match Url::parse(url_string) {
        Ok(url) => Ok(url),
        Err(e) => Err(format!("{}", &e)),
    }
}

pub fn remove_query_parameters(
    url_string: &str,
    params_to_remove: &Vec<&str>,
) -> Result<String, String> {
    let mut url = Url::parse(url_string).map_err(|e| e.to_string())?;

    // Extract query parameters into a HashMap
    let mut query_pairs: HashMap<_, _> = url.query_pairs().into_owned().collect();

    if query_pairs.is_empty() {
        return Ok(url.into());
    }

    // Remove specified parameters
    for param in params_to_remove {
        query_pairs.remove(*param);
    }

    // Reconstruct the query string
    let new_query: String = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(query_pairs)
        .finish();

    // Set the new query string
    url.set_query(Some(&new_query));

    Ok(url.into())
}

// hashes an integer value to a string
pub fn hash(myint: u32, mystr: &str) -> String {
    let length: u32 = mystr.len() as u32;
    let mut _quotient = myint;
    let mut _remainder;

    let mut _hash = String::new();

    while _quotient != 0 {
        _remainder = _quotient % length;
        _quotient /= length;
        let i = index_into_str(mystr, _remainder.try_into().unwrap());
        _hash.insert(0, i);
    }
    _hash
}

// index into string and return the character at the index
pub fn index_into_str(mystr: &str, index: usize) -> char {
    mystr.chars().nth(index).unwrap() // solid kak O(n) way to do this
    // copilot suggested this line - I say it's worth the $100/yr
    // for this alone
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // unit test the convert integer to hex code
    #[test]
    fn test_integer_to_hex() {
        let x = "0123456789ABCDEF";
        let y: u32 = 35631;
        let result = hash(y, x);
        assert_eq!(result, "8B2F");
    }

    #[test]
    fn test_integer_to_urlfriendly() {
        let x = "0123456789ABCDEFGHIJKLMNOPQURSTUVWXYZabcdefghijklmnopqrstuvwxyz._~()'!*:@,;";
        let y: u32 = 35631;
        let result = hash(y, x);
        assert_eq!(result, "6P6");
    }

    #[test]
    fn test_parse_url() -> Result<(), String> {
        let url = parse_url_local("https://www.rust-lang.org/")?;

        assert_eq!("www.rust-lang.org", url.domain().unwrap().to_string());
        Ok(())
    }

    #[test]
    fn test_parse_url_fail() -> Result<(), String> {
        let badurl = "httpss//www.rust-lang.org/";
        match parse_url_local(badurl) {
            Ok(_) => Err("Should have failed".to_string()),
            Err(x) => {
                println!("Error: {}", &x);
                Ok(())
            }
        }
    }

    #[test]
    fn hash_string() -> Result<(), String> {
        let badurl = "httpss//www.rust-lang.org/";

        match parse_url_local(badurl) {
            Ok(_) => Err("Should have failed".to_string()),
            Err(x) => {
                println!("Error: {}", &x);
                Ok(())
            }
        }
    }

    #[test]
    fn test_remove_query_parameters() {
        let url = "https://www.example.com/?param1=value1&param2=value2&param3=value3";
        let params_to_remove = vec!["param2", "param3"];
        let result = remove_query_parameters(url, &params_to_remove).unwrap();
        assert_eq!(result, "https://www.example.com/?param1=value1");
    }

    // pub struct Guess {
    //     value: i32,
    // }

    // impl Guess {
    //     pub fn new(value: i32) -> Result<Guess, String> {
    //         if !(1..=100).contains(&value) {
    //             Err("Guess value must be between 1 and 100, got {value}.".to_string())
    //         } else {
    //             Ok(Guess { value })
    //         }
    //     }

    //     pub fn valueee(&self) -> i32 {
    //         self.value
    //     }
    // }

    // #[test]
    // fn fart() -> Result<(), String> {
    //     let guess = Guess::new(5)?;
    //     let xx = guess.valueee();
    //     Ok(())
    // }
}
