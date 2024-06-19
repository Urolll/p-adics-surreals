# p-adics-surreals
Computations of p-adic Numbers and Surreal Numbers in Parallel

## surreal.rs
- [x] fn construct(num: &str) -> Surreal
      // takes a string of the form { L | R } and parse it to create a struct of surreal

- [x] fn print(n: &surreal)
      // prints a surreal number

- [x] fn negate(n: &Surreal) -> Surreal
      // returns the negate of a surreal number

- [ ] fn gt(n1: &Surreal, n2: &Surreal) -> Boolean
      // returns True if n1 is greater than n2, False otherwise

- [ ] fn lt(n1: &Surreal, n2: &Surreal) -> Boolean
      // returns True if n1 is less than n2, False otherwise

- [ ] fn ge(n1: &Surreal, n2: &Surreal) -> Boolean
      // returns True if n1 is greater than or equal to n2, False otherwise

- [ ] fn le(n1: &Surreal, n2: &Surreal) -> Boolean
      // returns True if n1 is less than or equal to n2, False otherwise

- [ ] fn eq(n1: &Surreal, n2: &Surreal) -> Boolean
      // returns True if n1 is equal to n2, False otherwise

- [ ] fn compare(n1: &Surreal, n2: &Surreal, )
      // pass a comparator function in and return the value that satisfies

- [ ] fn add(n1: &Surreal, n2: &Surreal) -> Surreal
      // adds two surreal numbers together

- [ ] fn sub(n1: &Surreal, n2: &Surreal) -> Surreal
      // subtracts two surreal numbers together

- [ ] fn mul(n1: &Surreal, n2: &Surreal) -> Surreal
      // multiplies two surreal numbers together

- [ ] fn div(n1: &Surreal, n2: &Surreal) -> Surreal
      // divides two surreal numbers together

- [ ] fn zero() -> Surreal
      // returns zero, which is { | }

- [ ] fn star(n: i32) -> Surreal
      // returns a multiple of n stars
      // star is defined as { 0 | 0 }

- [ ] fn astar(n1: i32, n2: i32) -> Surreal
      // returns a multiple of stars from doing n1 + n2
      // adding stars is equivalent to xor
