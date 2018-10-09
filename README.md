[![Build Status](https://travis-ci.org/matthieugouel/bjorn.svg?branch=master)](https://travis-ci.org/matthieugouel/bjorn)
![Crates.io](https://img.shields.io/crates/v/bjorn.svg)

# Bjørn

Another side project interpreter.

## Installation

The easiest way to install the interpreter is to use `cargo`.

```
$ cargo install bjorn
```


## Usage

First write some code in `bjorn` language in a file (ex. test.bj).

```
a = 1
b = 1

a + b
```

Then simply pass the file path as an argument to the `bjorn` interpreter.

```
$ bjorn test.bj
```

It should output :

```
2
```

## Uninstallation

```
cargo uninstall bjorn
```
