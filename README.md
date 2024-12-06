# Kust

_**Write Rust like you never left Kotlin.**_

Kust aims to make new Rustaceans, familiar with Kotlin, feel right at home. It's also aimed to bring a few of Kotlin's goodies to the existing Rust community, without compromising on any of Rust's benefits.

## Scope functions

Perform side effects and modify objects in a more concise and readable way.

Available functions:

- `using` - use a value in an expression without creating a new variable (returns a new value)
- `also` - perform a side effect using a value without modifying it (returns the same value)
- `apply` - modify a value before returning it (returns the same value)

> [!NOTE]
> The Kotlin `let` function has been renamed to `using`, as the `let` keyword is reserved in Rust.

> See [Kotlin documentation](https://kotlinlang.org/docs/scope-functions.html).

### Examples

**Using**

```rust
use some::Person;
use kust::ScopeFunctions;

fn greet_person(encoded_person: &str) -> String {
    let name = Person::from_str(encoded_person).using(|person| format!("{} {}", person.first_name, person.last_name));
    println!("Hi, {name}!")
}
```

**Also**

```rust
use some::ComplexType;
use kust::ScopeFunctions;

fn parse_items(&items: Vec<&str>) -> Vec<ComplexType> {
    items
        // easily add a debug print for each item
        .map(|item| ComplexType::from_str(item).also(|it| println!("{it}")))
        .filter(|item| !item.is_empty())
        .collect()
}
```

**Apply**

```rust
use some::Person;
use kust::ScopeFunctions;

fn get_users() -> Vec<Person> {
    vec![
        // set an additional attribute
        Person::create("Mike").apply(|it| it.age = 34),
        Person::create("Linda").apply(|it| it.age = 25),
    ]
}
```
