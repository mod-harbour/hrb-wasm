use std::convert::TryFrom;
use std::error::Error;

use crate::hrb::{Function, FunctionScope, HrbBody, Symbol};
use nom::{
    bytes::complete::{tag, take, take_until},
    combinator::{all_consuming, verify},
    multi::count,
    number::complete::{le_u32, le_u8},
    IResult, Parser,
};

use super::SymbolType;

fn value(input: &[u8]) -> IResult<&[u8], usize> {
    verify(le_u32.map(|size| size as usize), |ret| *ret <= 0x00FFFFFF)(input)
}

fn id(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, id) = take_until(b"\0" as &[u8])(input)?;
    // Consume the null byte.
    let (input, _) = take(1usize)(input)?;
    Ok((input, id))
}

fn symbol(input: &[u8]) -> IResult<&[u8], Symbol> {
    let (input, name) = id(input)?;
    let (input, scope) = le_u8.map(FunctionScope::from).parse(input)?;
    let (input, symbol_type) =
        verify(le_u8.map(SymbolType::try_from), Result::is_ok).parse(input)?;

    let ret = Symbol {
        name,
        scope,
        symbol_type: symbol_type.unwrap(),
    };
    Ok((input, ret))
}

fn function(input: &[u8]) -> IResult<&[u8], Function> {
    let (input, name) = id(input)?;
    let (input, size) = value(input)?;
    let (input, pcode) = take(size)(input)?;

    let ret = Function { name, pcode };
    Ok((input, ret))
}

fn hrb(input: &[u8]) -> IResult<&[u8], HrbBody> {
    // Head
    let (input, _) = tag(b"\xC0HRB")(input)?;
    let (input, _) = take(2usize)(input)?; // Consume the version.

    // Symbols
    let (input, num_symbols) = verify(value, |size| *size != 0)(input)?;
    let (input, symbols) = count(symbol, num_symbols)(input)?;

    let startup_symbol = symbols
        .iter()
        .enumerate()
        .filter(|(_, symbol)| symbol.scope.first && symbol.scope.is_initexit())
        .map(|(idx, _)| idx)
        .next();

    // Functions
    let (input, num_funcs) = value.map(|size| size as usize).parse(input)?;
    let (input, functions) = count(function, num_funcs)(input)?;

    let ret = HrbBody {
        symbols,
        functions,
        startup_symbol,
    };
    Ok((input, ret))
}

pub fn parse_hrb(input: &[u8]) -> Result<HrbBody, Box<dyn Error + '_>> {
    let (_, hrb) = all_consuming(hrb)(input)?;
    Ok(hrb)
}
