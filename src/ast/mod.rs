#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Ast<'source> {
    Add(Add<'source>),
    Assign(Assign<'source>),
    Block(Block<'source>),
    Call(Call<'source>),
    Divide(Divide<'source>),
    Equal(Equal<'source>),
    Function(Function<'source>),
    Id(Id<'source>),
    If(If<'source>),
    Multiply(Multiply<'source>),
    Not(Not<'source>),
    NotEqual(NotEqual<'source>),
    Number(Number),
    Return(Return<'source>),
    Subtract(Subtract<'source>),
    Var(Var<'source>),
    While(While<'source>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Add<'source> {
    pub left: &'source Ast<'source>,
    pub right: &'source Ast<'source>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Assign<'source> {
    pub name: &'source str,
    pub value: &'source Ast<'source>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Block<'source> {
    pub statements: Vec<&'source Ast<'source>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Call<'source> {
    pub callee: &'source str,
    pub args: Vec<&'source Ast<'source>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Divide<'source> {
    pub left: &'source Ast<'source>,
    pub right: &'source Ast<'source>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Equal<'source> {
    pub left: &'source Ast<'source>,
    pub right: &'source Ast<'source>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Function<'source> {
    pub name: &'source str,
    pub parameters: Vec<&'source Ast<'source>>,
    pub body: &'source Ast<'source>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Id<'source> {
    pub value: &'source str,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct If<'source> {
    pub conditional: &'source Ast<'source>,
    pub consequence: &'source Ast<'source>,
    pub alternative: &'source Ast<'source>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Multiply<'source> {
    pub left: &'source Ast<'source>,
    pub right: &'source Ast<'source>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Not<'source> {
    pub term: &'source Ast<'source>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NotEqual<'source> {
    pub left: &'source Ast<'source>,
    pub right: &'source Ast<'source>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Number {
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Return<'source> {
    pub term: &'source Ast<'source>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Subtract<'source> {
    pub left: &'source Ast<'source>,
    pub right: &'source Ast<'source>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Var<'source> {
    pub name: &'source str,
    pub value: &'source Ast<'source>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct While<'source> {
    pub conditional: &'source Ast<'source>,
    pub body: &'source Ast<'source>,
}
