# fake nim

func fib(x): int -> int =
  if eq(x, 0):
    0
  else:
    if eq(x, 1):
      1
    else:       # comment
      add(fib(sub(x, 1)), fib(sub(x, 2)))

negate(negate(1))
fib(5)
