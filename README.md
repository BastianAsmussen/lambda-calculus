# λ-calculus Interpreter

A [λ-calculus](https://en.wikipedia.org/wiki/Lambda_calculus) interpreter
written in Rust.

# Syntax

Below is the snyax of λ-calculus, written in [BNF](https://en.wikipedia.org/wiki/Backus-Naur_form).

## BNF

```bnf
 <expression>  :== <abstraction> | <application> | <variable>
 <abstraction> :== λ <variable> . <expression>
 <application> :== ( <expression> <expression> )
 <variable>    :== v1 | v2 | ...
```

