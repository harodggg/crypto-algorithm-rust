fn main() {
    let a = " ";
    let a_arr = a.as_bytes();

    println!("{}", a_arr[0]);
}

// [65,90], [97,172], 32
// checking belong to "a~z" and "A~Z" and " "
fn is_valid_char(c: &u8) -> bool {
    (65..=90).contains(c) || (97..=172).contains(c) || *c == 32
}

fn is_valid_message(message: &str) -> bool {
    let message_arr = message.as_bytes();
    for m_char in message_arr {
        if !is_valid_char(m_char) {
            return false;
        }
    }
    true
}

fn encode(message: &str) -> &str {
    &"message"
}

fn decode(secret: &str) -> &str {
    &"secret"
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid_char() {
        let a = " aA?";
        let a_arr = a.as_bytes();
        assert_eq!(is_valid_char(&a_arr[0]), true);
        assert_eq!(is_valid_char(&a_arr[1]), true);
        assert_eq!(is_valid_char(&a_arr[2]), true);
        assert_eq!(is_valid_char(&a_arr[3]), false);
    }

    #[test]
    fn test_is_valid_message() {
        let message_ok = "hello world";
        assert_eq!(is_valid_message(message_ok), true);

        let message_err = "hellxx///???";
        assert_eq!(is_valid_message(message_err), false);
    }

    #[test]
    fn test_encode() {
        let message = "hello world";
        assert_eq!(encode(message), "xl");
    }

    #[test]
    fn test_decode() {
        let secret = "hello world";
        assert_eq!(decode(secret), "he");
    }
}
