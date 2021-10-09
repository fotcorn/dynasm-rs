use super::ast::{Instruction, RawArg, RegId};
use super::Context;
use crate::parse_helpers::parse_ident_or_rust_keyword;
use syn::{parse, Token};

pub(super) fn parse_instruction(
    ctx: &mut Context,
    input: parse::ParseStream,
) -> parse::Result<(Instruction, Vec<RawArg>)> {
    let span = input.cursor().span();

    let op = parse_ident_or_rust_keyword(input)?;
    let mut args = Vec::new();

    // parse 0 or more comma-separated args
    if !(input.is_empty() || input.peek(Token![;])) {
        args.push(parse_arg(ctx, input)?);

        while input.peek(Token![,]) {
            // skip ,
            let _: Token![,] = input.parse()?;

            args.push(parse_arg(ctx, input)?);
        }
    }

    Ok((Instruction { ident: op, span }, args))
}

fn parse_arg(_ctx: &mut Context, input: parse::ParseStream) -> parse::Result<RawArg> {
    let span = input.cursor().span();

    let name = match input.step(|cursor| {
        if let Some((ident, rest)) = cursor.ident() {
            let mut ident = ident.to_string();

            
        }
        Err(cursor.error("expected identifier"))
    }) {
        Ok(name) => name,
        Err(_) => return Ok(None)
    };

    Ok(RawArg::Direct {
        reg: RegId::A0,
        span
    })
}
