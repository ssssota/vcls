use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Pos(usize, usize);

#[derive(Debug, PartialEq, Clone)]
pub struct Vcl {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub entries: Vec<(String, ObjectValue)>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectValue {
    Literal(Literal),
    Ident(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Include(IncludeDeclaration),
    Import(ImportDeclaration),
    Subroutine(SubroutineDeclaration),
    Acl(AclDeclaration),
    Backend(BackendDeclaration),
    Director(DirectorDeclaration),
    PenaltyBox(PenaltyBoxDeclaration),
    RateCounter(RateCounterDeclaration),
    Table(TableDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub struct IncludeDeclaration {
    pub path: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportDeclaration {
    pub ident: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SubroutineDeclaration {
    pub name: String,
    pub return_type: Type,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AclDeclaration {
    pub name: String,
    pub entries: Vec<AclEntry>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BackendDeclaration {
    pub name: String,
    pub config: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DirectorDeclaration {
    pub name: String,
    pub typ: DirectorType,
    pub config: Option<Object>,
    pub directions: Vec<Object>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PenaltyBoxDeclaration {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RateCounterDeclaration {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableDeclaration {
    pub name: String,
    pub typ: Type,
    pub entries: Vec<TableEntry>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableEntry {
    pub key: String,
    pub value: TableValue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TableValue {
    Ident(Variable),
    Literal(Literal),
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct AclEntry {
    pub negated: bool,
    pub addr: String,
    pub cidr: u8,
}

#[derive(Debug, PartialEq, Clone)]
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

impl Type {
    pub fn from_str(s: &str) -> Self {
        match s {
            "ACL" => Self::Acl,
            "BACKEND" => Self::Backend,
            "BOOL" => Self::Bool,
            "FLOAT" => Self::Float,
            "ID" => Self::ID,
            "INTEGER" => Self::Integer,
            "IP" => Self::IP,
            "RTIME" => Self::RTime,
            "STRING" => Self::String,
            "TIME" => Self::Time,
            "VOID" => Self::Void,
            _ => Self::Unknown(s.to_string()),
        }
    }
}
impl FromStr for Type {
    type Err = ();
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_str(s))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    RTime(RelativeTime),
    Object(Object),
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
    pub fn from_sec(sec: f64) -> Self {
        Self::from_ms((sec * 1000.0) as u64)
    }
    #[inline]
    pub fn from_min(min: f64) -> Self {
        Self::from_sec(min * 60.0)
    }
    #[inline]
    pub fn from_hour(hour: f64) -> Self {
        Self::from_min(hour * 60.0)
    }
    #[inline]
    pub fn from_day(day: f64) -> Self {
        Self::from_hour(day * 24.0)
    }
    #[inline]
    pub fn from_year(year: f64) -> Self {
        Self::from_day(year * 365.0)
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub else_ifs: Vec<ElseIfStatement>,
    pub else_: Option<Vec<Statement>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElseIfStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetStatement {
    pub target: Variable,
    pub operator: SetOperator,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct UnsetStatement {
    pub target: Variable,
    pub keyword: UnsetKeyword,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnsetKeyword {
    Unset,
    Remove,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AddStatement {
    pub target: Variable,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallStatement {
    pub target: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeclareStatement {
    pub target: Variable,
    pub typ: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ErrorStatement {
    pub status: Option<u8>,
    pub message: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EsiStatement;

#[derive(Debug, PartialEq, Clone)]
pub struct IncludeStatement {
    pub path: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LogStatement {
    pub message: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RestartStatement;

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SyntheticStatement {
    pub value: Option<Expression>,
    pub base64: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    Variable(Variable),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Call(CallExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub properties: Vec<String>,
    pub sub_field: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub lhs: Box<Expression>,
    pub operator: BinaryOperator,
    pub rhs: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    /// `==` (Equality)
    Eq,
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

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub rhs: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    /// `!` (Logical NOT)
    Not,
    /// `-` (Negation)
    Neg,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub target: Variable,
    pub arguments: Vec<Expression>,
}
