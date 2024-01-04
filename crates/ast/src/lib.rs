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
    Acl,
    Backend,
    Bool,
    Float,
    ID,
    Int,
    IP,
    RTime,
    String,
    Time,
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

pub struct Object {}

pub enum Statement {}
