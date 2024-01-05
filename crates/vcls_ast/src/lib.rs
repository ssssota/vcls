use std::net::IpAddr;

#[derive(Debug, PartialEq)]
pub struct Pos(usize, usize);

#[derive(Debug, PartialEq)]
pub struct Vcl {
    pub declarations: Vec<Declaration>,
}

pub type Obj = Vec<(String, Literal)>;

#[derive(Debug, PartialEq)]
pub enum Declaration {
    Include(IncludeDeclaration),
    Import(ImportDeclaration),
    Subroutine(SubroutineDeclaration),
    Acl(AclDeclaration),
    Backend(BackendDeclaration),
    Director(DirectorDeclaration),
    PenaltyBox,
    RateCounter,
    Table(TableDeclaration),
}

#[derive(Debug, PartialEq)]
pub struct IncludeDeclaration {
    pub path: String,
}

#[derive(Debug, PartialEq)]
pub struct ImportDeclaration {
    pub ident: String,
}

#[derive(Debug, PartialEq)]
pub struct SubroutineDeclaration {
    pub name: String,
    pub return_type: Type,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct AclDeclaration {
    pub name: String,
    pub entries: Vec<AclEntry>,
}

#[derive(Debug, PartialEq)]
pub struct BackendDeclaration {
    pub name: String,
    pub config: Option<Obj>,
}

#[derive(Debug, PartialEq)]
pub struct DirectorDeclaration {
    pub name: String,
    pub typ: DirectorType,
    pub config: Option<Obj>,
    pub directions: Vec<Obj>,
}

#[derive(Debug, PartialEq)]
pub struct TableDeclaration {
    pub name: String,
    pub typ: Type,
    pub values: Vec<Literal>,
}

#[derive(Debug, PartialEq)]
pub enum DirectorType {
    /// https://developer.fastly.com/reference/vcl/declarations/director/#random
    Random,
    /// https://developer.fastly.com/reference/vcl/declarations/director/#fallback
    Fallback,
    /// https://developer.fastly.com/reference/vcl/declarations/director/#content
    Content,
    /// https://developer.fastly.com/reference/vcl/declarations/director/#client
    Client,
    /// https://developer.fastly.com/reference/vcl/declarations/director/#consistent-hashing
    ConsistentHashing,
    /// fallback
    Unknown(String),
}

#[derive(Debug, PartialEq)]
pub struct AclEntry {
    pub negated: bool,
    pub addr: IpAddr,
    pub cidr: u8,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    /// https://developer.fastly.com/reference/vcl/types/acl/
    Acl,
    /// https://developer.fastly.com/reference/vcl/types/backend/
    Backend,
    /// https://developer.fastly.com/reference/vcl/types/bool/
    Bool,
    /// https://developer.fastly.com/reference/vcl/types/float/
    Float,
    /// https://developer.fastly.com/reference/vcl/types/id/
    ID,
    /// https://developer.fastly.com/reference/vcl/types/integer/
    Integer,
    /// https://developer.fastly.com/reference/vcl/types/ip/
    IP,
    /// https://developer.fastly.com/reference/vcl/types/rtime/
    RTime,
    /// https://developer.fastly.com/reference/vcl/types/string/
    String,
    /// https://developer.fastly.com/reference/vcl/types/time/
    Time,
    /// void for subroutines
    Void,
    /// fallback
    Unknown(String),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    RTime(RelativeTime),
    Object(Obj),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelativeTime {
    ms: u64,
}
impl RelativeTime {
    #[inline]
    pub fn from_ms(ms: u64) -> Self {
        Self { ms }
    }
    #[inline]
    pub fn from_sec(sec: u64) -> Self {
        Self::from_ms(sec * 1000)
    }
    #[inline]
    pub fn from_min(min: u64) -> Self {
        Self::from_sec(min * 60)
    }
    #[inline]
    pub fn from_hour(hour: u64) -> Self {
        Self::from_min(hour * 60)
    }
    #[inline]
    pub fn from_day(day: u64) -> Self {
        Self::from_hour(day * 24)
    }
    #[inline]
    pub fn from_year(year: u64) -> Self {
        Self::from_day(year * 365)
    }
}
impl std::ops::Add for RelativeTime {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_ms(self.ms + rhs.ms)
    }
}
impl std::ops::Sub for RelativeTime {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_ms(self.ms - rhs.ms)
    }
}
impl std::ops::Mul<u64> for RelativeTime {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: u64) -> Self::Output {
        Self::from_ms(self.ms * rhs)
    }
}
impl std::ops::Div<u64> for RelativeTime {
    type Output = Self;
    #[inline]
    fn div(self, rhs: u64) -> Self::Output {
        Self::from_ms(self.ms / rhs)
    }
}

#[derive(Debug, PartialEq)]
pub struct Object {}

#[derive(Debug, PartialEq)]
pub enum Statement {
    If(IfStatement),
    Set(SetStatement),
    Unset(UnsetStatement),
    Add(AddStatement),
    Call(CallStatement),
    Declare(DeclareStatement),
    Error(ErrorStatement),
    Esi(EsiStatement),
    Include(IncludeStatement),
    Log(LogStatement),
    Restart(RestartStatement),
    Return(ReturnStatement),
    Synthetic(SyntheticStatement),
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub else_ifs: Vec<ElseIfStatement>,
    pub else_: Option<Vec<Statement>>,
}

#[derive(Debug, PartialEq)]
pub struct ElseIfStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct SetStatement {
    pub target: Variable,
    pub operator: SetOperator,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub enum SetOperator {
    Set,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Amp,
    Bar,
    Hat,
    LShift,
    RShift,
    Ror,
    Rol,
    AmpAmp,
    BarBar,
}

#[derive(Debug, PartialEq)]
pub struct UnsetStatement {
    pub target: Variable,
    pub keyword: UnsetKeyword,
}

#[derive(Debug, PartialEq)]
pub enum UnsetKeyword {
    Unset,
    Remove,
}

#[derive(Debug, PartialEq)]
pub struct AddStatement {
    pub target: Variable,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct CallStatement {
    pub target: String,
}

#[derive(Debug, PartialEq)]
pub struct DeclareStatement {
    pub target: Variable,
    pub typ: Type,
}

#[derive(Debug, PartialEq)]
pub struct ErrorStatement {
    pub status: Option<u8>,
    pub message: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct EsiStatement;

#[derive(Debug, PartialEq)]
pub struct IncludeStatement {
    pub path: String,
}

#[derive(Debug, PartialEq)]
pub struct LogStatement {
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub struct RestartStatement;

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct SyntheticStatement {
    pub value: Option<Expression>,
    pub base64: bool,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Variable(Variable),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Call(CallExpression),
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub properties: Vec<String>,
    pub sub_field: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression {
    pub lhs: Box<Expression>,
    pub operator: BinaryOperator,
    pub rhs: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    /// `==` (Equality)

    /// `!=` (Non-equality)
    Ne,
    /// `<` (Less than)
    Lt,
    /// `<=` (Less than or equal to)
    Le,
    /// `>` (Greater than)
    Gt,
    /// `>=` (Greater than or equal to)
    Ge,
    /// `&&` (Logical AND)
    AmpAmp,
    /// `||` (Logical OR)
    BarBar,
    /// `+` (Addition)
    Add,
    /// `-` (Subtraction)
    Sub,
    /// `*` (Multiplication)
    Mul,
    /// `/` (Division)
    Div,
    /// `~` (Regular expression match)
    Tilde,
    /// `!~` (Regular expression non-match)
    NotTilde,
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub rhs: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    /// `!` (Logical NOT)
    Not,
    /// `-` (Negation)
    Neg,
}

#[derive(Debug, PartialEq)]
pub struct CallExpression {
    pub target: Variable,
    pub arguments: Vec<Expression>,
}
