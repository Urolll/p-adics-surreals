# p-adics-surreals
Computations of p-adic Numbers and Surreal Numbers in Parallel

## What are Surreal Numbers?
The surreal number is a set of numbers that contains different sizes of 
infinities as well as expressing game states in combinatorial game theory. One common 
way to express surreal numbers is using John Conway's construction notation. 
We put up braces recursively with surreals on the left and right to denote a surreal number.  

L denotes the possible moves for the "blue" player and R denotes the 
possible moves for the "red" player. If it leads to a positive value when converting the surreal 
form to the real form, then the "blue" player wins with optimal play. If it is 
negative, the "red" player wins with optimal play. If it is 0, it is a win for the second player.  

In general, we have the form S = { L | R }, where L and R are either numbers or another surreal. 
The number zero is denoted by the empty surreal, 0 = { | }. It is worth noting that certain numbers in 
surreals do not have real counterparts. Here are examples of surreal numbers:  

1 = { 0 | }

2 = { 1 | }

-1 = { | 0 }

-2 = { | -1 }

1/2 = { 0 | 1 }

star = { 0 | 0 }

star(2) = { 0, star | 0, star } = { 0, { 0 | 0 } | 0, { 0 | 0 } }

## surreal.rs
- [x] pub struct Surreal { pub l: Option<Vec<SurrealValue>>, pub r: Option<Vec<SurrealValue>>, }

- [x] pub enum SurrealValue { Integer(i32), Surreal(Surreal), }  

- [x] pub struct PStar { pub l: Vec<StarValue>, pub r: Vec<StarValue>, }  

- [x] pub enum StarValue { String(String), Integer(i32), }  

- [x] fn construct(num: &str) -> Surreal  
      // takes a string of the form { L | R } and parse it to create a struct of surreal

- [x] fn print(n: &surreal)  
      // prints a surreal number

- [x] fn negate(n: &Surreal) -> Surreal  
      // returns the negate of a surreal number

- [x] fn append(surreal: &mut Surreal, value: SurrealValue, to_left: bool)  
      // takes a mutable reference to a surreal number  
      // appends a surreal value to the left or right field of it

- [x] fn gt(n1: &Surreal, n2: &Surreal) -> Boolean  
      // returns True if n1 is greater than n2, False otherwise

- [x] fn lt(n1: &Surreal, n2: &Surreal) -> Boolean  
      // returns True if n1 is less than n2, False otherwise

- [x] fn ge(n1: &Surreal, n2: &Surreal) -> Boolean  
      // returns True if n1 is greater than or equal to n2, False otherwise

- [x] fn le(n1: &Surreal, n2: &Surreal) -> Boolean  
      // returns True if n1 is less than or equal to n2, False otherwise

- [x] fn eq(n1: &Surreal, n2: &Surreal) -> Boolean  
      // returns True if n1 is equal to n2, False otherwise  
      // this checks for exact equivalence, not numerical

- [x] fn compare(n1: &Surreal, n2: &Surreal, )  
      // pass a comparator function in and return the value that satisfies

- [x] fn add(n1: &Surreal, n2: &Surreal) -> Surreal  
      // adds two surreal numbers together
      // only defined for simple surreals

- [x] fn pdt_add(n1: &Surreal, x: i32, n2: &Surreal, y: i32) -> Surreal  
      // adds two surreal numbers together given predetermined values of n1 and n2

- [x] fn zero() -> Surreal  
      // returns zero, which is { | }

- [x] fn star(n: i32) -> Surreal  
      // returns a multiple of n stars  
      // star is defined as { 0 | 0 }

- [x] fn astar(n1: i32, n2: i32) -> Surreal  
      // returns a multiple of stars from doing n1 + n2  
      // adding stars is equivalent to xor  

- [x] fn pretty_star(n: i32) -> PStar  
      // returns a multiple of n stars readily pretty printed

- [x] fn pretty_astar(n1: i32, n2: i32) -> PStar  
      // returns a multiples of stars from doing n1 + n2 readily pretty printed

- [x] fn print_pstar(star: PStar)  
      // takes a pretty printed star and prints to the console

- [x] fn expand_pstar(star: PStar) -> Surreal  
      // converts a pretty printed star to its surreal counterpart      

### Example of surreal.rs Usage
```rust
mod pstar;
mod surreal;
use crate::pstar::*;
use crate::surreal::*;

fn main() {
    let x = construct("{ 1.0, 2.0 | }");
    let y = construct("{ 2.0, 3.0, 4.0 | 9.0, 2.0 }");
    let zero = zero();
    let alt_zero = Surreal {
        l: Some(vec![]),
        r: Some(vec![]),
    };
    let k: Surreal = Surreal {
        l: Some(vec![
            SurrealValue::Float(1.0),
            SurrealValue::Float(2.0),
            SurrealValue::Float(3.0),
        ]),
        r: Some(vec![]),
    };

    let neg_x = negate(&x);
    print(&x);
    print(&neg_x);

    let neg_y = negate(&y);
    print(&y);
    print(&neg_y);

    print(&alt_zero);
    print(&negate(&zero));

    print(&negate(&k));

    let nested = construct("{ 1.0 | { 0.0 | { -1.0 | } } }");
    print(&nested);
    print(&negate(&nested));

    let star_1 = star(1);
    print(&star_1);
    let star_2 = star(2);
    print(&star_2);
    let star_3 = star(3);
    print(&star_3);
    let star = astar(2, 3);
    print(&negate(&star));

    let pstar = pretty_star(2);
    let _pastar = pretty_astar(2, 3);
    print_pstar(&pstar);
    print(&expand_pstar(pstar));
    print_pstar(&pretty_star(3));
    println!("{}", add(&construct("{1.0 | }"), &construct("{ | -2.0}")));
    println!(
        "{}",
        pdt_add(
            &construct("{1.0, 2.0, 3.0, 4.0, 5.0 | }"),
            6.0,
            &construct("{ | -2.0}"),
            -3.0
        )
    );
    let half = construct("{0 | 1}");
    println!("{}", pdt_add(&half, 0.5, &half, 0.5));
}
```

## What are P-adic Numbers?
A p-adic number is an infinitely long number going leftwards in base 'p', a prime number. 
Think of a number like ...999999, with infinitely many 9s, if we add 1 to this number, we get 
a bunch of ...00000. So, ...999 + 1 = ...000 meaning that ...999 equals -1. 
That was a number in base 10, which is not necessarily p-adic, because 10 is not a prime number. 
However, we can expand this intuition to any bases we want. 

## p-adic.rs
- [x] pub struct Padic { pub v: Frac, pub p: i64, pub expanded: Vec<i64>, }  

- [x] pub struct Frac { pub numer: i64, pub denom: i64, }

- [x] pub fn expand(f: &Frac, p: i64, precision: usize)  
      // expands a fraction f with adic base 'p' with specified precision  

- [x] pub fn print_p_adic(padic: &Padic)

- [x] pub fn add_p_adic(p1: &Padic, p2: &Padic) -> Padic  
      // adds 2 padic numbers together, returning a new padic number

- [x] pub fn add(&self, other: &Frac) -> Frac  
      // adds 2 fractions together, returning a new fraction

- [x] pub fn print_as_frac(p: &Padic)  
      // prints the fractional value of a p-adic number

### Example of p-adic.rs Usage
```rust 
mod frac;
mod padic;
use crate::frac::*;
use crate::padic::*;

fn main() {
    let frac = Frac {
        numer: 128,
        denom: 9,
    };
    let p = 7;
    let padic = expand(&frac, p, 10);
    println!("{:?}", padic);
    print_p_adic(&padic);

    let doubled = add_p_adic(&padic, &padic);
    println!("{:?}", doubled);
    print_p_adic(&doubled);
    print_as_frac(&doubled);
}
```
