# shoggoth.rs

Generic and type-level programming for Rust

[![build status](https://api.travis-ci.org/epsilonz/shoggoth.rs.svg?branch=master)](https://travis-ci.org/epsilonz/shoggoth.rs)

_It was a terrible, indescribable thing vaster than any subway train—a shapeless congeries of protoplasmic bubbles, faintly self-luminous, and with myriads of temporary eyes forming and un-forming as pustules of greenish light all over the tunnel-filling front that bore down upon us, crushing the frantic penguins and slithering over the glistening floor that it and its kind had swept so evilly free of all litter._

—H. P. Lovecraft, _At The Mountains of Madness_

## Synopsis

This library implements generic and type-level programming functionality. It is heavily inspired by Miles Sabin's [Shapeless](https://github.com/milessabin/shapeless) for Scala.

## Features

#### Current

* generic programming with tuples
* type-level booleans and bits
* type-level lists and difference lists
* type-level binary natural numbers
* type macros
* reflection of static type-level data to value-level runtime data

## Documentation

See the API documentation [here](http://epsilonz.github.io/shoggoth.rs/doc/shoggoth/).

## Requirements

NOTE: Shoggoth depends on an experimental type macros feature
described in
[this RFC](https://github.com/rust-lang/rfcs/pull/873). The feature is
implemented in
[this rust branch](https://github.com/freebroccolo/rust/tree/feature/type_macros).

1.   [Rust](http://www.rust-lang.org/)
2.   [Cargo](http://crates.io/)

You can install both with the following:

```
$ curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```

See [Installing Rust](http://doc.rust-lang.org/guide.html#installing-rust) for further details.

## Usage

```
$ cargo build       ## build library and binary
$ cargo test        ## run tests in ./tests
$ cargo bench       ## run benchmarks in ./benches
```

## Discussion

There is an IRC channel on [freenode](https://freenode.net) (chat.freenode.net) at [#epsilonz](http://webchat.freenode.net/?channels=%23epsilonz).
