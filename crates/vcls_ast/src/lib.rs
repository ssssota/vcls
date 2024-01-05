pub struct Pos(usize, usize);

pub struct Vcl {
    pub declarations: Vec<Declaration>,
}

pub type Obj = Vec<(String, Literal)>;

pub enum Declaration {
    Include {
        path: String,
    },
    Import {
        ident: String,
    },
    Subroutine {
        name: String,
        return_type: Type,
        body: Vec<Statement>,
    },
    Acl {
        name: String,
        entries: Vec<AclEntry>,
    },
    Backend {
        name: String,
        config: Option<Obj>,
    },
    Director {
        name: String,
        typ: DirectorType,
        config: Obj,
        directions: Vec<Obj>,
    },
    PenaltyBox,
    RateCounter,
    Table {
        name: String,
        typ: Type,
        values: Vec<Literal>,
    },
}

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

pub enum AclEntry {
    Ipv4 {
        addr: [u8; 4],
        cidr: u8,
        negated: bool,
    },
    Ipv6 {
        addr: [u8; 16],
        cidr: u8,
        negated: bool,
    },
}

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

pub struct Object {}

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
pub struct IfStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub else_ifs: Vec<ElseIfStatement>,
    pub else_: Option<Vec<Statement>>,
}
pub struct ElseIfStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

pub struct SetStatement {
    pub target: Variable,
    pub operator: SetOperator,
    pub value: Expression,
}
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
pub struct UnsetStatement {
    pub target: Variable,
    pub keyword: UnsetKeyword,
}
pub enum UnsetKeyword {
    Unset,
    Remove,
}
pub struct AddStatement {
    pub target: Variable,
    pub value: Expression,
}
pub struct CallStatement {
    pub target: String,
}
pub struct DeclareStatement {
    pub target: Variable,
    pub typ: Type,
}
pub struct ErrorStatement {
    pub status: Option<u8>,
    pub message: Option<String>,
}
pub struct EsiStatement;
pub struct IncludeStatement {
    pub path: String,
}
pub struct LogStatement {
    pub message: String,
}
pub struct RestartStatement;
pub struct ReturnStatement {
    pub value: Option<Expression>,
}
pub struct SyntheticStatement {
    pub value: Option<Expression>,
    pub base64: bool,
}

pub enum Expression {
    Literal(Literal),
    Variable(Variable),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Call(CallExpression),
}
pub struct Variable {
    pub name: String,
    pub properties: Vec<String>,
    pub sub_field: Option<String>,
}
pub struct BinaryExpression {
    pub lhs: Box<Expression>,
    pub operator: BinaryOperator,
    pub rhs: Box<Expression>,
}
pub enum BinaryOperator {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    AmpAmp,
    BarBar,
    Add,
    Sub,
    Mul,
    Div,
}
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub rhs: Box<Expression>,
}
pub enum UnaryOperator {
    Not,
    Neg,
}
pub struct CallExpression {
    pub target: Variable,
    pub arguments: Vec<Expression>,
}
