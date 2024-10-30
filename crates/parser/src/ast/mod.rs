use std::fmt::Debug;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::prelude::EdgeRef;
use crate::parsing_error::ParsingError;
use crate::token_stream::TokenStream;
use crate::tokens::{Keyword, TokenType};
use self::nodes::interface::InterfaceASTNode;

pub mod nodes;

pub struct ElpASTContext<'a> {
    pub ast: &'a mut ElpAST<'a>,
    pub parent_node_index: &'a NodeIndex,
    pub token_stream: &'a mut TokenStream
}

pub trait ASTNodeProducer {
    fn new() -> Self;
    fn produce(&self, context: &mut ElpASTContext<'_>) -> Result<ElpASTGraph, Box<ParsingError>> where Self: std::marker::Sized;
}

// ASTNode is an expression parsed from the source file.
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, Default)]
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

pub struct ElpAST<'a> {
    pub graph: &'a mut ElpASTGraph,
    pub root_index: NodeIndex,
    pub token_stream: &'a mut TokenStream,
}

impl ElpAST<'_> {
    pub fn new(token_stream: &mut TokenStream) -> Self {
        let mut graph = ElpASTGraph::new();
        let root_index = graph.add_node(ASTNode::Root);
        
        Self {
            graph,
            root_index,
            token_stream
        }
    }
    
    pub fn produce(&mut self) {
        let mut current_context = ElpASTContext {
            ast: &mut self,
            parent_node_index: &root_index,
            token_stream: &mut token_stream,
        };

        while let Some(token) = &self.token_stream.token() {
        match token.token_type {
        TokenType::Keyword(Keyword::Interface) => {
        match InterfaceASTNode::new().produce(&mut current_context) {
        Ok(new_graph) => {
        self.insert_graph_at_node(&mut self.graph, &new_graph, current_context.parent_node_index);
        }
        Err(err) => {
        println!("Error parsing interface: {:?}", err);
        break;
        }
        }
        }
        _ => {
        println!("Unexpected token {:#?}", token);
        }
        }
        }

        instance
    }
    
    pub fn insert_graph_at_node(
        &mut self,
        graph: &mut ElpASTGraph,
        source: &ElpASTGraph,
        at_node_index: &NodeIndex,
    ) {
        let mut node_map = Vec::new();

        for node in source.node_weights() {
            let new_index = graph.add_node(node.clone());
            node_map.push(new_index);
        }

        for edge in source.edge_references() {
            let source = node_map[edge.source().index()];
            let target = node_map[edge.target().index()];
            graph.add_edge(source, target, edge.weight().clone());
        }

        let root_nodes: Vec<NodeIndex> = source
            .node_indices()
            .filter(|&node| source.edges_directed(node, petgraph::Incoming).count() == 0)
            .collect();

        for root in root_nodes {
            let mapped_root = node_map[root.index()];
            graph.add_edge(at_node_index, mapped_root, Default::default());
        }
    }
}