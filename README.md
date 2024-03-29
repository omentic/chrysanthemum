# chrysanthemum

chrysanthemum is a simple language with a complex type system, initially written as a term project for CPSC 539.
It implements a number of features from the excellent *Types and Programming Languages*, including:
- The simply typed lambda calculus
- Bidirectional type checking and subtyping support
- A somewhat complex type system: including support for:
  - `unit`, `bool`, `int`, `nat`, `float`, `str`,
  - `struct`, `tuple`, `union`, `list`, `array`, `slice`,
  - `interface`, `empty`, `error`

## todo

- [x] the simple lambda calculus: implement `execute`
- [x] to be fancy: implement `parse`
- [x] to lose my sanity: implement `parse_file`
- [x] bidirectional typechecking: implement `infer` and `check`
- [x] extend to additional basic types: refactor `Term`
- [x] extend to complex types: improve `subtype`
- [x] meet my original standards: implement `interface`
- [ ] make complex types useful: implement `access`
- [ ] type classes: implement `monomorphize`
- [ ] simple effects: extend `ast`
- [x] testtesttest

## architecture

```bash
src/
src/main.rs    # the user facing program
src/simple.rs  # the simple lambda calculus: execution
src/ast.rs     # the fundamental representation of types and terms
src/bidirectional.rs # the core of the language: checking, inference
src/unification.rs   # an alternate core: checking and inference by unification
src/parser.rs        # parses user programs into proper data structures
src/monomorphize.rs  # a monomorphization pass for type classes
src/effects.rs       # code for effects idk
test/ # various tests
```

## bibliography

- [Types and Programming Languages](https://www.cis.upenn.edu/~bcpierce/tapl/)
- [Advanced Topics in Types and Programming Languages](https://www.cis.upenn.edu/~bcpierce/attapl/)
- [Bidirectional Typing Rules: A Tutorial](https://www.davidchristiansen.dk/tutorials/bidirectional.pdf)
- [Bidirectional Typechecking](https://research.cs.queensu.ca/home/jana/bitype.pdf)
- [Bidirectional Type Class Instances](https://arxiv.org/pdf/1906.12242.pdf)
- [Typechecking for Higher-Rank Polymorphism](https://arxiv.org/pdf/1306.6032.pdf)
- [How to make ad-hoc polymorphism less ad-hoc](https://dl.acm.org/doi/pdf/10.1145/75277.75283)
