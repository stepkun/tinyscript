# tinyscript - A small, C-like scripting language

__tinyscript__ is considered to ba a superset of the scripting language
defined in [BehviorTree.CPP](https://www.behaviortree.dev/docs/guides/scripting).

The implementation follows the pattern of `clox` as described in Part III of [crafting interpreters](https://craftinginterpreters.com/)

## Usage

```rust
use tinyscript::{Runtime, environment::DefaultEnvironment};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = Runtime::default();
    let mut env = DefaultEnvironment::default();
    let value = runtime.run("2 * 2 == 4", &mut env)?;
    Ok(())
}
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above,
without any additional terms or conditions.


## Data types

| Type        | Examples         |
|-------------|------------------|
| Boolean     | true/false       |
| String      | 'hello world'    |
| Enum        | RED, GREEN, BLUE |
| Numbers:    |                  |
| Integer     | 42               |
| Hexadecimal | 0x01             |
| Float       | 3.14             |

Note: under the hood an Enum is always interpreted as its integer value.

### Boolean Values
Boolean values can be one of `true` or `false`.

Setting booleans:
```no-test
val_a = true
val_b := !false
```
The logical `!` works with boolean literals. 
`!false` is the equivalent of `true`. 
`val_a` and `val_b` above are equivalent. 

### Strings

Strings are enclosed by '...'.

### Enums

Enums must be registered to the Runtime before they can be used.

### Numbers

#### Negative numbers

| Operator | Description |
|---------|--------------|
| ~       |  Negate      |


## Statements

Examples:

```no-test
print 42
print 42;
42; print 42; variable:= 42; (13 + 12)
```

Multiple statements in a single script are separated by a  __semicolon__.
The last statements may or may not end with a semicolon.

## Assignment operators

Examples:

```no-test
var_a := 42
var_b = 3.14
message = 'hello world'
```

- The first line assigns the number 42 to the variable __var_a__.
- The second line assigns the number 3.14 to the variable __var_b__.
- The third line assigns the string "hello world" to the variable __message__.

## Arithmetic operators and parenthesis 

Examples:

```no-test
var_a := 7
var_b := 5
var_b *= 2
var_c := (var_a * 3) + var_b
```

The resulting values of `var_a` is 10 and `var_c` is 31. 

The following operators are supported:

| Operator | Assign Operator  | Description |
|----------|------------------|-------------|
| +        |  +=              | Add         |
| -        |  -=              | Subtract    |
| *        |  *=              | Multiply    |
| /        |  /=              | Divide      |

These operators can be used only on Number data types, only the addition acan also be used on Strings.

# Bitwise operators

These operators work only on integer and hexadecimal numbers.
Using them with a string or float number will cause an error.

Examples:

```no-test
value:= 0x7F
val_a:= value & 0x0F
val_b:= value | 0xF0
```

The value of `val_a` is 0x0F (or 15); `val_b` is 0xFF (or 255). 

| Operator | Description |
|----------|-------------|
| \|      |  Bitwise or  |
| &       |  Bitwise and |
| ^       |  Bitwise xor |

## Logic and comparison operators

Operators which return a boolean.

Example:

```no-test
val_a := true
val_b := 5 > 3
val_c := (val_a == val_b)
val_d := (val_a && val_b) || !val_c
```

| Operator | Description    |
|----------|----------------|
| &&       |  Logic and     |
| \|\|     |  Logic or      |
| !        |  Negation      |
| ==       |  Equality      |
| !=       |  Inequality    |
| <        |  Less          |
| <=       |  Less equal    |
| >        |  Greater       |
| >=       |  Greater equal |


## Ternary operator **if-then-else**

Example:

```no-test
val_b = (val_a > 1) ? 42 : 24
```
