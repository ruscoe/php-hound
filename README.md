# PHP Hound

An opinionated PHP issue sniffer written in Rust.

## Scans for

### Possible accidental assignment

Checks `if`, `elseif`, and `while` for accidential assignment (`$a = b` rather than `$a == $b`)

While it is not technically wrong to use something like `if ($a = function()) {}` to
determine if a function returns false or NULL, this is side-effect programming and
can lead to a confusing codebase.
