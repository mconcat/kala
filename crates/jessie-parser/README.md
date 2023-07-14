# Parser

## Lexer

Lexer is designed as a pushdown automata.

### Top Level

Default state for module lexing. 

- "function" => Function
- "if", "while", "for", "switch" => Control Statement
- "const", "let" => Variable Declaration
- "{" => Block
- "import", "export" => Module Declaration

### Module Declaration

#### Import

#### Export

### Variable Declaration

#### Const

#### Let

### Function

- Identifier?
- Parenthesized
- Block

### Control Statement

#### If

- If
- Parenthesized
- Block

#### While

- While
- Parenthesized
- Block

#### For

- For
- LeftParen
- Expression
- Of =>
-   Expression
- Semicolon =>
-   Expression
-   Semicolon
-   Expression
- RightParen
- Block

#### Switch

- Switch
- LeftParen
- Expression
- RightParen
- Block

### Escape Statement

#### Break

- Break

#### Return

- Return
- Expression?

#### Throw

- Throw
- Expression

### Block

- LeftBrace
- (Statement Semicolon)*
- RightBrace

### Parenthesized

- LeftParen | ArrowLeftParen
- (Expression Comma) *
- RightParen | ArrowRightParen
- FatArrow?

### DataLiteral

### Array

### Record

### Operation