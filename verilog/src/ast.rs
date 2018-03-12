#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Module(pub Ident, pub Option<Vec<ParamArg>>, pub Vec<Arg>);

pub type Arg = (Ident, Dir, Option<BitRange>, Option<DataType>);

pub type ParamArg = (Ident, Num);

pub type BitRange = (Num, Num);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ident(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Dir {
    Input,
    Output,
    InOut,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DataType {
    Reg,
    Integer,
    Bit,
    Logic,
    Byte,
    Int,
    ShortInt,
    LongInt
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Num(pub i32);

