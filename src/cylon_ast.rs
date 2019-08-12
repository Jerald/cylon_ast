use std::collections::HashMap;

use serde::{Serialize, Deserialize};

const CYLON_AST_VERSION: &str = "1.0.0";

#[derive(Serialize, Deserialize)]
#[serde(rename = "root")]
#[serde(rename_all = "lowercase")]
pub struct Root
{
    pub version: String,
    pub metadata: HashMap<String, String>,
    pub program: Program
}

impl Root
{
    pub fn new(program: Program) -> Root
    {
        Root {
            program,
            ..Default::default()
        }
    }
}

impl Default for Root
{
    fn default() -> Self
    {
        let mut metadata = HashMap::new();
        metadata.insert("exporter".to_owned(), format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));

        Root {
            version: CYLON_AST_VERSION.to_owned(),
            metadata,
            program: Program {
                lines: vec![]
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "program")]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub struct Program
{
    pub lines: Vec<Line>
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "line")]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub struct Line
{
    pub comment: String,
    pub code: Vec<Statement>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum Statement
{
    #[serde(rename = "statement::goto")]
    Goto { expression: Expression },

    #[serde(rename = "statement::if")]
    If { condition: Expression, body: Vec<Statement>, else_body: Vec<Statement> },

    #[serde(rename = "statement::assignment")]
    Assignment { identifier: String, operator: String, value: Expression },

    #[serde(rename = "statement::expression")]
    Expression { expression: Expression }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum Expression
{
    #[serde(rename = "expression::group")]
    Group { group: Box<Expression> },

    #[serde(rename = "expression::binary_op")]
    BinaryOp { operator: String, left: Box<Expression>, right: Box<Expression> },

    #[serde(rename = "expression::unary_op")]
    UnaryOp { operator: String, operand: Box<Expression> },

    #[serde(rename = "expression::number")]
    Number { num: String },

    #[serde(rename = "expression::string")]
    String { str: String },

    #[serde(rename = "expression::identifier")]
    Identifier { name: String }
}