pub struct Definition {
    pub name: String,
    pub args: Vec<String>,
    pub body: Expression,
}

pub enum Expression {
    Unit(Atom),
    Let(String, Box<Expression>, Box<Expression>),
    // Function application
    Apply(String, Vec<Atom>),
    // Simple match statement, no patterns
    Match(Atom, Vec<(usize, Expression)>, Option<Box<Expression>>),
    // Primitive operations
    Operate(Operation, Atom, Atom),
}

pub enum Atom {
    Var(String),
    Lit(usize),
}

pub enum Operation {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Comparison
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
    // Bitwise
    And,
    Or,
    Xor,
    Sr,
    Sl,
}
