mod parser;

use std::{convert::TryFrom, error::Error};

use parser::parse_hrb;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct FunctionScope {
    pub public: bool,
    pub r#static: bool,
    pub first: bool,
    pub init: bool,
    pub exit: bool,
    pub message: bool,
    pub memvar: bool,
}

impl FunctionScope {
    fn is_initexit(&self) -> bool {
        self.init && self.exit
    }
}

impl Into<u8> for FunctionScope {
    fn into(self) -> u8 {
        let mut ret = 0;
        if self.public {
            ret |= 0x01;
        }
        if self.r#static {
            ret |= 0x02;
        }
        if self.first {
            ret |= 0x04;
        }
        if self.init {
            ret |= 0x08;
        }
        if self.exit {
            ret |= 0x10;
        }
        if self.message {
            ret |= 0x20;
        }
        if self.memvar {
            ret |= 0x80;
        }
        ret
    }
}

impl From<u8> for FunctionScope {
    fn from(scope: u8) -> Self {
        FunctionScope {
            public: scope & 0x01 != 0,
            r#static: scope & 0x02 != 0,
            first: scope & 0x04 != 0,
            init: scope & 0x08 != 0,
            exit: scope & 0x10 != 0,
            message: scope & 0x20 != 0,
            memvar: scope & 0x80 != 0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum SymbolType {
    NoLink,
    Function,
    External,
    Deferred,
}

impl Into<u8> for SymbolType {
    fn into(self) -> u8 {
        match self {
            SymbolType::NoLink => 0,
            SymbolType::Function => 1,
            SymbolType::External => 2,
            SymbolType::Deferred => 3,
        }
    }
}

impl TryFrom<u8> for SymbolType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(SymbolType::NoLink),
            1 => Ok(SymbolType::Function),
            2 => Ok(SymbolType::External),
            3 => Ok(SymbolType::Deferred),
            _ => Err(()),
        }
    }
}

struct Symbol<'a> {
    name: &'a [u8],
    scope: FunctionScope,
    symbol_type: SymbolType,
}

struct Function<'a> {
    name: &'a [u8],
    pcode: &'a [u8],
}

pub struct HrbBody<'a> {
    symbols: Vec<Symbol<'a>>,
    functions: Vec<Function<'a>>,
    startup_symbol: Option<usize>,
}

pub fn load(body: &[u8]) -> Result<HrbBody, Box<dyn Error + '_>> {
    let hrb_body = parse_hrb(body)?;

    // TODO: PCODE "linking"

    Ok(hrb_body)
}
