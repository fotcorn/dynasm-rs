use proc_macro_error::emit_error;
use syn::parse;

use crate::arch::Arch;
use crate::common::{Jump, Size, Stmt};
use crate::State;

mod parser;
mod ast;

struct Context<'a, 'b: 'a> {
    pub state: &'a mut State<'b>
}

#[derive(Clone, Debug)]
pub struct ArchRiscv64 {}

impl Default for ArchRiscv64 {
    fn default() -> ArchRiscv64 {
        ArchRiscv64 {}
    }
}

impl Arch for ArchRiscv64 {
    fn name(&self) -> &str {
        "riscv64"
    }

    fn set_features(&mut self, features: &[syn::Ident]) {
        if let Some(feature) = features.first() {
            emit_error!(feature, "Arch riscv64 has no known features");
        }
    }

    fn handle_static_reloc(&self, _stmts: &mut Vec<Stmt>, reloc: Jump, _size: Size) {
        let span = reloc.span();
        emit_error!(span, "Static relocation not supported");
    }

    fn default_align(&self) -> u8 {
        0
    }

    fn compile_instruction(
        &self,
        state: &mut State,
        input: parse::ParseStream,
    ) -> parse::Result<()> {
        
        let mut ctx = Context {
            state
        };

        let (instruction, args) = parser::parse_instruction(&mut ctx, input)?;
        
        Ok(())
    }
}
