# Rust tips

Source: https://www.reddit.com/r/rust/comments/1d28yqe/what_are_some_rust_practices_you_learned_in_a/

## Format

Format your code by using:

```sh
cargo fmt --check
```

Proofread your code by using:

```sh
cargo clippy --D clippy::pedantic
```

Consider using nursery... and maybe with +beta.

Ideally require these steps for all code check ins.

Format your documentation by using:

```sh
cargo doc
```


## Dependencies

Audit your dependencies for security issues.

One way is:

```sh
cargo install cargo-audit
cargo audit
```

Update your dependencies often. 

One way is:

```sh
cargo install cargo-edit
cargo install cargo-upgrades
cargo upgrade
```

This is because it gets difficult to update dependencies once a cargo audit spits out an issue and you must move up multiple semantic versions.

Ideally require these steps for all code check ins.


## Use macros sparingly

Don't use macros unless absolutely necessary.

If you can use a function rather than a macro use the function. 

This is because macros are essentially a different language, and LSPs have a hard time parsing them (usually they simply cannot parse them). They are absolute HELL for maintainability or for newcomers to the codebase.


## Use let

Use `let Some()` and `let Ok()` with or without `if` as required, instead of using `match` for binary expressions (unless you truly can't). Match is awesome, but it's ugly compared 

Examples:

```rust
let Some(x) = from_something else { return };
let Some(x) = from_something else { break };
```


## Errors

Bubble your errors and log at appropriate spots rather than immediately logging errors and returning. 

For example, have functions that return `Result<(), errortype>` and log errors in the caller rather than returning `()` and logging each error inside of the callee and returning `()`.


## Map

Use `.map()` and `.filter_map()` because they improve code readability.

Use `.inspect_err()` if you must log an error immediately rather than bubbling it up.


## Premature optimization

Write your feature the easy way, use clones and lots of mut. Then refine it.

It's better to be readable than fast. The compiler will usually optimize your code so that it's good enough. If you are in a hot path, then and only then optimize.

More optimized code is often more readable for more advanced developers. For example you might think that you should lock a Vec you have inside of a mutex and have a for loop pushing things into it one by one, but having a `.iter().map(...).collect::<Vec<_>>();` and then using `.append()` on your mutexed Vec only after you've built up the new values is more readable and faster. 


## Use the compiler

If you're fighting the compiler, you're doing something wrong. Rust will fight you if you are doing something wrong. Sometimes you'll find yourself fighting the compiler in more ways than just fighting the borrow checker. 


## Use From/Into

Use the `From` trait and the `.into()` method:

```rust
impl From<X> for <Y>... .into()
```

## Use traits for strings 

You often want your string typed parameters to be `impl Into<String>` rather than `&str` that you eventually call `.to_string()` on, or passing in a whole `String`.


## Don't unwrap

Replace `.unwrap()` with `.expect()` or `#[allow(clippy::used_unwrap)]`, especially in production code.

As a code reviewer I cannot tell whether you actually intend the thread to crash if you have a `None/Err`, or if you simply forgot to handle the error properly. 

When you use `.expect()` then you tell me "yes I mean this and here is why". Or if `.expect()` is too verbose and takes away from readability then use `#[allow(clippy::unwrap_used)]` which tells me "yes I meant this, and the error is either way too verbose for .expect() or way too obvious to be worth mentioning (e.g. a lock is poisoned)".


## Don't expect

I never use `.expect()` in production code. If I saw it in code review, I'd think the author hasn't figured out what kind of error he's actually dealing with.

Is it a violation of an invariant that couldn't be conveyed to the compiler and therefore needs an .unwrap()? Then .unwrap() should be used, signaling to the reader that validity of the condition is assured by other means. Add a comment if it isn't obvious.

Is it an error that can happen during normal operation of the program? Handle it as such, with ? and so on.

Is it an irrecoverable error that prevents the program from continuing? Then panic!() explicitly, which not only gives you a better message than "expectation failed" but also allows to include other pieces of information for more context when debugging.

Using `.expect()` is a half-measure that makes you feel good that "you didn't just .unwrap()!" but is usually a sign you haven't really thought things through. 


## Use SAFETY

Use `// SAFETY:` comments for unsafe and wherever you as cast there are a few other places where I want to be assured you did due diligence outside of unsafe and as cast as well, but they are not coming to mind.


## Help the code reviewer

Keep the code reviewer in mind when you write your code.

Help the code reviewer by using `cargo fmt --check` and `cargo clippy --D clippy::pedantic` and `// Safety` and other conventions, so the code reviewer can focus on your semantics and not your syntax.


## Favor locality


There is a balance between locality of code that is really difficult to maintain in Rust (it's harder than C++). Do your best. 

Macros that are far away from where they are called are really bad for this.


## Proptest

https://proptest-rs.github.io/proptest/intro.html

https://docs.rs/proptest/


## Hide complexity

Avoid leaking complex features of Rust across your entire codebase. For example, features such as higher-rank trait bounds, non-trivial lifetimes, and anything else you would find in the Rustonomicon generally belong in library code with crisp interfaces.

A "leaky" interface that caused me and my team pain was using trait bounds as extensions:

* Using different impl's impl <T, U, V> Collection for Foo<T, U, V> where T: .... with varying bounds is a cool rust feature, but has mostly just resulted in more people wasting time with "Method exists but trait bounds not satisfied" errors in practice. Creating some multi generic trait bounded abominations is already bad enough. Forcing the end user to decipher it in error messages is exactly how you make people hate rusts type system.

A "crisp" interface I found successful was to encapsulate memory management structs:

* You can encapsulate Arc, Rc, Cell, Mutex, etc in structs quite easily and use the guards to handle advanced references. This allows the implementer to start out with reckless clone()ing, then choose exactly how best to work around the borrow checker later. This is especially useful for setting up good code reviews for the people still learning the rust memory management structs.

Keep unsafe & FFIs in leaf modules:

* If you plan on shipping production code in rust, there is a good chance you will take on a C/C++ dependencies. Miri is the main tool for finding UB in unsafe rust, which don't play nicely yet with FFIs. You will want to keep them in disjoint modules to use testing tools independently. You want them in the "leaf nodes" of your call stack and not weaving betweenSafe-Rust -> C -> Unsafe-Rust etc. Non-leaves should be safe rust.


## Use newtypes


Use newtypes. It's helped a lot in not mixing up different integer-valued things that look the same, but aren't. 


We also stick very much to the Not Rocket Science school of thought: https://graydon2.dreamwidth.org/1597.html - that means:

    Create enforceable conventions.

    Enforce them in build tools/CI/CD.

We have written custom lints to check for:

    Alphabetic ordering of items in source files. (It's not worth discussing order, and defining a convention makes it obvious).

    Usage of unwraps must be commented.

    Non-usage of results (other than Result) must be commented.

    Usage of `use` imports must be commented. (Our convention is to always use full paths).

    And some other.


## Use the type system

Rust's type system is very effective at constraining data to valid values, and simplifying the downstream code. 

A lot of my work involves parsing and data conversion, and I find the struct + enum building blocks to be very powerful for modelling the input and output data. Then conversion often becomes a large set of TryFrom or From trait implementations. For anything that can only be validated at run-time, I use private struct members and constructors. This significantly reduces the test space, as well as allowing me to leverage validity assumptions further down the data pipeline, often making code simpler.


## Use error handling

Error handling seems really important but can easily get in the way. I've had mixed results across my projects so far. I've found Jeremy Chone's "Rust Error Handling - Best Practices" pretty useful though, albeit recent.

In particular, I've had to dig myself out of an anyhow hole a few times now - mostly gross overuse of bail!() and anyhow!() for ad-hoc errors. I think the lesson I've learned is that using structured errors (e.g. via enum) is often worth doing from the start, even for application code. They also make the unit tests much nicer to write - no more string matching!

I think there's probably a nice balance using structured enums with anyhow, I just haven't quite found it yet. Maybe using the anyhow::Result type with structured enums, but avoiding bail!/anyhow!. I feel like I'm getting closer.


## Try thiserror

Using `thiserror` even in application code is quick and easy, and it makes it easier to potentially extract parts of it into libraries shared between different projects and teams.


## Use snapshot testing

Snapshot testing (I've used `insta`) is a great accelerator. It makes the experience of writing and updating tests much more streamlined. I wish I had tried it earlier. I'm now slowly working it back into ecosystem crates.

Great for libs, I often couple it with a feature for serde. It’s fantastic here. 


## Try winnow

Try winnow for writing parsers.


## Try JetBrains

I'm using JetBrains IDEA + Rust plugin with CoPilot. This AI does a really good job of catching onto the boilerplate and replicating it, or helping fill out standard trait implementations or enhancing my IDE's auto-complete. It has saved me a lot of time, although sometimes it messes up, but I find that's easy to detect and fix. I find ChatGPT 4 much better at answering the higher-level questions, though, especially the kind like "I have this but I want this" or "how do I do this" queries.


## Read Rust for Rustaceans

I think "Rust for Rustaceans" is a good read, and the author's YouTube videos are gold. 


###### TODO

Putting in that extra effort to make strong guarantees about your code is much easier in Rust than in pretty much any other equivalently useful language (I'm sure the pure functional languages are even better, but I'm now moving a company's SPA to Haskell). Being able to encode that "This function can only fail in one of 3 ways" in a Result with a finite enum of error cases massively helps with onboarding people on your codebase. The more information you can cleanly present to your team, the better.

On that, spend the time to write good docstrings, ideally with examples as well! There's been a few times where my Python-favouring team could just write entire apps using Rust based on how the in-IDE examples showed them how to compose functions and data. Plus, these examples can be unit tested as a part of Cargo, which really helps with your own development as well.

Lastly, really lean into Cargo. While the default lints and formatting don't match what the rest of my org is used to, the simplicity of everyone having the same setup is worth that initial shock of code being formatted differently.

One bonus thing: it depends on your work, but showing how to have your Rust codebase be used in JS (wasm-bindgen), Python (pyo3), C/++ (extern "C") etc. helps. I've managed to win over a lot of my students/colleagues by simply showing them they can get the benefits of Rust in just a small segment of their codebase, rather than uprooting the whole thing. Most applications need parsers, and Rust is really good for it.



## Foster a culture of productivity


Foster a culture of productivity, not technical cleverness.

Rust is a very complex language with powerful concepts, and people are often tempted to use latest and most advanced features to solve certain problems. Sometimes out of curiosity, sometimes to strive for "elegance". Many times, this comes with strong opinions. But this is often unproductive and raises the bar for less experienced developers to contribute.

To get things done, it's often helpful to allow for imperfection. And by that I don't mean "use unsafe everywhere", but rather things like:

* Clone instead of complex lifetimes

* `dyn Trait` is completely fine, it's rare enough that it's allowed

* Traits like `Borrow`, `AsRef`, `ToOwned`, etc. are primarily intended for libraries, not everyday code

* Don't create a trait just because two APIs are similar, make sure you need the polymorphism and the overhead is worth it. Good example, std's `Vec`, `VecDeque`, and `LinkedList` have very similar interface but barely share traits.

* If it's hard to come up with a data model that statically only allows valid state, use runtime checks (e.g. during construction). This is often better than a deeply intertangled abstraction, because complexity is also harder to validate for correctness.

* If you can use threads instead of `async`, do it. It's so much nicer.

* You need `Rc<RefCell<T>>` a lot? Everyone tells you you're doing things wrong, but it's often more productive than alternatives, especially when prototyping. Embrace it by making the pattern more ergonomic.

Any successful business has technical debt in some way. While Rust isn't a language that makes fast prototyping (hacking) things easy, there are definitely ways, and refactorings typically have lower risk to introduce problems than e.g. dynamically typed languages.

There's of course a balance to everything. Part of engineering is to know when it's important to build super-robust, "clean" solutions, and when trade-offs for faster delivery are acceptable. 


## Use multiple crates


Separating a project into multiple crates is healthy, ergonomic, and can speed up compile times. However, too much isolation can lead to issues where you end up either forcefully needing to use crates in a way that depend on each other to implement a trait - or you end up using wrapper types.

If your symptoms are like I mentioned, you’ve invested too much into concrete types and not enough into traits and implementations (interfaces) . Invest in decoupling. This isn’t specific to rust, but a common pitfall I see because it’s “easier to reason with” during early stages. 


## Define From

One very practical thing I enjoy is defining From<T>s against my own app's config files. You're bound to accumulate a bunch of 3rd party crates that need to be initialized. Historically I've seen a lot of non-rust code that spaghettis lookups into your config, or puts every possible prop into the arguments of the functions themselves (eg this fn needs the db user and connection string, this one needs the db name and port separately, etc.) But being able to just From<MyConfig> for ThirdPartyConfig gives you an obvious, manageable spot to go for all that. It also isolates many of the breaking changes when you update those libs. 

