# Whitescape

## Table of Contents

- [Description](#description)
- [Syntax](#syntax)
- [Example Programs](#example-programs)
- [Work in Progress](#work-in-progress)
- [Interpreter](#interpreter)

## Description

The whitescape is a project that aims to transpile a custom high-order-language into the esoteric [Whitespace](http://compsoc.dur.ac.uk/whitespace/tutorial.php) programming language using Rust. I created it to familiarize myself with Rust 🦀 and challenge myself by building a complex program in an unfamiliar field.

## Syntax

### Expressions

The transpiler supports basic arithmetic operations: addition, subtraction, multiplication, and division. Operations are executed from left to right, diverging from the conventional mathematical order. It's crucial to separate all operands by a space for valid expressions. For instance, instead of `1+2+3`, it should be written as `1 + 2 + 3`.

### Example programs

<details>
  <summary>Print a number</summary>
  
  ```
  print(101);
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

You can find the interpreter implementation [here](https://github.com/aramikuto/code-challenges/tree/main/challenges/whitespace/interpreter).

---

**Note**: _This project is currently a work in progress._
