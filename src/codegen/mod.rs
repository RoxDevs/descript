use std::collections::{HashMap, HashSet};

use inkwell::{
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    values::{BasicValueEnum, GlobalValue, PointerValue},
    AddressSpace, OptimizationLevel,
};

use crate::ast::{Kw, Literal, Stmt};

#[derive(Debug)]
pub struct CodeGen<'ctx> {
    ctx: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    defined_str_literals: HashSet<String>,
    str_literal_globals: HashMap<String, GlobalValue<'ctx>>,
}

impl<'a> CodeGen<'a> {
    pub fn new(ctx: &'a Context) -> Self {
        let module = ctx.create_module("program");
        CodeGen {
            ctx,
            module,
            builder: ctx.create_builder(),
            defined_str_literals: HashSet::new(),
            str_literal_globals: HashMap::new(),
        }
    }
    pub fn compile(&mut self, input: Vec<Stmt>) {
        let i64_type = self.ctx.i64_type();
        let main_fn_type =
            i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        let main_fn = self.module.add_function("main", main_fn_type, None);
        let block = self.ctx.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(block);
        for stmt in input {
            self.compile_stmt(stmt)
        }
        self.execute()
    }

    // Thanks https://github.com/phodal-archive/llvm-rust-helloworld
    pub fn execute(&self) {
        let ee = self
            .module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();
        let maybe_fn = unsafe { ee.get_function::<unsafe extern "C" fn() -> f64>("main") };

        let compiled_fn = match maybe_fn {
            Ok(f) => f,
            Err(err) => {
                panic!("{:?}", err);
            }
        };

        unsafe {
            compiled_fn.call();
        }
    }
    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt.kw {
            Kw::Echo => self.compile_echo(stmt.args),
        }
    }
    fn declare_str(&mut self, inp: String) -> GlobalValue {
        let array = self.ctx.i8_type().array_type((inp.len() + 1) as u32);
        let gv = self.module.add_global(array, None, inp.as_str());
        self.str_literal_globals.insert(inp, gv);

        gv
    }
    fn get_or_globalize_str_literal(&mut self, lit: impl Into<String>) -> GlobalValue {
        let lit = lit.into();
        if self.defined_str_literals.get(&lit).is_some() {
            if let Some(g_val) = self.str_literal_globals.get(&lit) {
                g_val.clone()
            } else {
                self.declare_str(lit)
            }
        } else {
            self.define_str(lit)
        }
    }
    fn define_str(&mut self, lit: String) -> GlobalValue {
        let mut charcodes: Vec<_> = lit.clone().chars().map(|c| c as u8).collect();
        charcodes.push(0);

        let array_ty = self.ctx.i8_type().array_type(charcodes.len() as u32);
        let array_vals: Vec<_> = charcodes
            .iter()
            .map(|v| self.ctx.i8_type().const_int((*v).into(), false))
            .collect();

        let global = self.module.add_global(array_ty, None, lit.as_str());
        global.set_initializer(&self.ctx.i8_type().const_array(array_vals.as_slice()));

        self.defined_str_literals.insert(lit.clone());
        self.str_literal_globals.insert(lit.clone(), global.clone());

        global
    }

    fn compile_echo(&mut self, args: Vec<Literal>) {
        assert_eq!(args.len(), 1);
        let arg = args[0].clone();
        let val = match arg {
            Literal::Str(s) => {
                self.get_or_globalize_str_literal(s)
            }
            Literal::Int(_) => todo!(),
            Literal::Float(_) => todo!(),
            Literal::NestedStmt(_) => todo!(),
        };
        drop(args);
    }
}
