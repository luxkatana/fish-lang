use crate::tokenizer::{Token, TokenType};
use std::any::Any;

pub enum NodeType {
    ExpressionInt,
    ExpressionIdent,
    Exit,
    Variable,
    BinaryExpressionAdd,
    BinaryExpressionMul
}
pub trait Node: Any {
    fn as_any(&self) -> &dyn Any;
    fn nodetype(&self) -> NodeType;
}
pub struct NodeExpressionInt {
    pub int_literal: Token,
}
pub struct NodeBinaryExpressionAdd {
    pub left: ExpressionTypes,
    pub right: ExpressionTypes
}
impl Node for NodeBinaryExpressionAdd {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn nodetype(&self) -> NodeType {
        NodeType::BinaryExpressionAdd
    }
}

pub struct NodeBinaryExpressionMul {
    pub left: ExpressionTypes,
    pub right: ExpressionTypes
}
impl Node for NodeBinaryExpressionMul {
    fn nodetype(&self) -> NodeType {
        NodeType::BinaryExpressionMul
        
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
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
    pub expr: Box<dyn Node>,
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
    pub fn create_nodes(&mut self) -> Result<Vec<Box<dyn Node>>, String> {
        let mut nodes: Vec<Box<dyn Node>> = vec![];
        while let Some(token) = self.consume() {
            match token.t_type {
                TokenType::Return => {
                    if self.peek(0).is_some() {
                        if matches!(self.peek(0).unwrap().t_type, TokenType::IntegerLiteral) {
                            nodes.push(Box::new(NodeExit {
                                expr: ExpressionTypes::Int(NodeExpressionInt {
                                    int_literal: self.peek(0).unwrap(),
                                }),
                            }));
                        } else if matches!(self.peek(0).unwrap().t_type, TokenType::Identifier) {
                            // nodes.push(Box::new(NodeExpressionIdentifier))
                            nodes.push(Box::new(NodeExit {
                                expr: ExpressionTypes::Identifier(
                                    NodeExpressionIdentifier {
                                        ident: self.peek(0).unwrap(),
                                    },
                                ),
                            }));
                        }
                    } else {
                        nodes.push(Box::new(NodeExit {
                            expr: ExpressionTypes::Int(NodeExpressionInt {
                                int_literal: Token {
                                    t_type: TokenType::IntegerLiteral,
                                    value: Some("0".to_string()),
                                },
                            }),
                        }));
                    }
                }
                TokenType::LetKeyword => {
                    if self.peek(0).is_some()
                        && matches!(self.peek(0).unwrap().t_type, TokenType::Identifier)
                        && self.peek(1).is_some()
                        && matches!(self.peek(1).unwrap().t_type, TokenType::Equal)
                        && self.peek(2).is_some()
                    {
                        match self.peek(2).unwrap().t_type {
                            TokenType::IntegerLiteral => {
                                nodes.push(Box::new(NodeVariable {
                                    identifier: self.peek(0).unwrap(),
                                    expr: Box::new(NodeExpressionInt {
                                        int_literal: self.peek(2).unwrap(),
                                    }),
                                }));
                            }
                            TokenType::Identifier => {
                                nodes.push(Box::new(NodeVariable {
                                    identifier: self.peek(0).unwrap(),
                                    expr: Box::new(NodeExpressionIdentifier {
                                        ident: self.peek(2).unwrap(),
                                    }),
                                }));
                            }
                            _ => {}
                        }
                    }
                }
                TokenType::Identifier => {
                    if &token.value.unwrap() == "exit" {
                        if self.peek(0).is_some()
                            && matches!(self.peek(0).unwrap().t_type, TokenType::OpenParam)
                            && self.peek(2).is_some()
                            && matches!(self.peek(2).unwrap().t_type, TokenType::CloseParam)
                        {
                            if let Some(inner) = self.peek(1) {
                                match inner.t_type {
                                    TokenType::Identifier => nodes.push(Box::new(NodeExit {
                                        expr: ExpressionTypes::Identifier(
                                            NodeExpressionIdentifier { ident: inner },
                                        ),
                                    })),
                                    TokenType::IntegerLiteral => {
                                        nodes.push(Box::new(NodeExit {
                                            expr: ExpressionTypes::Int(
                                                NodeExpressionInt { int_literal: inner },
                                            ),
                                        }));
                                    }
                                    _ => {}
                                }
                            }
                        } else {
                            return Err("exit() must have one parameter".to_string());
                        }
                    }
                },
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
