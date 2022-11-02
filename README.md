# memoires 🧠

The hardest way to implement memoization in Rust...

## Usage

Lets imagine you have a function that implement Fibonacci sequence:

```rust
fn fib(n: usize) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}


fn main() {
    for i in 1..40 {
        println!("{}", fib(i))
    }
}
```

It gonna be change to:

```rust
use memoires::Memoire;

// The two generics of Memoire<usize, usize> must be change to the types
// your function will return.
//
// If you have a f(String) -> String, you gonna write Memoire<String, String>.
//
// IMPORTANT: the types must implement the Clone, Eq and Hash traits !
//
fn fib<I, O>(n: usize, m: &mut Memoire<usize, usize>) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        m.run(n - 1) + m.run(n - 2) // Replace the function name with m.run
    }
}

fn main() {
    let mut fib_mem = Memoire::new(fib::<isize, isize>);

    for i in 1..40 {
        println!("{}", fib_mem.run(i))
    }
}
```