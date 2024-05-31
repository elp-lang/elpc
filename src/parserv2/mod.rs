use std::sync::Arc;

use crate::lexer::lexer;
use crate::lexer::parsing_error::ParsingError;
use crate::parserv2::node::AstNode;
use crate::parserv2::visitor::TokenVisitor;

mod node;
mod visitor;
pub mod visitors;

pub struct Parser<'p> {
    visitors: Vec<Box<&'p mut dyn TokenVisitor>>,
    token_stream: Vec<lexer::Token>,
}

impl<'p> Parser<'p> {
    pub fn new(token_stream: Vec<lexer::Token>) -> Parser<'p> {
        Parser {
            token_stream,
            visitors: Vec::new(),
        }
    }

    pub fn register_visitor(&mut self, visitor: Box<&'p mut dyn TokenVisitor>) -> &mut Self {
        self.visitors.push(visitor);

        self
    }

    pub async fn parse(&mut self) -> Result<Vec<Arc<AstNode>>, ParsingError> {
        let mut accepted_tokens: Vec<lexer::Token> = Vec::new();
        let mut parsed_nodes: Vec<Arc<AstNode>> = Vec::new();

        for visitor in &mut self.visitors {
            for token in self.token_stream.iter() {
                if visitor.accept(token) {
                    accepted_tokens.push(token.to_owned());
                } else {
                    break;
                }
            }

            if accepted_tokens.len() > 0 {
                match visitor.parse(accepted_tokens.to_owned()).await? {
                    Some(node) => {
                        parsed_nodes.push(node);
                    }

                    None => {}
                }
            }
        }

        Ok(parsed_nodes)
    }
}
