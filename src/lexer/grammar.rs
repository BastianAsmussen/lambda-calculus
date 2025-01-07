pub const GRAMMAR: &str = r"
  <digit>   ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
  <integer> ::= <digit> | <integer> <digit>

  <expression>  ::= <abstraction> | <application> | <variable>
  <abstraction> ::= 'l' <variable> '.' <expression>
  <application> ::= '(' <expression> <expression> ')'
  <variable>    ::= 'v' <integer>
";
