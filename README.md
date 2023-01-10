# Simple Regex in Rust

Basic regex engine in rust for educational purposes.
Mainly to explore building libraries in rust and testing.
Not close to an efficient implmentation, nor trying to be.

Inspired by [this article](https://www.cs.princeton.edu/courses/archive/spr09/cos333/beautiful.html).

[Regex Wikipedia](https://en.wikipedia.org/wiki/Regular_expression)

## Current Features
- compile(regex): compile to set of base instructions that evaulate to a bool
	* **NoOp** -> *always true*
	* **Cmp(char)** -> *true when char matches src char else false*
	* **AtLeast(num)** -> *true when src char occurs AtLeast num times*
	* **AtMost(num)** -> (rename?) *always true, but only matches src char at most num times*
	* **Final** -> *always true, signals successful run*
- '*', '+', '?': compiles to AtLeast(0), AtLeast(1), AtMost(1) respectively
- '.': compiles to NoOp
- c (any other character): compiles to Cmp(c)
- contains_match(): does source string contain the pattern
- replace(): replace first match with tgt string

## TODO
- [X] implement basic regex compiler
- [X] implement basic regex execute
- [ ] add more features
	- [ ] start and end of lines
	- [ ] negates
	- [ ] classes & groups
