use proc_macro2::Span;
use syn;

// Contains the actual instruction mnemnonic.
#[derive(Debug)]
pub struct Instruction {
    pub span: Span,
    pub ident: syn::Ident,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RegId {
    A0 = 0x01,
}

// basic parse results, before we start doing any kind of checking
#[derive(Debug)]
pub enum RawArg {
    // direct register reference
    Direct { span: Span, reg: RegId },
    // just an arbitrary expression
    Immediate { prefixed: bool, value: syn::Expr }
}
