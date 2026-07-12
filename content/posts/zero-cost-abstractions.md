# Zero-Cost Abstractions in Practice

Understanding what "zero-cost" really means in Rust, with assembly analysis and benchmarking techniques.

## The Promise

Rust's zero-cost abstractions mean you don't pay a runtime performance penalty for using high-level constructs. The compiler optimizes away abstractions so the final binary is as fast as hand-written low-level code.

> "What you don't use, you don't pay for. And further: What you do use, you couldn't hand code any better." — Bjarne Stroustrup

## Iterators vs Loops

A common question: are iterators as fast as traditional loops?

```rust
// Using iterators
let sum: i64 = (0..1_000_000)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .sum();

// Equivalent imperative loop
let mut sum: i64 = 0;
for i in 0..1_000_000 {
    if i % 2 == 0 {
        sum += i * i;
    }
}
```

Both produce identical assembly at `-O2`. The iterator version is **zero-cost** — the compiler inlines everything and generates the same machine code.

## Generics vs Trait Objects

Generics use **monomorphization** — the compiler generates specialized code for each concrete type:

```rust
// Static dispatch (monomorphized) — zero overhead
fn process<T: Render>(item: T) -> String {
    item.render()
}

// Dynamic dispatch (vtable lookup) — slight overhead
fn process(item: &dyn Render) -> String {
    item.render()
}
```

Use generics when performance is critical. Use trait objects when you need runtime polymorphism and the vtable indirection is negligible.

## Benchmarking

Use `criterion` for reliable benchmarks:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_parser(c: &mut Criterion) {
    c.bench_function("parse markdown", |b| {
        b.iter(|| parse_markdown(black_box(SAMPLE_INPUT)))
    });
}

criterion_group!(benches, benchmark_parser);
criterion_main!(benches);
```

## What "Zero-Cost" Doesn't Mean

- It doesn't mean zero **compile time** cost — generics monomorphization increases compile time
- It doesn't mean zero **binary size** cost — each monomorphized function is duplicated
- It doesn't mean you can't write slow code — algorithmic choices still matter

## Conclusion

Zero-cost abstractions let you write expressive, high-level Rust without sacrificing performance. The compiler is your partner — trust it, measure, and only optimize when benchmarks tell you to.
