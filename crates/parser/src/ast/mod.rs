use std::fmt::Debug;
use petgraph::graph::{DiGraph, NodeIndex};
use crate::parsing_error::ParsingError;
use crate::token_stream::TokenStream;
use crate::tokens::{Keyword, TokenType};
use self::nodes::interface::InterfaceASTNode;

pub mod nodes;

pub struct ElpASTContext<'a> {
    pub graph: &'a mut ElpASTGraph,
    pub node_index: &'a NodeIndex,
    pub parent_node_index: Option<&'a NodeIndex>,
    pub edge: ASTNodeEdge,
    pub token_stream: &'a mut TokenStream
}

pub trait ASTNodeProducer {
    fn new() -> Self;
    fn produce(&self, context: &ElpASTContext<'_>) -> Result<Self, ParsingError>;
}

// ASTNode is an expression parsed from the source file.
pub enum ASTNode {
    // Represents the root of the AST tree.
    Root,

    // Type system.
    InterfaceDeclaration(InterfaceASTNode),
    TypeAlias,

    // Statements.
    VariableDeclaration,
    FunctionDeclaration,
    ObjectDeclaration,
    ComponentDeclaration,

    // Expressions.
    BlockStatement
}
#[derive(Debug, Default)]
pub enum ASTNodeEdge {
    #[default]
    Direct,
    Conditional,
    Loop,
    Returns,
    Call,
}

/// The general representation of your scripts is a
/// directed graph of nodes and edges that represent
/// your script in a natural way while maintaining
/// the intended purpose of the script.
pub type ElpASTGraph = DiGraph<ASTNode, ASTNodeEdge>;

pub struct ElpAST {
    pub graph: ElpASTGraph,
    pub root_index: NodeIndex,
    token_stream: TokenStream
}

impl ElpAST {
    pub fn new(token_stream: TokenStream) -> Self {
        let mut graph = ElpASTGraph::new();
        let root_index = graph.add_node(ASTNode::Root);

        Self {
            graph,
            root_index,
            token_stream
        }
    }

    fn parse_tokens(&mut self) {
        while let Some(token) = self.token_stream.token() {
            match token.token_type {
                TokenType::Keyword(Keyword::Interface) => {

                }
                _ => {
                    println!("Unexpected token {:#?}", token);
                }
            }
        }
    }
}