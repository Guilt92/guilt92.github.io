# Rust Traits and Generics Deep Dive

Mastering trait objects, static dispatch, associated types, and advanced generic programming patterns in Rust.

## Traits: The Foundation of Polymorphism

Traits define shared behavior. They're Rust's equivalent of interfaces, but more powerful:

```rust
trait Drawable {
    fn draw(&self, canvas: &mut Canvas);

    fn bounding_box(&self) -> Rect;

    // Default implementation
    fn is_visible(&self) -> bool {
        true
    }
}

struct Circle { x: f64, y: f64, radius: f64 }
struct Rect { x: f64, y: f64, width: f64, height: f64 }

impl Drawable for Circle {
    fn draw(&self, canvas: &mut Canvas) {
        canvas.draw_circle(self.x, self.y, self.radius);
    }

    fn bounding_box(&self) -> Rect {
        Rect {
            x: self.x - self.radius,
            y: self.y - self.radius,
            width: self.radius * 2.0,
            height: self.radius * 2.0,
        }
    }
}
```

## Static vs Dynamic Dispatch

### Static Dispatch (Generics)

The compiler generates specialized code for each concrete type — zero overhead:

```rust
fn draw_all<T: Drawable>(items: &[T], canvas: &mut Canvas) {
    for item in items {
        item.draw(canvas);
    }
}

// Compiled as:
// fn draw_all_circle(items: &[Circle], canvas: &mut Canvas) { ... }
// fn draw_all_rect(items: &[Rect], canvas: &mut Canvas) { ... }
```

### Dynamic Dispatch (Trait Objects)

Uses a vtable for runtime polymorphism — slight indirection cost:

```rust
fn draw_all(items: &[&dyn Drawable], canvas: &mut Canvas) {
    for item in items {
        item.draw(canvas);
    }
}
```

### When to Use Which

| Criteria | Static Dispatch | Dynamic Dispatch |
|----------|----------------|------------------|
| Performance | Zero overhead | Vtable induction |
| Binary size | Larger (monomorphization) | Smaller |
| Compile time | Slower | Faster |
| Heterogeneous collections | Not possible | Possible |

## Associated Types

Associated types fix the output type of a trait implementation:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    current: u32,
    max: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}
```

## Trait Bounds and Where Clauses

```rust
// Simple bound
fn serialize<T: Serialize>(value: &T) -> String { ... }

// Multiple bounds
fn process<T: Display + Debug + Clone>(item: &T) { ... }

// Where clause (cleaner for complex bounds)
fn render<T>(item: &T) -> String
where
    T: Display + Debug + Clone + HasBounds,
{
    format!("{}", item)
}
```

## Advanced: Trait Objects with Lifetime

```rust
trait Processor {
    fn process(&self, data: &[u8]) -> Vec<u8>;
}

fn make_processor<'a>(kind: &str) -> Box<dyn Processor + 'a> {
    match kind {
        "compress" => Box::new(Compressor::new()),
        "encrypt" => Box::new(Encryptor::new()),
        _ => panic!("Unknown processor"),
    }
}
```

## Blanket Implementations

The standard library implements traits for types automatically:

```rust
// Every type that implements Display also implements ToString
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        // ...
    }
}

// This is why you can call .to_string() on any Display type
let s = 42.to_string(); // Works because i32 implements Display
```

## Conclusion

Traits and generics are Rust's most powerful abstraction tools. Master them, and you can write code that's both generic and zero-cost — the best of both worlds.
