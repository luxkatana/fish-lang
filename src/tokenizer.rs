#[derive(Clone, Debug)]
pub enum TokenType {
    Return,
    IntegerLiteral,
    LetKeyword,
    Identifier,
    Equal,
    OpenParam,
    CloseParam,
    Newline
}
#[derive(Clone, Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub value: Option<String>,
}
pub struct Tokenizer {
    inputdata: String,
    pointer_index: i32,
}
impl Tokenizer {
    pub fn new(inputdata: String) -> Self {
        Self {
            inputdata,
            pointer_index: 0,
        }
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = vec![];
        let mut buffer = String::new();
        while let Some(mut character) = self.consume() {
            if character == '#' {
                while character != '\n' {
                    if let Some(s) = self.consume() {
                        character = s;
                    } else {
                        break;
                    }
                }
                continue;
            }
            if character == '\n' {
                tokens.push(Token {
                    t_type: TokenType::Newline,
                    value: None
                });
                continue;
            }
            else if character.is_ascii_whitespace() {
                continue;
            }

            if character.is_alphabetic() {
                while character.is_alphabetic() {
                    buffer.push(character);
                    if let Some(new_char) = self.consume() {
                        character = new_char;
                    } else {
                        break;
                    }
                }
                if buffer == "return" {
                    tokens.push(Token {
                        t_type: TokenType::Return,
                        value: None,
                    });
                } else if buffer == "let" {
                    tokens.push(Token {
                        t_type: TokenType::LetKeyword,
                        value: None,
                    });
                } else {
                    tokens.push(Token {
                        t_type: TokenType::Identifier,
                        value: Some(buffer.clone()),
                    })
                }
                buffer.clear();
                self.pointer_index -= 1;
            } else if character.is_numeric() {
                while character.is_numeric() {
                    buffer.push(character);
                    if let Some(_) = self.peek(0) {
                        character = self.consume().unwrap();
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    t_type: TokenType::IntegerLiteral,
                    value: Some(buffer.clone()),
                });
                buffer.clear();
                self.pointer_index -= 1; // one to far
            } else if character == '=' {
                tokens.push(Token {
                    t_type: TokenType::Equal,
                    value: None,
                });
            } else if character == '(' {
                tokens.push(Token {
                    t_type: TokenType::OpenParam,
                    value: None,
                });
            } else if character == ')' {
                tokens.push(Token {
                    t_type: TokenType::CloseParam,
                    value: None,
                });
            } else {
                return Err(format!("Weird character: '{character}'"));
            }
        }

        Ok(tokens)
    }
    fn consume(&mut self) -> Option<char> {
        let x = self.peek(0);
        self.pointer_index += 1;
        x
    }
    fn peek(&self, offset: usize) -> Option<char> {
        self.inputdata
            .chars()
            .nth(offset + (self.pointer_index as usize))
    }
}
