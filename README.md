# chrysanthemum: a simple type system

## todo

- [x] the simple lambda calculus: implement `execute`
- [x] to be fancy: implement `parse`
- [x] to lose my sanity: implement `parse_file`
- [x] bidirectional typechecking: implement `infer` and `check`
- [x] extend to additional basic types: refactor `Term`
- [ ] extend to complex types: implement `access`
- [ ] simple effects: extend `ast`
- [ ] type classes: implement `monomorphize`
- [x] testtesttest

## architecture

```bash
src/
src/main.rs # the user facing program
src/parser.rs # parses user programs into proper data structures
src/ast.rs # the fundamental representation of the program
src/simple.rs # the core of the lambda calculus: checking, inference, evaluation
src/effects.rs # code for effects idk
src/classes.rs # a monomorphization pass for type classes
test/ # various tests
```
