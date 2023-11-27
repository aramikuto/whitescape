## Description

This project entails a Rust implementation of a compiler for the [Whitespace](http://compsoc.dur.ac.uk/whitespace/tutorial.php) programming language.

## Example programs

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

---

**Note**: _This project is currently a work in progress._

The interpreter can be found in the [adjacent folder](../interpreter/).
