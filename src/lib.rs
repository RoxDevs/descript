#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]

use ast::Stmt;
use chumsky::{Parser, IterParser};
use codegen::CodeGen;
use inkwell::context::Context;
use parser::stmt;

mod parser;
mod ast;
pub mod codegen;

// pub fn run(inp: &str) {
//     let parser = stmt().repeated().collect::<Vec<Stmt>>();
//     let stmts = parser.parse(inp).unwrap();
//     let ctx = Context::create();
//     let mut compiler = CodeGen::new(&ctx);
//     compiler.compile(stmts);
//     compiler.execute()
// }