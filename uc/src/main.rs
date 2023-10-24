#[derive(Debug, PartialEq)]
enum Token {
    Number(usize),
    Add(Box<Token>, Box<Token>),
    // Multiple(Box<Token>, Box<Token>)
}

impl Token {
    fn new(self) -> Token {
        use Token::*;

        match self {
            Number(value) => Number(value),
            Add(left, right) => match *left {
                Number(left_value) => match *right {
                    Number(right_value) => {
                        Add(Box::new(Token::Number(left_value)), Box::new(Token::Number(right_value)))
                    }
                    _ => panic!("panic")
                }
                _ => panic!("panic")
            },
            /*
            Multiple(left, right) => match *left {

            }
            */
        }
    }

    fn to_s(self) -> String {
        use Token::*;

        match self {
            Number(value) => value.to_string(),
            Add(left, right) => match *left {
                Number(left_value) => match *right {
                    Number(right_value) => {
                        format!("{} + {}", left_value, right_value)
                    }
                    _ => panic!("panic")
                }
                _ => panic!("panic")
            }
        }
    }
}

fn main() {
    let num = Token::new(Token::Number(99));

    let add = Token::new(Token::Add(Box::new(Token::Number(10)), Box::new(Token::Number(20))));

    println!("{:?}", num.to_s());
    println!("{:?}", add.to_s());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance() {
        assert_eq!(Token::Number(99), Token::new(Token::Number(99)));
        assert_eq!(Token::Add(Box::new(Token::Number(10)), Box::new(Token::Number(20))),
        Token::new(Token::Add(Box::new(Token::Number(10)), Box::new(Token::Number(20)))));
    }
}
