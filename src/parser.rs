use crate::tokenizer::{Token, TokenType};
use std::any::Any;

pub enum NodeType {
    ExpressionInt,
    ExpressionIdent,
    Exit,
    Variable,
}
pub trait Node: Any {
    fn as_any(&self) -> &dyn Any;
    fn nodetype(&self) -> NodeType;
}
pub struct NodeExpressionInt {
    pub int_literal: Token,
}

impl Node for NodeExpressionInt {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn nodetype(&self) -> NodeType {
        NodeType::ExpressionInt
    }
}

pub struct NodeExpressionIdentifier {
    pub ident: Token,
}
impl Node for NodeExpressionIdentifier {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn nodetype(&self) -> NodeType {
        NodeType::ExpressionIdent
    }
}
pub struct NodeVariable {
    pub expr: ExpressionTypes, // enum that also contains the expression-data self
    pub identifier: Token,
}
impl Node for NodeVariable {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn nodetype(&self) -> NodeType {
        NodeType::Variable
    }
}
pub enum ExpressionTypes {
    Int(NodeExpressionInt),
    Identifier(NodeExpressionIdentifier),
}
pub struct NodeExit {
    pub expr: ExpressionTypes,
}
impl Node for NodeExit {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn nodetype(&self) -> NodeType {
        NodeType::Exit
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pointer_index: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pointer_index: 0,
        }
    }
    fn parse_expression(&mut self) -> Option<ExpressionTypes> {
        let token = self.consume().unwrap();
        match token.t_type {
            
            TokenType::IntegerLiteral => {
                return Some(ExpressionTypes::Int(NodeExpressionInt {
                    int_literal: token
                }));
            },
            TokenType::Identifier => {
                return Some(ExpressionTypes::Identifier(NodeExpressionIdentifier {
                    ident: token
                }));
            },
            _ => None
        }
    }
    pub fn create_nodes(&mut self) -> Result<Vec<Box<dyn Node>>, String> {
        let mut nodes: Vec<Box<dyn Node>> = vec![];
        while let Some(token) = self.peek(0) {
            match token.t_type {
                TokenType::Return => {
                    if self.peek(1).is_some() {
                        if let Some(expr) = self.parse_expression() {
                            nodes.push(Box::new(NodeExit {
                                expr
                            }));
                        }
                    }
                    else {
                        nodes.push(Box::new(NodeExit {
                            expr: ExpressionTypes::Int(NodeExpressionInt {
                                int_literal: Token {
                                    t_type: TokenType::IntegerLiteral,
                                    value: Some("0".to_string()),
                                },
                            }),
                        }));
                    }

                    // if self.peek(0).is_some() {
                    //     if matches!(self.peek(0).unwrap().t_type, TokenType::IntegerLiteral) {
                    //         nodes.push(Box::new(NodeExit {
                    //             expr: ExpressionTypes::Int(NodeExpressionInt {
                    //                 int_literal: self.peek(0).unwrap(),
                    //             }),
                    //         }));
                    //     } else if matches!(self.peek(0).unwrap().t_type, TokenType::Identifier) {
                    //         // nodes.push(Box::new(NodeExpressionIdentifier))
                    //         nodes.push(Box::new(NodeExit {
                    //             expr: ExpressionTypes::Identifier(NodeExpressionIdentifier {
                    //                 ident: self.peek(0).unwrap(),
                    //             }),
                    //         }));
                    //     }
                    // } else {
                    //     nodes.push(Box::new(NodeExit {
                    //         expr: ExpressionTypes::Int(NodeExpressionInt {
                    //             int_literal: Token {
                    //                 t_type: TokenType::IntegerLiteral,
                    //                 value: Some("0".to_string()),
                    //             },
                    //         }),
                    //     }));
                    // }
                }
                TokenType::LetKeyword => {
                    if self.peek(1).is_some() && // the identifier-name self
                        self.peek(1).is_some() && // This is the '='
                        self.peek(2).is_some() {// the expr 
                        self.consume(); // the 'let' keyword
                        let identifier = self.consume().unwrap(); //  the identifier self
                        self.consume(); // the '='
                        if let Some(expr) = self.parse_expression() {
                            nodes.push(Box::new(NodeVariable {
                                identifier,
                                expr
                            }))
                        }
                        else {
                            todo!()
                            // weird identifier
                        }
                    }
                    // if self.peek(0).is_some()
                    //     && matches!(self.peek(0).unwrap().t_type, TokenType::Identifier)
                    //     && self.peek(1).is_some()
                    //     && matches!(self.peek(1).unwrap().t_type, TokenType::Equal)
                    //     && self.peek(2).is_some()
                    // {
                    //     match self.peek(2).unwrap().t_type {
                    //         TokenType::IntegerLiteral => {
                    //             nodes.push(Box::new(NodeVariable {
                    //                 identifier: self.peek(0).unwrap(),
                    //                 expr: Box::new(NodeExpressionInt {
                    //                     int_literal: self.peek(2).unwrap(),
                    //                 }),
                    //             }));
                    //         }
                    //         TokenType::Identifier => {
                    //             nodes.push(Box::new(NodeVariable {
                    //                 identifier: self.peek(0).unwrap(),
                    //                 expr: Box::new(NodeExpressionIdentifier {
                    //                     ident: self.peek(2).unwrap(),
                    //                 }),
                    //             }));
                    //         }
                    //         _ => {}
                    //     }
                    // }
                }
                TokenType::Identifier => {
                    if &token.value.unwrap() == "exit" {
                        if self.peek(1).is_some() && // Open param '('
                            matches!(self.peek(1).unwrap().t_type, TokenType::OpenParam) &&
                            self.peek(2).is_some() &&  // the expr self 
                            self.peek(3).is_some() &&  // the closing bracket ')'
                            matches!(self.peek(3).unwrap().t_type, TokenType::CloseParam) {
                                self.consume(); // the exit identifier self
                                self.consume(); // the open bracket
                                if let Some(inside) = self.parse_expression() {
                                    nodes.push(Box::new(NodeExit {
                                        expr: inside
                                    }))
                                }
                                else  {
                                    return Err("Exit needs atleast one parameter (0 provided)".to_string())
                                }
                                self.consume(); // closing bracket
                            }
                        
                        // if self.peek(0).is_some()
                        //     && matches!(self.peek(0).unwrap().t_type, TokenType::OpenParam)
                        //     && self.peek(2).is_some()
                        //     && matches!(self.peek(2).unwrap().t_type, TokenType::CloseParam)
                        // {
                        //     if let Some(inner) = self.peek(1) {
                        //         match inner.t_type {
                        //             TokenType::Identifier => nodes.push(Box::new(NodeExit {
                        //                 expr: ExpressionTypes::Identifier(
                        //                     NodeExpressionIdentifier { ident: inner },
                        //                 ),
                        //             })),
                        //             TokenType::IntegerLiteral => {
                        //                 nodes.push(Box::new(NodeExit {
                        //                     expr: ExpressionTypes::Int(NodeExpressionInt {
                        //                         int_literal: inner,
                        //                     }),
                        //                 }));
                        //             }
                        //             _ => {}
                        //         }
                        //     }
                        // } else {
                        //     return Err("exit() must have one parameter".to_string());
                        // }
                    }
                }
                _ => {}
            };
        }

        Ok(nodes)
    }
    fn peek(&self, offset: usize) -> Option<Token> {
        self.tokens.get(offset + self.pointer_index).cloned()
    }
    fn consume(&mut self) -> Option<Token> {
        let before = self.peek(0);
        self.pointer_index += 1;
        before
    }
}
