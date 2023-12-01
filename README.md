# Whitescape

## Table of Contents

- [Description](#description)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [Syntax](#syntax)
  - [Expressions](#expressions)
  - [Strings](#strings)
- [Built-in Functions](#built-in-functions)
- [Example Programs](#example-programs)

## Description

The whitescape is a project that aims to transpile a custom high-order-language into the esoteric [Whitespace](http://compsoc.dur.ac.uk/whitespace/tutorial.php) programming language using Rust. I created it to familiarize myself with Rust 🦀 and challenge myself by building a complex program in an unfamiliar field.

## Usage

1. Clone the repository.
1. Write your program within the `code` variable inside the `main.rs` file.
1. Run `cargo run`. Upon successful compilation, the resulting whitespace code will be outputted to the `out/a.out` file.

## Roadmap

This project isn't about making a brand-new popular language. It's focused on building a basic yet complete high-level language that will be converted into whitespace code.

- [ ] Implement basic arithmetic and IO.
  - [x] Addition, subtraction.
  - [x] Integer input.
  - [ ] Output for integers and strings.
  - [ ] Multiplication, division, modulo.
  - [x] Input for arbitrary length strings.
- [ ] Implement basic flow control.
  - [x] While loop.
  - [ ] Functions.
- [ ] Implement more advanced data types like strings.
- [ ] Add configuration options for code input and output.
- [ ] Improve project source code readability.
- [ ] Implement improved syntax checking with easily understandable errors.

## Contributing

Any contributions are more than welcome!

## Syntax

### Expressions

The transpiler supports basic arithmetic operations: addition, subtraction, multiplication, and division. Operations are executed from left to right, diverging from the conventional mathematical order. It's crucial to separate all operands by a space for valid expressions. For instance, instead of `1+2+3`, it should be written as `1 + 2 + 3`.

### Strings

```
string[13] greeting = "Hello world!"
```

The code above defines a string with an initial value. String sizes are mandatory; the size must account for the null terminator placed at the end of the string (`string[2] one_symbol = "a"`).

## Built-in Functions

#### `concat(target, source)`

Concatenate the content of the source to the top of the target.

### Example Programs

<details>
  <summary>Hello world</summary>
  
  ```
  print("Hello, world");
  exit;
  ```
</details>

<details>
  <summary>Print a number</summary>
  
  ```
  print(101);
  exit;
  ```
</details>

<details>
  <summary>Read a number</summary>
  
  ```
  int a;
  read(a);
  print(a + 1);
  exit;
  ```
</details>

<details>
  <summary>Print an integer variable</summary>
  
  ```
  int m = 11;
  print(m);
  exit;
  ```
</details>

<details>
  <summary>Basic while loop</summary>

```
int m = 8;
while (m < 11) {
    print(m);
    m = m + 1;
}
exit;
```

</details>

<details>
  <summary>Great user</summary>

```
string[32] greeting = "Hello, ";
string[25] name;
read(name);
concat(greeting, name);
print(greeting);
exit;
```

</details>

---

**Note**: _Please note that this project is a work in progress._
