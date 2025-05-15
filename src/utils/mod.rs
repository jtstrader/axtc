use std::io;

pub mod constants;

/// Wait for user confirmation, looping and re-prompting in the case of invalid input.
pub fn wait_for_user_confirmation(prompt: &str) -> bool {
    let mut stdin = std::io::stdin().lock();
    loop {
        println!("{}", prompt);
        match read_user_confirm(&mut stdin) {
            Ok(true) => return true,
            Ok(false) => return false,
            _ => {
                eprintln!("axtc: invalid input");
                continue;
            }
        };
    }
}

fn read_user_confirm<R: io::BufRead>(reader: &mut R) -> io::Result<bool> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    match input.to_lowercase().trim() {
        "y" => Ok(true),
        "n" => Ok(false),
        _ => Err(io::ErrorKind::InvalidInput)?,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_user_confirm_yes() {
        let mut response = &b"y"[..];
        let res = read_user_confirm(&mut response);
        assert!(matches!(res, Ok(true)));
    }

    #[test]
    fn read_user_confirm_yes_ignore_capitalization() {
        let mut response = &b"Y"[..];
        let res = read_user_confirm(&mut response);
        assert!(matches!(res, Ok(true)));
    }

    #[test]
    fn read_user_confirm_no() {
        let mut response = &b"n"[..];
        let res = read_user_confirm(&mut response);
        assert!(matches!(res, Ok(false)));
    }

    #[test]
    fn read_user_confirm_no_ignore_capitalization() {
        let mut response = &b"N"[..];
        let res = read_user_confirm(&mut response);
        assert!(matches!(res, Ok(false)));
    }

    #[test]
    fn read_user_confirm_fail() {
        let mut response = &b"abcdefg"[..];
        let res = read_user_confirm(&mut response);
        assert!(res.is_err());
    }
}
