# chrysanthemum: a simple type system

## todo

- [x] the simple lambda calculus: implement `execute`
- [ ] to lose my sanity: implement `parse`
- [ ] bidirectional typechecking: implement `infer` and `check`
- [ ] simple effects: extend `ast`
- [ ] type classes: implement `monomorphize`
- [ ] to be fancy: implement `parse_file`
- [ ] extend to additional basic types: implement `cast`
- [ ] extend to complex types
- [ ] testtesttest

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
