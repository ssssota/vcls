use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
pub struct Span(pub usize, pub usize);

#[derive(Debug, PartialEq, Clone)]
pub struct Vcl {
    pub declarations: Vec<Declaration>,
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
    pub span: Span,
    pub path: StringToken,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportDeclaration {
    pub span: Span,
    pub ident: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SubroutineDeclaration {
    pub span: Span,
    pub name: String,
    pub return_type: Type,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AclDeclaration {
    pub span: Span,
    pub name: String,
    pub entries: Vec<AclEntry>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BackendDeclaration {
    pub span: Span,
    pub name: String,
    pub config: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DirectorDeclaration {
    pub span: Span,
    pub name: String,
    pub typ: DirectorType,
    pub config: Option<Object>,
    pub directions: Vec<Object>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PenaltyBoxDeclaration {
    pub span: Span,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RateCounterDeclaration {
    pub span: Span,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableDeclaration {
    pub span: Span,
    pub name: String,
    pub typ: Type,
    pub entries: Vec<TableEntry>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableEntry {
    pub span: Span,
    pub key: StringLiteral,
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
    pub span: Span,
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
    pub fn from_keyword(s: &str) -> Self {
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

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(StringLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    Bool(BoolLiteral),
    RTime(RTimeLiteral),
    Object(Object),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub span: Span,
    pub tokens: Vec<StringToken>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringToken {
    pub span: Span,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerLiteral {
    pub span: Span,
    pub value: i64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FloatLiteral {
    pub span: Span,
    pub value: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BoolLiteral {
    pub span: Span,
    pub value: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RTimeLiteral {
    pub span: Span,
    pub value: RelativeTime,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub entries: Vec<(String, ObjectValue)>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectValue {
    Literal(Literal),
    Ident(String),
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
    pub span: Span,
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub els: Option<ElseStatement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ElseStatement {
    If(Box<IfStatement>),
    Body(Vec<Statement>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetStatement {
    pub span: Span,
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

impl FromStr for SetOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "=" => Ok(Self::Set),
            "+=" => Ok(Self::Add),
            "-=" => Ok(Self::Sub),
            "*=" => Ok(Self::Mul),
            "/=" => Ok(Self::Div),
            "%=" => Ok(Self::Mod),
            "&=" => Ok(Self::Amp),
            "|=" => Ok(Self::Bar),
            "^=" => Ok(Self::Hat),
            "<<=" => Ok(Self::LShift),
            ">>=" => Ok(Self::RShift),
            "ror=" => Ok(Self::Ror),
            "rol=" => Ok(Self::Rol),
            "&&=" => Ok(Self::AmpAmp),
            "||=" => Ok(Self::BarBar),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnsetStatement {
    pub span: Span,
    pub target: Variable,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AddStatement {
    pub span: Span,
    pub target: Variable,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallStatement {
    pub span: Span,
    pub target: Variable,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeclareStatement {
    pub span: Span,
    pub target: Variable,
    pub typ: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ErrorStatement {
    pub span: Span,
    pub status: Option<Expression>,
    pub message: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EsiStatement {
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IncludeStatement {
    pub span: Span,
    pub path: StringToken,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LogStatement {
    pub span: Span,
    pub message: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RestartStatement {
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub span: Span,
    pub value: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SyntheticStatement {
    pub span: Span,
    pub value: Expression,
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
    pub span: Span,
    pub name: String,
    pub properties: Vec<String>,
    pub sub_field: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub span: Span,
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
    pub span: Span,
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
    pub span: Span,
    pub target: Variable,
    pub arguments: Vec<Expression>,
}
