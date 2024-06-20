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

### Example of surreal.rs Usage
```rust
fn main() {
      let zero = construct("{ | }"); // creates a surreal number with L = NONE, R = NONE (value = 0)
      let num1 = construct("{ 1 | }"); // creates a surreal with L = 1, R = NONE (value = 2)
      let num2 = construct("{ 1, 2 | 0 }"); // creates a surreal with L = 1 and 2, and R = 0
      let neg_num1 = negate(&num1); // negates num1 resulting in { | -1 }
      let num1_plus_zero = add(&num1, &zero); // adds num1 and zero resulting in { 1 | }
      print(&neg_num1); // prints { | -1 }

      let alt_zero = Surreal {
            l: vec![].into(),
            r: vec![].into(),
      }; 
      
      let alt_num2 = Surreal {
            l: vec![1, 2].into(),
            r: vec![0].into(),
      };
      // alternative way to define surreal
      
      let also_alt_zero = zero(); // alternative function to define zero
      let s: Surreal = star(1); // creates a surreal with { 0 | 0 } 
      let s2 = astar(2, 3); // creates a surreal with value of *2 + *3 = *1
                             // its value is { 0 | 0 }
}
```
