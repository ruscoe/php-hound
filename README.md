# PHP Hound

![php-hound-banner](https://github.com/user-attachments/assets/7fcd6bcf-a837-4d00-a101-b60f260120c9)

An opinionated PHP issue sniffer written in Rust.

## Scans for

### Possible accidental assignment

Checks `if`, `elseif`, and `while` for accidential assignment
(`$a = $b` rather than `$a == $b`)

While it is not technically wrong to use something like `if ($a = function()) {}` to
determine if a function returns false or NULL, this is side-effect programming and
can lead to a confusing codebase.

### Incrementing and decrementing variables inside conditions

Checks `if` and `elseif` for incrementing or decrementing variables.

A case of side-effect programming, changing variables inside conditions can
be confusing.

### Use of the eval() function

Using `eval()` is a security risk, allowing possible remote code execution.
