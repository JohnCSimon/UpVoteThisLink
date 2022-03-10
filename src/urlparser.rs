use url::Url;

pub fn parse_url_local(url_string: &str) -> Result<Url, String> {
    match Url::parse(url_string) {
        Ok(url) => Ok(url),
        Err(e) => Err(format!("{}", &e)),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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
}
