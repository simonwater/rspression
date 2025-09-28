pub struct Precedence;

impl Precedence {
    pub const PREC_NONE: i32 = 0;
    pub const PREC_ASSIGNMENT: i32 = 1;   // =
    pub const PREC_OR: i32 = 2;          // or
    pub const PREC_AND: i32 = 3;          // and
    pub const PREC_EQUALITY: i32 = 4;     // == !=
    pub const PREC_COMPARISON: i32 = 5;   // < > <= >=
    pub const PREC_TERM: i32 = 6;        // + -
    pub const PREC_FACTOR: i32 = 7;       // * / %
    pub const PREC_POWER: i32 = 8;        // **
    pub const PREC_UNARY: i32 = 9;        // ! -
    pub const PREC_CALL: i32 = 10;        // . ()
    pub const PREC_PRIMARY: i32 = 11;     // number, string, id
}
