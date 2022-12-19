pub const PLUS: u8 = b'+';
pub const MINUS: u8 = b'-';
pub const TIMES: u8 = b'*';
pub const COLUMN: u8 = b':';
pub const SEMICOLON: u8 = b';';
pub const LPAREN: u8 = b'(';
pub const RPAREN: u8 = b')';
pub const LBRACE: u8 = b'{';
pub const RBRACE: u8 = b'}';
pub const LBRACKET: u8 = b'[';
pub const RBRACKET: u8 = b']';
pub const EQUAL: u8 = b'=';
pub const UNDERSCORE: u8 = b'_';
pub const QUESTION: u8 = b'?';
pub const DOT: u8 = b'.';
pub const TILDE: u8 = b'~';
pub const DOUBLE_TAG: &[u8; 2] = b"##";
pub const TAG: u8 = b'#';
pub const CIRCUMFLEX: u8 = b'^';
pub const DOLLAR: u8 = b'$';
pub const EQQ: &[u8; 2] = b"==";
pub const LESS: u8 = b'<';
pub const GREATER: u8 = b'>';
pub const LESS_EQ: &[u8; 2] = b"<=";
pub const GREATER_EQ: &[u8; 2] = b">=";
pub const NOT_EQ: &[u8; 2] = b"!=";
pub const NAT_LESS: &[u8; 2] = b"#<";
pub const NAT_LEQ: &[u8; 3] = b"#<=";
pub const WHITESPACE: &[u8; 4] = b" \t\r\n";
pub const COMMENT: &[u8; 2] = b"//";
pub const COMMENT_START: &[u8; 2] = b"/*";
pub const COMMENT_END: &[u8; 2] = b"*/";
pub const BINARY: &[u8; 2] = b"01";
pub const TAG_SIGN: &[u8; 2] = &[TAG, DOLLAR];
pub const TYPE: &[u8; 4] = b"Type";