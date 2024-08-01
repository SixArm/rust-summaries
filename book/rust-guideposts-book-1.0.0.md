---
title: "Rust Guideposts"
author: "By Joel Parker Henderson & ChatGPT"
date: "2023-04-10"
subtitle: "Ecosystem • Language • Concepts • Crates • More"
#geometry: "left=2.54cm,right=2.54cm,top=1.91cm,bottom=1.91cm"
titlepage: true
titlepage-color: "FFFFFF"
titlepage-text-color: "000000"
titlepage-rule-color: "CCCCCC"
titlepage-rule-height: 4
logo: "files/logo.png"
logo-width: 100
page-background:
page-background-opacity:
links-as-notes: false
#lot: true
#lof: true
listings-disable-line-numbers: true
listings-no-page-break: false
disable-header-and-footer: false
header-left:
header-center:
header-right:
footer-left: "© Joel Parker Henderson"
footer-center: "License: CC-BY-NC-SA-4.0"
footer-right:
subparagraph: true
lang: en-US
---

# Introduction

This page is a section divider and intentionally blank.


## What are Rust Guideposts?

Rust Guideposts are quick topic summaries to help people learn about the Rust programming language, ecosystem, concepts, crates, and more.

You can try any topic, in any order, at any time, akin to a frequently asked questions resource.

Link: <https://github.com/sixarm/rust-guideposts>

Rust Guideposts work best as an adjunct to a comprehensive Rust book, such as the The Rust Programming Language.

A comprehensive book is valuable to read, cover to cover, for thorough technical explanations.

Link: <https://doc.rust-lang.org/book/>

We welcome constructive advice, new topic ideas, pull requests, open issues, and the like. See CONTRIBUTING.md. Rust Guideposts are continually evolving, with ongoing additions, corrections, and optimizations.


## What are Rust Guideposts' projects?

<https://github.com/sixarm/rust-guideposts>

Rust Guideposts provides sample projects. These projects are small Rust programs that you can read, build, and run. Each project demonstrates one quick topic summary, or demonstrates one crate. The projects are in the Rust Guideposts repository, in the `projects` directory.

Some of the projects for topics are:

* [from_and_into_traits](https://github.com/SixArm/rust-guideposts/tree/main/projects/topics/from_and_into_traits)
* [closures_for_iterators](https://github.com/SixArm/rust-guideposts/tree/main/projects/topics/closures_for_iterators)
* [test_driven_development](https://github.com/SixArm/rust-guideposts/tree/main/projects/topics/test_driven_development)
* [pass_by_value_or_reference](https://github.com/SixArm/rust-guideposts/tree/main/projects/topics/pass_by_value_or_reference)
* [the_borrow_checker](https://github.com/SixArm/rust-guideposts/tree/main/projects/topics/the_borrow_checker)

Some of the projects for crates are:

* [assertables/values_strings_sets](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/assertables/values_strings_sets)
* [csv/read_a_spreadsheet_file](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/csv/read_a_spreadsheet_file)
* [reqwest/make_http_requests](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/reqwest/make_http_request)
* [serde/parse_json_data](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/serde/parse_json_data)
* [sqlx/create_table_insert_into_select](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/sqlx/create_table_insert_into_select)

Example command to run a project:

```sh
cd projects/topics/hello_world
cargo run
```

Many of the projects include a simple integration test:

```sh
cd projects/topics/hello_world
cargo test
```


## Who is this for?

Rust Guideposts are for anyone who wants to learn about Rust, and who likes to browse quick topic summaries. We're creating it because we are very excited about Rust, and very excited about more people learning Rust.

We're creating these guideposts to help teach students and junior-intermediate developers who are generally familiar with introductory programming concepts and programming languages.

Questions that come up frequently with our students and developers:

* Questions about Rust in context, such as how Rust relates to their own interests, and to other programming languages.

* Questions about Rust language concepts, such as memory management, traits, and futures.

* Questions about Rust in practice, such as how to access a database, what tools do, and which libraries to choose.

These guideposts are trying to answer these kinds of questions quickly and simply. Browse the topics you want, and skip the rest. And if you have questions, you can let us know, because we're continually adding topics and improving these guideposts.

### For Teachers

If you're a teacher, we'd love to hear from you about how you're teaching Rust, how your students are learning, and how we can improve this project for your students. Email joel@joelparkerhenderson.com.


### For Professionals

If you're a professional, we'd love to learn about how you and your company are working with Rust, and how we can improve this project for your coworkers. Email joel@joelparkerhenderson.com.


## How can I contribute?

Rust Guideposts are a free community resource, and we welcome help.

Anyone can edit these guideposts to improve them, such as to edit an existing page, or add a new page, or refactor how the pages are organized. You can do this a variety of ways.

* You can create a GitHub pull request that provides the changes you want.

* You can open a GitHub issue that provides your suggestions, and we can converse about what you want.

* You can contact the maintainer via email at joel@joelparkerhenderson.com.


### Git repository

All the work is available via our GitHub repository.

Link: <https://github.com/sixarm/rust-guideposts>

All the projects are available in the `projects` directory.


### Open source license

If you would like to contribute work, then your work will use the project license.

The project license is Creative Commons Attribution Non Commercial Share Alike 4.0 International (CC-BY-NC-SA-4.0). 

The license is a free libre open source license. We use the license because it enables people to share improvements with everyone.


# Welcome to Rust

This page is a section divider and intentionally blank.


## What is a Rust "Hello, World!" program?

In Rust, a simple "Hello, World!" program is:

```rust
fn main() {
    println!("Hello, World!");
}
```

This program contains a single function, `main()`, which is the entry point for the program. The function body is enclosed in curly braces `{}` and contains a single statement:

```rust
println!("Hello, World!");
```

This statement prints the text "Hello, World!" to the console using Rust's standard library macro `println!()`. The `println!()` macro is a convenient way to print formatted text to the console, and in this case, it simply prints the string literal "Hello, World!".

When you run this program, you should see the text "Hello, World!" printed to the console.

To create this program, the typical way is to use the Rust `cargo` package manager, which can create an example project:

```sh
cargo new hello
cd hello
```

Then edit the `src/main.rs` file, which is automatically created with the "hello world" code above. Change it as you wish.

To run the program:

```sh
cargo run
```

You should see the output:

```txt
Hello, World!
```


## What is a Rust "FizzBuzz" program?

A "FizzBuzz" program is a job interview challenge: print the numbers 1 to 100, except replace any multiple of 3 with "Fizz", any multiple of 5 with "Buzz", and any multiple of both 3 and 5 with "FizzBuzz".

One way to write FizzBuzz:

```rust
for i in 1..=100 {
    if i % 3 == 0 && i % 5 == 0 {
        println!("FizzBuzz");
    } else if i % 3 == 0 {
        println!("Fizz");
    } else if i % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", i);
    }
}
```

The example uses a `for` loop, `if`...`else` control flow statements, `%` modulo operator, `&&` logical operator, and `println!` macros to print output.

FizzBuzz output should start with these lines:

```text
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
```


## What is a Rust "Fibonacci" program?

A "Fibonacci" program is a job interview challenge: given an index number `n`, print its Fibonacci sequence term.

The Fibonacci sequence is a mathematical sequence in which each number is the sum of the two preceding ones: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, etc. For example, index number `0` is the sequence term `0`, and index number `12` is the  sequence term `144`. 

The example below defines a function named `fibonacci`. The function takes an input which is the Fibonacci sequence index number, and returns the Fibonacci sequence term number. For example, input index `0` returns term `0`, and input index  `12` returns term `144`.

Example:

```rust
pub fn fibonacci(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        2.. => fibonacci(n - 1) + fibonacci(n - 2),
        _ => panic!("{}", n)
    }
}

fn main() {
    println!("{}", fibonacci(12));
}
```

Output:

```text
144
```

The "Fibonacci" program is frequently seen in programming for benchmarking, because the calculations and the recursions grow very quickly, and also because there are a variety of ways to optimize the program to run faster, to work without recursions, and to use less memory. For more about this, see the Rust Guideposts page "Memoize a function with cached".



# Learning

This page is a section divider and intentionally blank.


## What makes Rust good?

There are a variety reasons why Rust is considered a good programming language and good for developers.

* Performance: Rust is designed to provide high performance and low-level control, making it an excellent choice for systems programming. Its memory safety guarantees, achieved through its ownership and borrowing system, allow it to be fast without sacrificing safety.

* Safety: Rust's ownership and borrowing system ensures that programs are safe and free from common programming errors such as null pointer dereferences, buffer overflows, and data races. Rust is designed to prevent undefined behavior and make it difficult to write unsafe code.

* Concurrency: Rust's design is well-suited for concurrent and parallel programming. Its ownership and borrowing system make it easier to write code that is thread-safe and can be run in parallel.

* Community: Rust has a large and active community that is dedicated to improving the language and its ecosystem. The community provides excellent documentation, libraries, and tools that make it easier to learn and use Rust.

* Cross-platform support: Rust can be used to write code for a wide range of platforms, including desktop, mobile, and web applications. Rust also has excellent support for compiling to WebAssembly, which makes it possible to write high-performance web applications in Rust.

Overall, Rust's combination of performance, safety, concurrency, community, and cross-platform support make it an excellent programming language for a wide range of applications.

## What is the Rust ecosystem?

The Rust programming language has a growing and vibrant ecosystem that includes a wide range of tools, libraries, and frameworks to support development in various domains. Here are some key components:

* Rust standard library: The Rust standard library provides a set of essential data types and functions that are included in every Rust project.

* Rust tooling: Rust has a growing ecosystem of development tools, including IDEs, code editors, linters, and debuggers.

* Cargo: Cargo is Rust's package manager and build tool. It provides an easy way to manage dependencies, build projects, and publish to the community.

* Rust crates: Rust crates are packages that can be managed by Cargo. The Rust community maintains a large repository of open-source crates, covering a wide range of functionality.

* System programming libraries: Rust is well-suited for system programming, and the language has many libraries to support this, such as low-level C libraries, and ergonomic interface to Unix system calls, and more.

* Web frameworks: Rust has several web development frameworks, including Actix, Rocket, and Warp. These frameworks provide abstractions and tools to build web applications in Rust.

* Embedded development libraries: Rust is increasingly being used for embedded development, and the language has libraries to support this, such as crates for microcontrollers, and libriaries for hardware abstraction layers, and more.

These are just a few examples of the tools and libraries available in the Rust ecosystem. The Rust community is active and collaborative, with ongoing efforts to improve and expand the language's ecosystem.

## Who might benefit from learning Rust?

Rust is a versatile language that can be used in a variety of domains, from systems programming to web development. Here are some groups of people who might benefit from learning Rust:

* Systems programmers: Rust's combination of high performance and memory safety make it an excellent choice for systems programming, including operating systems, device drivers, and low-level network programming.

* Web developers: Rust's ability to compile to WebAssembly and its focus on performance make it a good fit for building high-performance web applications.

* Game developers: Rust's low-level control and performance make it a good choice for game development, particularly for real-time games that require fast processing and efficient memory usage.

* Security researchers: Rust's memory safety guarantees make it an excellent choice for writing secure software and for performing security research.

* Embedded systems developers: Rust's low-level control and efficient memory usage make it a good choice for embedded systems development, including robotics, Internet of Things (IoT) devices, and microcontrollers.

* Developers interested in learning new programming paradigms: Rust's ownership and borrowing system and its emphasis on functional programming concepts such as immutability make it an interesting language to learn for those interested in exploring new programming paradigms.

Overall, Rust can be a good fit for a wide range of developers, depending on their interests and needs.


## What are good ways to learn Rust?

There are a variety of good ways to learn Rust, depending on your learning style and preferences.

Here are a few suggestions:

* Read the official Rust book: The Rust Programming Language is an excellent resource for learning Rust. It covers all the fundamentals of the language, including ownership, borrowing, lifetimes, and more. The book is well-written, easy to follow, and includes plenty of examples and exercises.

* Work through Rust exercises and tutorials: There are several online resources that provide Rust exercises and tutorials, such as Rustlings, Exercism, and Rust by Example. These resources provide hands-on experience with Rust and help you solidify your understanding of the language.

* Join the Rust community: Rust has a vibrant and welcoming community that can provide valuable support and resources as you learn. Joining the Rust community can help you find answers to your questions, connect with other Rust programmers, and stay up-to-date on the latest developments in the language.

* Contribute to open source projects: Contributing to open source projects is a great way to learn Rust and gain experience with real-world projects. You can start by finding a Rust project that interests you and submitting a pull request to fix a bug or add a feature.

* Build your own Rust projects: Building your own Rust projects is a great way to practice your skills and explore the language's features. Start with a simple project, such as a command-line tool, and gradually work your way up to more complex applications.

Whichever approach you choose, learning Rust takes time and effort. With dedication and persistence, you can become a proficient Rust programmer and take advantage of the language's many benefits.


## What are good projects to learn Rust?

Learning Rust can be a rewarding experience, and contributing to open-source projects can be a great way to develop your skills while making meaningful contributions to the community.

* Rustlings is a collection of small exercises designed to help you learn Rust syntax and concepts. It covers topics like ownership, borrowing, and macros, and is a great way to start.

* Rust Game Development Working Group is a community of programmers working on game libraries and tools. This group can be a great way to learn about Rust's capabilities for gaming and graphics programming.

* Servo is a modern, high-performance browser engine written in Rust. It is a complex project that touches on many different aspects of systems programming, including concurrency, memory management, and performance optimization.

* Tokio is a runtime for writing asynchronous Rust applications. It provides abstractions and tools for writing scalable and efficient network applications. Contributing to it can be a good way to learn about concurrency,  async/await features, and futures.

* RustCrypto provides cryptographic libraries. It includes implementations of cryptographic algorithms, as well as higher-level libraries for building secure systems. This project can be a great way to learn about Rust memory safety, security, and low-level systems programming.

* The Rust language itself is a great way to learn Rust. Dive into the source code. Learning it and contributing to it can be a great way to learn about the language and its internals.

These are just a few examples of the many open-source Rust projects available for learning and contributing. Whatever your interests, there is likely a Rust project out there that can help you develop your skills.

## Is Rust a good first language?

<https://www.reddit.com/r/rust/comments/owmxhr> - paraphrased

Question: Is rust a good first language for a complete beginner? I have a little programming experience and I want to try low-level stuff.


### Opinion 1: your goals

If your goal is to get deep into programming then Rust is a solid choice. It will force you to recognize how things work from low to high level and why they are built the way they are. Unlike other low-level options (like C/C++) Rust also pushes you towards good practices and modern ideas/approaches which is good.

If your goal is to dabble a little then Python is great. It sweeps a lot of inconvenient details under the rug which does make it way easier to use. But the most important part is its wide ecosystem: whatever you do, from web services to games, from machine learning to video processing, there is likely a library that can help you.


### Opinion 2: learning curve

On the one hand, Rust forces you to “program well” from the start, or things simply won’t work. This steers you away from many mistakes you can make in other langues, especially other low level langues. And if you can avoid getting into lifetimes too much, the base language is very nice to work with compared to most older languages.

On the other hand, Rust forces you to think about things that some other languages handle automatically. Additionally, Rust is compiled and has less immediate ability to give you feedback outside of printing to the screen and warnings/errors. Many tools are also in 3rd party libraries, many which are still WIP, so you’ll need to learn more than just the language to do “cooler” stuff.


# Caveats

This page is a section divider and intentionally blank.


## What are the hardest parts of Rust?

While Rust is a powerful programming language with many benefits, it can also have some challenges. Here are some of the hardest parts:

* Ownership and Borrowing: Rust's ownership and borrowing system is one of its most unique and powerful features, but it can also be one of the most challenging to learn. Understanding ownership, borrowing, and lifetimes can take time and practice, especially for those who are used to garbage-collected or reference-counted languages.

* Error messages: While Rust's error messages are known for being helpful and informative, they can also be overwhelming for new users. Rust's borrow checker is very strict, and its error messages can sometimes be difficult to understand, especially when dealing with complex borrowing situations.

* Macros: Rust's macro system is a powerful tool for metaprogramming, but it can also be challenging to use. Macros require a deep understanding of Rust's syntax and type system, and they can be difficult to debug when something goes wrong.

* Syntax: Rust's syntax can be verbose and sometimes difficult to read, especially for those who are used to more concise or expressive languages. This can make it harder to write clean, readable code, especially for beginners.

* Limited ecosystem: While Rust's ecosystem is growing rapidly, it is still relatively small compared to other languages. This can make it harder to find libraries and tools for certain tasks, and it can also make it harder to find experienced Rust developers to work with.


## Rust stable versus Rust nightly

In Rust, there are two main channels of development for the compiler and language: the Rust stable channel and the Rust nightly channel.

The Rust stable channel is the main release channel for Rust, where only stable and well-tested features are included. The goal of this channel is to provide a stable and reliable Rust experience for most users. The stable channel has a predictable release schedule and is recommended for most users.

The Rust nightly channel is a more experimental channel that contains bleeding-edge features that are still under development. The nightly channel is updated more frequently than the stable channel, and it may contain features that are not yet stable or well-tested. The nightly channel is intended for developers who want to experiment with new features, contribute to the Rust project, or provide early feedback on new features.

Some features are only available on the nightly channel, while others are only available on the stable channel. In general, the Rust team works to stabilize features as quickly as possible and move them to the stable channel.

To switch between the stable and nightly channels, you can use the rustup tool.

To switch to the latest stable version of Rust, you can run:

```
rustup default stable
```

To switch to the latest nightly version of Rust, you can run:

```
rustup default nightly
```

Overall, the choice between the stable and nightly channels depends on your needs. If you want a stable and reliable Rust experience, you should use the stable channel. If you want to experiment with new features or contribute to the Rust project, you may want to use the nightly channel.

## What is Rust missing?

While Rust is a powerful and versatile language, there are still some areas where it may be lacking in comparison to other languages. Here are a few things that Rust may be missing:

* Mature ecosystem: Rust is still a relatively new language, and as a result, its ecosystem is still developing. Some libraries or tools may not be as fully-featured or mature as those available in more established languages.

* Slower compilation times: Rust's powerful type system and borrow checker can result in longer compilation times compared to other languages. This can be a drawback for developers who require faster feedback cycles.

* Limited support for garbage collection: Rust does not have built-in support for garbage collection, which can make it more difficult to manage memory in some cases. While Rust's ownership and borrowing system provides safety guarantees and avoids issues such as memory leaks, it can also require more careful management of memory allocation and deallocation.

* Learning curve: Rust has a steep learning curve, especially for developers who are not familiar with low-level systems programming or functional programming concepts. This can make it challenging for developers to become proficient in the language quickly.

* Limited support for some platforms: While Rust has good support for Linux and other popular platforms, support for some niche platforms or hardware may be limited. This can be a concern for developers working on specialized projects that require support for these platforms.

It's worth noting that many of these limitations are being actively addressed by the Rust community, and the language continues to evolve and improve over time.


## What are some non-goals of Rust?

These non-goals are listed in a previous official Rust FAQ.

1. We do not employ any particularly cutting-edge technologies. Old, established techniques are better.

2. We do not prize expressiveness, minimalism or elegance above other goals. These are desirable but subordinate goals.

3. We do not intend to cover the complete feature-set of C++, or any other language. Rust should provide majority-case features.

4. We do not intend to be 100% static, 100% safe, 100% reflective, or too dogmatic in any other sense. Trade-offs exist.

5. We do not demand that Rust run on “every possible platform”. It must eventually work without unnecessary compromises on widely-used hardware and software platforms.


## Why do companies not use Rust?

Rust has gained a lot of popularity and adoption in recent years, but some companies are hesitant to adopt the language. Here are some potential reasons why companies may avoid Rust:

* Lack of expertise: Rust is a relatively new language and may not yet have a large pool of experienced developers compared to more established languages. Companies may be hesitant to adopt Rust if they do not have the in-house expertise or resources to adopt Rust.

* Risk aversion: Some companies may be risk-averse and may prefer to stick with more established languages that have a proven track record of success. Rust still a relatively new, and may be perceived as less stable or less reliable compared to more established languages.

* Learning curve: Rust's syntax and concepts can be challenging for developers who are unfamiliar with systems programming or functional programming. Companies may be hesitant to adopt Rust if they anticipate a steep developer learning curve.

* Limited ecosystem: While Rust's ecosystem is growing rapidly, it may not yet have the same level of library support or tooling as more established languages. This can make it more difficult or time-consuming for companies to develop and maintain Rust code.

* Legacy code: Many companies have existing codebases written in other languages, and transitioning to Rust may require significant time and resources. Companies may be hesitant to make this investment if the benefits of transitioning are not clear.

Many of these concerns are actively being addressed by the Rust community, with ongoing efforts to improve the language's ecosystem and make it more accessible to developers of all backgrounds.


# What makes Rust special?

This page is a section divider and intentionally blank.


## The borrow checker

The Rust borrow checker is a tool that ensures memory safety in Rust programs by preventing data races and other forms of undefined behavior related to memory management. In Rust, memory is managed through a system of ownership and borrowing, where ownership represents exclusive control over a piece of memory, and borrowing represents temporary access to that memory.

When a variable is created in Rust, it becomes the owner of the memory it represents. The owner is responsible for freeing the memory when the variable goes out of scope. However, Rust also allows you to borrow references to the memory owned by another variable, but with certain constraints. The borrow checker enforces these constraints to prevent invalid memory access and data races.

The borrow checker analyzes Rust code to ensure that each reference to memory is valid and safe. It enforces a set of rules that govern how and when references can be created, used, and dropped. These rules include:

* Only one mutable reference to a piece of memory can exist at a time.

* Mutable references can't coexist with immutable references to the same piece of memory.

* References must always be valid and non-null.

* The lifetime of a reference must be shorter than the lifetime of the memory it refers to.

The borrow checker is an important part of Rust's memory safety guarantees and has become one of the most notable features of the language. It can be challenging to work with at first, especially for developers coming from languages without similar constraints, but it ultimately helps catch many memory-related bugs at compile time rather than at runtime.


## The borrow checker - example

The borrow checker guarantees that an immutable borrow never changes data. This guarantee enables you to have multiple immutable borrows of the same object simultaneously.

```rust
let mut a = ['a', 'b', 'c'];

let b = &a; // Borrow data immutably
//b[0] = 'x'; // Changing data won't compile
println!("{:?}", b[0]); // Reading data is fine

let c = &mut a; // Borrow data mutably
c[0] = 'x'; // Changing data is fine.
println!("{:?}", c[0]); // Reading data is fine.
```

The borrow checker guarantees that mutable borrows of the same object never overlap. This guarantee protects you from accidentally doing conflicting mutations in mutable borrows.

```rust
let mut a = ['a', 'b', 'c'];

// Valid code because the mutable borrows are one at a time.
let b = &mut a;
b[0] = 'x';
let c = &mut a;
c[0] = 'y';

// Invalid code because the mutable borrows overlap.
//let b = &mut a;
//let c = &mut a;
//b[0] = 'x';
//c[0] = 'y';
```


## Channels for thread communication

Rust channels are a way to facilitate communication between threads in Rust. They allow threads to send messages to each other in a synchronized and safe manner, without the need for explicit locking or other synchronization primitives.

In Rust, channels are created using the `std::sync::mpsc` module, which stands for "multiple producer, single consumer." This means that multiple threads can send messages into a channel, but there will only be one thread receiving those messages.

To create a channel, you first need to import the module, then you can send messages and receive messages.

```rust
use std::sync::mpsc;
fn main() {
    let (sender, receiver) = mpsc::channel(); // create channel
    sender.send("Hello, World!").unwrap(); // send message
    let message = receiver.recv().unwrap(); // receive message
}
```

If there are no messages in the channel, the `recv` method will block until a message is available. Alternatively, you can use the `try_recv` method to receive a message without blocking:

```rust
match receiver.try_recv() {
    Ok(message) => println!("Received message: {}", message),
    Err(_) => println!("No message received"),
}
```

It's important to note that sending and receiving messages through a Rust channel takes ownership of the values being sent. This means that the value being sent is moved into the channel, and can no longer be used by the sender after the send operation. Similarly, the value received from the channel is moved out of the channel, and can no longer be received by any other threads. This ownership model ensures that Rust channels are safe and thread-safe.


## Concurrency and parallelism

In Rust, concurrency refers to the ability of a program to perform multiple tasks or operations at the same time, while parallelism refers to the ability of a program to perform multiple tasks or operations simultaneously, using multiple processors or cores.

Rust provides mechanisms for concurrency and parallelism:

* Threads: Rust's standard library provides a low-level interface for creating and managing threads. Threads allow a program to execute multiple tasks in parallel, but require careful synchronization to avoid data races and other concurrency issues.

* Channels: Rust's channels provide a high-level mechanism for communication between threads. Channels allow multiple threads to send and receive data, and ensure that the data is transmitted in a synchronized and safe manner.

* Futures: Rust's futures provide a mechanism for asynchronous programming, allowing a program to perform non-blocking I/O and other operations without blocking the main thread. Futures are composable and can be combined to create complex asynchronous workflows.

* Atomic types: Rust's atomic types provide a safe and efficient way to share data between threads. Atomic types are designed to be thread-safe, and provide operations that ensure that data is updated atomically, without the need for locks or other synchronization mechanisms.

Rust's concurrency and parallelism mechanisms are designed to be safe and efficient, and take advantage of Rust's ownership and borrowing system to prevent data races and other concurrency issues. Additionally, Rust's compiler provides powerful static analysis and optimization tools that can help identify and eliminate potential issues in concurrent and parallel code.


## Error messages

Rust is known for having particularly helpful and informative error messages compared to other programming languages. Rust's error messages are designed to be both human-readable and actionable, providing developers with clear guidance on how to fix issues in their code.

Here are some key features of Rust error messages:

* Contextual information: Rust's error messages typically include contextual information such as the location and type of the error, as well as relevant code snippets and variable values.

* Suggested fixes: In many cases, Rust will provide suggested fixes for common errors, such as missing semicolons or incorrect variable types. These suggestions can save developers time and make it easier to correct errors.

* Explanation of the problem: Rust's error messages often include detailed explanations of the problem, helping developers to understand the underlying issue and how to avoid it in the future.

* Help with complex concepts: Rust's error messages can also help with complex concepts like ownership and borrowing. The messages will often explain how Rust's ownership system works and suggest ways to restructure code to avoid common pitfalls.

* Clear formatting: Rust's error messages are designed to be easy to read and understand, with clear formatting and helpful color coding.

Overall, Rust's error messages are a powerful tool for developers, helping them to identify and fix issues in their code quickly and efficiently. They are a testament to Rust's focus on developer experience and the language's commitment to making it easy to write safe and efficient code.

## Foreign Function Interface (FFI)

In Rust, the Foreign Function Interface (FFI) allows Rust code to interoperate with code written in other languages, such as C or C++. This enables Rust to be used in mixed-language projects or to use existing libraries that are written in other languages.

To use the FFI in Rust, you first need to declare an external function or type from another language using the `extern` keyword:

```rust
extern "C" {
    fn some_function(arg1: i32, arg2: *mut i32) -> i32;
}
```

This declares a function called `some_function` that takes an `i32` and a pointer to an `i32` as arguments and returns an `i32`. The "C" string in the `extern` declaration specifies the calling convention, which tells the Rust compiler how to interact with the external function.

To call this function from Rust, you can use the `unsafe` keyword to tell the Rust compiler that the function call is unsafe and may have side effects:

```rust
let arg1 = 42;
let mut arg2 = 0;
let result = unsafe { some_function(arg1, &mut arg2) };
```

This calls the `some_function` function with the specified arguments, passing a mutable reference to `arg2` using the `&mut` operator.

Rust also provides a `#[no_mangle]` attribute that can be used to disable Rust's name mangling, which can be useful when interacting with external libraries. For example, you can declare a Rust function with the `#[no_mangle]` attribute and call it from C code.

In summary, the Rust FFI enables Rust code to interoperate with code written in other languages, and can be used to call external functions from Rust or to expose Rust functions to other languages.


## Futures for asynchronous operations

In Rust, a future is a type that represents an asynchronous operation that may not have completed yet. Futures are for writing non-blocking code, such as to read a file, make a web request, or query a database.

Rust's futures are composable, which means that multiple futures can be combined to create more complex workflows. Futures can be chained together to form a pipeline, with each future as a step in the pipeline. When a future completes, it can trigger the next future to execute.

Futures are executed by an executor, which is responsible for scheduling and running the futures. Rust provides several built-in executors.

Example of a Rust future for an asynchronous HTTP request:

```rust
use futures::Future;
use reqwest::Url;

async fn fetch_url(url: Url) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    Ok(text)
}

fn main() {
    let url = Url::parse("https://example.com").unwrap();
    let future = fetch_url(url);
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let text = runtime.block_on(future).unwrap();
    println!("response text is {}", text)
}
```

This example defines an asynchronous function `fetch_url`. The function accepts a URL, then uses the `reqwest` crate to make an HTTP GET request to the URL, then returns the response text as a `String`.

The `fetch_url` function is async, so returns a `Future` that we store in a variable. We use the tokio runtime to run the `Future`. This blocks until it completes. Finally, we print the result.


## Monomorphisation

Rust monomorphization is a process where generic code is transformed into specific code for each concrete type used in the program. In other words, it is the process of generating specialized code for each type that is used in a generic function or struct.

This is different from traditional dynamic dispatch, where a function or method call is resolved at runtime, based on the type of the object or value being operated on. With monomorphization, the specific implementation of a generic function is determined at compile-time, and there is no runtime overhead associated with dynamic dispatch. Monomorphization makes Rust code faster and more efficient than code that relies on dynamic dispatch.

Here's an example of monomorphism in Rust:

```rust
fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let int_sum = add(1, 2);
    let float_sum = add(1.0, 2.0);
    println!("Integer sum: {}", int_sum);
    println!("Float sum: {}", float_sum);
}
```

In this example, the add function takes two arguments of type `T`, which must implement the `std::ops::Add trait`, and returns their sum of the same type `T`. Because the type parameter `T` is constrained to implement `std::ops::Add`, the compiler can statically determine the concrete type of `T` at compile-time, resulting in monomorphic code that is optimized for the specific types used.

In the `main` function, we call add twice: once with integers and once with floats. Since Rust uses monomorphization, the compiler generates two separate versions of the add function, one for integers and one for floats. This results in efficient optimized code.

## Resource Acquisition Is Initialization

Resource Acquisition Is Initialization (RAII) is a fundamental concept in many programming languages, and helps memory safety.

RAII is a way of managing resources such as memory, files, connections, etc.  The core idea: when you acquire a resource, then you initialize an object that represents that resource; when that object is no longer needed, then its destructor is called, which releases the resource.

In Rust, RAII is implemented through the use of ownership and the `Drop` trait. Whenever an object is created in Rust, it is associated with an owner that is responsible for managing its memory and resources. When the owner goes out of scope, Rust automatically calls the `Drop` trait implementation for that object, which allows the object to clean up any resources it may have acquired.

Example of RAII with the standard library File type:

```rust
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::create("example.txt")?;
    // Do some work with the file variable ...
    Ok(())
}
```

In this example, we create a new `File` object using the `File::create()` method, which opens a new file for writing. When the file variable goes out of scope at the end of the function, Rust automatically calls the file's destructor, which closes the file handle and frees the file's resources.

RAII for managing resources and it helps ensure that your programs are both safe and reliable. By relying on RAII and the ownership system, Rust programs can avoid many common problems such as resource leaks, null pointer dereferences, and other forms of undefined behavior.


## Unsafe code

Rust is a programming language that prioritizes safety and correctness. However, there are situations where you may need to bypass Rust's built-in safety checks to perform certain operations. In these cases, Rust provides a way to write unsafe code within a safe Rust program.

Unsafe code is Rust code that the compiler cannot verify for safety at compile-time. This code is typically used when working with low-level operations that require direct access to system resources or when interacting with code written in other programming languages.

In unsafe code, Rust allows the use of several features that are not permitted in safe code, including:

* Dereferencing raw pointers: Raw pointers are unmanaged pointers that do not have any safety guarantees. Dereferencing raw pointers can lead to undefined behavior, such as null pointer dereferences, use-after-free errors, and other memory-related bugs.

* Calling unsafe functions: Unsafe functions are Rust functions that are marked with the unsafe keyword. These functions can perform operations that are not safe to perform in safe Rust code, such as accessing memory directly or performing system-level operations.

* Modifying global state: Rust's ownership and borrowing system ensures that data is accessed safely. However, unsafe code can bypass these guarantees, and modify global state directly, which can lead to race conditions and other bugs.

Code marked as unsafe doesn't mean it's inherently dangerous or incorrect. Unsafe code is often necessary for performance-critical code, interfacing with external systems, or implementing low-level abstractions. However, writing and working with unsafe code requires a deep understanding of Rust's memory and ownership model. Rust also provides several tools, such as unsafe blocks, to help ensure that unsafe code is written and used correctly.


## WebAssembly (WASM)

WebAssembly (WASM) is a binary instruction format that allows code to be executed in a sandboxed environment on web browsers, outside of the JavaScript runtime. Rust is one of the programming languages that can be compiled to WebAssembly, which allows Rust code to be executed in web browsers and other WASM environments.

Rust's support for WebAssembly comes through the Rust stdweb and wasm-bindgen crates, which provide tools for interacting with the WASM environment from Rust code. These crates allow Rust code to be compiled to WASM and provide a bridge between Rust and JavaScript, enabling Rust functions to be called from JavaScript and vice versa.

One of the main benefits of using Rust for WebAssembly is performance. Rust's focus on low-level control and efficient memory management make it a good fit for WASM, which has similar performance requirements to native code. Additionally, Rust's ownership and borrowing model can help prevent memory-related bugs in WASM code, which is especially important in the security-sensitive environment of the web.

Rust's support for WebAssembly also extends beyond the web. WASM can be run in a variety of environments, including mobile devices, IoT devices, and server-side applications. Rust's cross-platform support and memory safety features make it a good choice for developing WASM applications that can run on a variety of platforms.

To use the WASM crate, add the dependency to your project `Cargo.toml` file:

```
[dependencies]
wasm-bindgen = "0.2.72"
```

Overall, Rust's support for WebAssembly makes it a powerful tool for developing high-performance, secure, and cross-platform applications that can be executed in a variety of environments, including web browsers.



## WebAssembly - example

Create a new Rust project, such as running:

```sh
cargo new wasm-example --lib
```

Add the `wasm-bindgen` dependency to your `Cargo.toml` file.

In your `lib.rs` file, add the `wasm_bindgen` macro to the top of the file, and define a simple Rust function that takes two numbers and returns their sum:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Build your Rust code as a WebAssembly module by running the following command, which creates a WASM file called `wasm-example.wasm` in the `target/wasm32-unknown-unknown/release/` directory:


```sh
cargo +nightly build --target wasm32-unknown-unknown --release
```

Finally, create a JavaScript file that loads the WASM module and calls your Rust function:

```javascript
import("./wasm_example_bg.wasm").then((module) => {
  const { add } = module;
  console.log(add(1, 2)); // outputs 3
});
```

This JavaScript code loads the WASM module using the `import()` function, which is a new feature in JavaScript that allows you to dynamically load modules at runtime. Once the module is loaded, you can call your Rust function using the `add` variable.


## Zero-cost abstractions

In Rust, zero-cost abstractions are a design principle that refers to the idea that abstractions, such as functions and data structures, should not impose any runtime overhead compared to the equivalent low-level, manual code that they replace.

This means that, while Rust's standard library provides a high-level API with powerful abstractions, the generated code should be just as fast and efficient as if the code were manually written with lower-level constructs.

To achieve this, Rust uses a combination of static analysis and code generation techniques, such as inlining, loop unrolling, and code specialization. For example, the Rust compiler may choose to inline a function call instead of generating code to jump to the function at runtime, thereby avoiding the overhead of a function call.

Furthermore, Rust's ownership and borrowing system allows the compiler to optimize the generated code by eliminating unnecessary memory allocations and deallocations, reducing runtime overhead and improving performance.

This approach allows Rust developers to write high-level code that is easy to read and maintain, while still achieving the performance and efficiency of low-level code. This makes Rust a popular choice for performance-critical applications, such as game engines, web browsers, and operating systems.

Overall, zero-cost abstractions are an important aspect of Rust's design, and they enable Rust to combine high-level abstractions with low-level performance, making it a powerful and efficient language for building complex and performance-critical systems.


## Zero-cost abstractions - example

Here's an example of zero-cost abstractions:

```
fn add<T: std::ops::Add<Output=T>>(x: T, y: T) -> T {
    x + y
}

fn main() {
    let x = 1;
    let y = 2;
    let z = add(x, y);
    println!("{}", z);
}
```

In this example, the `add` function takes two arguments of any type that implements the `Add` trait, adds them together using `+`, and returns the result.

The `add` function is generic, so it can be used with any type that implements `Add`, such as numbers, strings, or even custom objects.

Because the function is generic, it will be optimized by the Rust compiler to perform as efficiently as possible. This means that using the `add` function will not incur any additional runtime overhead, even though it uses an abstraction (the `Add` trait) to make the function more generic and reusable.

In this way, Rust demonstrates the concept of zero-cost abstraction, allowing developers to write modular, reusable code without sacrificing performance.


# Type guideposts

This page is a section divider and intentionally blank.


## Scalar types

Rust has several scalar types that represent basic values and data structures. These types are built into the language and do not require any additional dependencies or libraries to use.

Boolean (bool): Represents a logical value, either true or false.

```rust
let a: bool = true;
```

Signed integers (i8, i16, i32, i64, i128): Represent whole numbers that can be positive or negative. The number after the 'i' represents the number of bits the integer type uses.

```rust
let a: i8 = 1;
let b: i16 = 1;
let c: i32 = 1;
let d: i64 = 1;
let e: i128 = 1;
```

Unsigned integers (u8, u16, u32, u64, u128): Represent whole numbers that can only be positive. The number after the 'u' represents the number of bits the integer type uses.

```rust
let a: u8 = 1;
let b: u16 = 1;
let c: u32 = 1;
let d: u64 = 1;
let e: u128 = 1;
```

Floating-point numbers (f32, f64): Represent decimal numbers with single or double precision.

```rust
let a: f32 = 1.0;
let b: f64 = 1.0;
```

Character (char): Represents a single Unicode character.

```rust
let a: char = 'a';
```


## Compound types

In Rust, a compound type is a type that is composed of other types. There are two main compound types in Rust: tuples and arrays.

Tuples: A tuple is an ordered list of elements of different types. Tuples in Rust are declared using parentheses and the elements are separated by commas. For example, the following code creates a tuple containing a string and an integer:

```rust
let my_tuple = ("Hello, World!", 42);
```

We can access the individual elements of a tuple using indexing syntax:

```rust
let my_tuple = ("Hello, World!", 42);
let my_string = my_tuple.0;
let my_int = my_tuple.1;
```

Arrays: An array is a fixed-size collection of elements of the same type. Arrays in Rust are declared using square brackets and the elements are separated by commas. For example, the following code creates an array of integers with five elements:

```rust
let my_array = [1, 2, 3, 4, 5];
```

We can access the individual elements of an array using indexing syntax:

```rust
let my_array = [1, 2, 3, 4, 5];
let my_element = my_array[2]; // Access the third element
```

Arrays in Rust have a fixed size, which means that they cannot be resized at runtime. However, Rust provides a more flexible compound type called a vector, which can be resized dynamically.

Compound types are useful for grouping related data together and passing them around as a single unit. They also allow for more complex data structures and algorithms to be created. By using tuples and arrays effectively, Rust developers can write more efficient and maintainable code.


## Sum types and product types

In Rust, sum types and product types are two fundamental concepts in algebraic data types, which are used to define custom data structures.


### Sum type

A sum type is a type that can have one of several possible values. In Rust, sum types are defined using the `enum` keyword. An enum can have one or more variants, each of which can contain different types of data.

Example sum type:

```rust
enum Color {
    Red,
    Green,
    Blue,
}
```

In this example, `Color` is a sum type that combines three variants into a single type.


### Product type

A product type is a type that combines two or more types into a single type. In Rust, product types are defined using the tuple syntax, which looks like (T1, T2, ..., Tn). The resulting type can be thought of as a record that contains values of each of the individual types.

Example product type:

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
```

In this example, `Point` is a product type that combines three i32 values into a single type.



## str type versus String type

In Rust, both `str` and `String` are used to represent textual data, always using Unicode UTF-8 encoded u8 bytes. But they have some differences in terms of how they are stored and accessed.

* Type: `str` is a primitive type. `String` is a standard library type.

* Memory: `str` is immutable, and can be stored in the program's binary, or stack, or heap. `String` is mutable, growable, and heap-allocated.

* Ownership: `&str` is a slice that borrows ownership from another string or a static string literal. In contrast, `String` owns the string data it contains.

* Lifetime: `str` has a static lifetime (i.e., it lives as long as the program runs) in case of string literals, or a borrowed lifetime (i.e., it lives as long as the reference it was borrowed from) in case of borrowed slices. `String` has a dynamic lifetime (i.e., it lives as long as there is a reference to it).

* Usage: `&str` is usually used for function arguments and return types, or for string literals, while `String` is typically used when you need to create or modify a string at runtime.

* Coercion: a `&String` can be coerced to a `&str`, such as when a `&String` arg is passed to a function signature with a `&str` parameter.

* Indexing: Indexing by bytes is different than counting by characters, because `str` and `String` both store Unicode characters, using the UTF-8 variable-width encoding format, which means one character can take up more than one byte.


## String types for UTF-8, C, OS, and paths

Rust provides different string types for different kinds of text.

`str` and `String`: a Unicode UTF-8 value sequence. A `str` is slice-like and immutable. A `String` is owned, mutable, and growable.

`CStr` and `CString`: a C-style null-terminated char byte sequence. A `CStr` is slice-like and immutable. A `CString` is owned, mutable, and growable.

`OsStr` and `OsString`: a platform-specific operating system string. A `OsStr` is slice-like and immutable. A `OsString` is owned, mutable, and growable.

`Path` and `PathBuf`: a platform-specific file path string. A `Path` is slice-like and immutable. A `PathBuf` is owned, mutable, and growable.

Converting between a platform-specific type (`OsStr`, `OsString`, `Path`, `PathBuf`) and a platform-independent type (`str`, `String`) may require lossy conversion, or handling conversion errors.

Examples of string types:

```rust
use std::ffi::{CStr, CString};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

let a: &str = "foo";
let b: String = String::from("foo");

let c: &CStr =  CStr::from_bytes_with_nul(b"foo\0")
    .expect("Error in CStr::from_bytes_with_nul");
let d: CString = CString::from_vec_with_nul(b"foo\0".to_vec())
    .expect("Error in CString::from_vec_with_nul");

let e: &OsStr = OsStr::new("foo");
let f: OsString = OsString::from("foo");

let g: &Path = Path::new("foo");
let h: PathBuf = PathBuf::from("foo");
```


## Box type for a smart pointer

In Rust, a `Box` is a smart pointer that provides a way to allocate memory on the heap and move data into that memory.

`Box` will allocate an object at runtime rather than at compile time. When a value is wrapped in a `Box`, it is moved to the heap and the `Box` itself is stored on the stack. This allows you to allocate a large object on the heap without having to worry about stack size limitations. When a `Box` goes out of scope, the memory it allocated is automatically deallocated. This eliminates the need to manually manage memory and helps prevent common memory-related bugs such as memory leaks and dangling pointers.

Another benefit of `Box` is that it enables ownership transfer. When you move a value into a `Box`, you transfer ownership of the value to the `Box`. This means that the `Box` becomes the owner of the value and is responsible for cleaning it up when it goes out of scope. This can be useful when you need to transfer ownership of a value between different parts of your program.


### Usage

To use `Box`, you can create a new instance by calling the `Box::new` function and passing in the value you want to allocate on the heap. For example, to allocate a new `i32` value on the heap and store it in a `Box`:

```rust
let my_box = Box::new(42);
```

This creates a new `Box` that contains the value 42. When `my_box` goes out of scope, the memory it allocated will be automatically deallocated.

Overall, `Box` is a useful tool for allocating objects on the heap, transferring ownership between parts of your program, and using automatic deallocation to help prevent memory-related bugs.



## Cow type for clone-on-write

<https://doc.rust-lang.org/std/borrow/enum.Cow.html>

A Rust `Cow` type is a clone-on-write smart pointer. When a function receives a `Cow` type as an argument, the function can modify the data without actually modifying the original data structure. Instead, the `Cow` type makes a clone of the data when it is modified, and any other references to the original data continue to point to the original data.

The `Cow` type is implemented as an enum with two variants: `Borrowed` and `Owned`, which express "either a reference, or an owned type". You choose which variant you want depending on your goal.

```rust
use std::borrow::Cow;

fn main() {
    let a = ['a', 'b', 'c'];
    let mut b = Cow::Borrowed(&a);

    // The `b` Cow enum is borrowed, and points to `a`.
    match b {
        Cow::Borrowed(_) => println!("Borrowed"),
        Cow::Owned(_) => println!("Owned"),
    }

    // Convert `b` to mutable i.e. clone it, then change it.
    b.to_mut()[0] = 'x';

    // Now the `b` Cow enum is Owned i.e. has its own data.
    match b {
        Cow::Borrowed(_) => println!("Borrowed"),
        Cow::Owned(_) => println!("Owned"),
    }
}
```

A typical use case for `Cow` is optimization by not doing copies. For example, you write a function that returns a String, but there are cases when you already have a &'static str containing the data; you can return `Cow::Borrowed` so you don't need to allocate and copy a new String.


## RefCell type for dynamic borrowing

The Rust `RefCell` type is a container type that provides dynamic borrow checking at runtime, allowing for mutable or immutable borrows of its inner value based on certain rules. For example, there are cases where runtime borrow checking is necessary, such as when a value needs to be mutated within a shared reference.

* Mutable borrows: `RefCell` provides mutable borrows of an inner value through the use of its `borrow_mut` method. This method returns a mutable reference (`&mut`) to the inner value, which can be modified. However, `borrow_mut` will panic at runtime if there are any outstanding references (mutable or immutable) to the inner value.

* Immutable borrows: `RefCell` provides immutable borrows through its `borrow` method, which returns an immutable reference (`&`) to the inner value. Multiple immutable references can be outstanding at the same time, but attempting to call borrow_mut while there are outstanding immutable references will cause a panic.

Example:

```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(1);

    // Borrow a mutable reference to x's inner value
    let mut mutable_ref = x.borrow_mut();
    *mutable_ref = 2;

    // Borrow an immutable reference to x's inner value
    let immutable_ref = x.borrow();
    println!("The value of x is: {}", *immutable_ref);
}
```

## Rc type for single-thread sharing

In Rust, `Rc` (Reference Counted) is a smart pointer that provides shared ownership of a value. `Rc` tracks the number of references to a value. If a new reference is created, `Rc` increments the reference count. If an existing reference is dropped, `Rc` decrements the reference count. When the reference count reaches zero, `Rc` drops the value.

Unlike `Arc` smart pointer, `Rc` cannot be safely shared between threads and is used for single-threaded scenarios.

For example, consider the following code:

```rust
use std::rc::Rc;

fn main() {
    let shared_data = Rc::new(vec![1, 2, 3]);
    let data1 = shared_data.clone();
    let data2 = shared_data.clone();

    println!("{:?}", shared_data);
    println!("{:?}", data1);
    println!("{:?}", data2);
}
```

Here, an `Rc` shares ownership of a vector between multiple references. The `Rc::new()` function creates a new `Rc` that points to a vector of [1, 2, 3]. The `clone()` method creates two new `Rc`s that point to the same vector, and the reference count is incremented. The `println!()` macro prints the values of each reference to the console.

`Rc` is a useful tool for scenarios where shared ownership of a value is needed in a single-threaded environment. By using reference counting to manage the lifetime of the value, `Rc` ensures that the value is not dropped until all references to it have been dropped.


## Arc type for multi-thread sharing

In Rust, `Arc` (Atomically Reference Counted) is a smart pointer that provides shared ownership of a value, similar to `Rc` (Reference Counted) smart pointer. The difference is that `Arc` can be safely shared between threads, for concurrent programming; this is because `Arc` uses atomic operations to increment and decrement the reference count.

`Arc` works by keeping track of the number of references to a value. When a new reference to the value is created, `Arc` increments the reference count. When an existing reference is dropped, `Arc` decrements the reference count. When the reference count reaches zero, `Arc` drops the value. When an `Arc` is cloned, a new pointer to the same value is created, and the reference count is incremented.

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let shared_data = Arc::new(vec![1, 2, 3]);
    for i in 0..3 {
        let data = shared_data.clone();
        thread::spawn(move || {
            let vec = data.iter()
            .map(|x| x + i).collect::<Vec<_>>();
            println!("{:?}", vec);
        });
    }
}
```

Here, an `Arc` shares ownership of a vector between multiple threads. The `Arc::new()` function creates a new `Arc` that points to a vector of `[1, 2, 3]`. The `clone()` method creates a new `Arc` that points to the same vector, and the reference count is incremented. The `thread::spawn()` function creates three threads, each of which iterates over the vector and adds the current loop index to each element. The results are collected into a new vector, which is printed to the console.


## Pin type for memory location

<https://doc.rust-lang.org/std/pin/>

Rust `Pin` type is a type that is used to express that a value should not be moved in memory. When an object is pinned, it means that its memory location cannot change, even if other parts of the program try to move it.

The `Pin` type is commonly used in Rust when dealing with data structures that hold references to other objects. In such cases, moving the data structure could invalidate the references, leading to undefined behavior.

To create a pinned object, you can use the `Pin::new` function, which takes a reference to the object and returns a `Pin` wrapper around it. This Pin wrapper can then be used to access the object, but it cannot be moved or dropped without first calling the unpin method on it.

Additionally, Rust provides a `Pin<&mut T>` type, which can be used to create a pinned reference to a mutable object. This allows you to modify the object through the reference while still ensuring that its memory location does not change.

Overall, Rust Pin `type` is an important tool for ensuring memory safety when dealing with complex data structures and references. It allows you to express the requirement that certain objects should not be moved in memory, which can help prevent bugs and ensure the correctness of your program.




## Pin type - example

Here's an example of how to use the Rust Pin type:

```rust
use std::pin::Pin;

struct Data {
    value: i32,
}

impl Data {
    fn new(value: i32) -> Self {
        Self { value }
    }
}

fn main() {
    let data = Data::new(1);
    let pinned_data = Pin::new(&data);

    // Invalid move of `data`:
    // let moved_data = data;

    // Invalid move of `pinned_data`:
    // let moved_pinned_data = pinned_data;

    // We can access the value of `data` through `pinned_data`
    println!("{}", pinned_data.value);
}
```

In this example, we define a Data struct that holds a single integer value. We then create a new instance of this struct, and use the `Pin::new` function to create a Pin wrapper around a reference to this instance.

Once `pinned_data` is created, trying to move data results in a compile-time error. Similarly, attempting to move `pinned_data` results in a compile-time error, because it is a wrapper around a pinned reference.

We can still access the value of data through `pinned_data`, as shown by the `assert_eq!` statement. The reference remains valid, even if the data structure itself is moved.


# Trait guideposts

This page is a section divider and intentionally blank.


## Copy trait and Clone trait for duplication

In Rust, the Copy trait controls how values are copied, while the Clone trait controls how values are cloned.

The `Copy` trait is used for types that can be safely copied bit-by-bit, without any special consideration for ownership or memory management. When a value with the `Copy` trait is assigned to a new variable or passed to a function, a bitwise copy of the original value is made. This means that the original value remains unchanged, and any changes made to the copied value do not affect the original.

Examples of types that implement the `Copy` trait include simple scalar types like integers and booleans, as well as tuples and arrays that only contain types that implement the `Copy` trait.

The `Clone trait`, on the other hand, is used for types that need to be explicitly cloned in order to make a copy. When a value with the `Clone` trait is cloned, a new instance of the value is created, and any owned data is also cloned. This means that changes made to the cloned value do not affect the original, and vice versa.

To implement the `Clone` trait for a type, you need to provide an implementation of the `clone` method, which creates a new instance of the type and clones any owned data. Rust also provides a default implementation of `Clone` for types that implement the `Copy` trait, which simply returns a bitwise copy of the value.

```
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }

fn main() {
    let a = Point { x: 10, y: 20 };
    let b = a; // This does a  copy
    let c = a.clone(); This does a clone
}
```


## Debug trait for debugging and printing

In Rust, the `Debug` trait is a built-in trait that allows developers to print and debug Rust types. It provides a basic representation of a type suitable for debugging purposes.

When a type implements the `Debug` trait, it can be printed using the println! macro with the `{:?}` format specifier. This will print a debug representation of the type, which is often more informative than the default string representation.

To implement the `Debug` trait for a custom type, developers need to define a `debug` method on the type that returns a `fmt::Debug` trait object. This method should return a formatter that describes the structure of the type in a way that is suitable for debugging.

For example, let's consider a simple `Point` struct:

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

We use the `derive` attribute to automatically generate an implementation of the `Debug` trait for our `Point` struct. This creates a `debug` method that returns a formatter that prints the `x` and `y` fields.

With this implementation, we can use the `println!` macro to print a `Point` value like this:

```rust
let p = Point { x: 10, y: 20 };
println!("Point: {:?}", p);
```

This will output:

```text
Point: Point { x: 10, y: 20 }
```


## Display trait for formatting

The Rust `Display` trait is a built-in trait that allows developers to format a value as a string for display purposes. It provides a human-readable representation of a type.

When a type implements the `Display` trait, it can be formatted as a string using the `format!` macro or the `println!` macro with the `{}` format specifier.

To implement the `Display` trait for a custom type, we define a `fmt` method on the type that takes a formatter object. The formatter object implements the `fmt::Write` trait, which provides methods for writing to a string buffer.

Example:

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The `write!` macro writes fields into the formatter
        write!(f, "x is {} and y is {}", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("Point {}", p); // "Point x is 1 and y is 2"
}
```
Here, we define a `fmt` method for the `Display` trait on our `Point` struct. This method takes a formatter object, and writes the `x` field and `y` field of the struct into a user-friendly string.


## dyn trait for dynamic dispatch

In Rust, a `dyn trait` is a way to specify a trait object with dynamic dispatch.

A `trait` object is a pointer to an object that implements a trait, and is used when the concrete type of an object is not known at compile time. In other words, it allows you to write code that can work with different types that implement a particular trait without knowing the exact type at compile time.

When defining a `trait` object in Rust, you can use the `dyn` keyword to indicate that the trait object should be dynamically dispatched. This means that the specific implementation of the trait for a given object will be determined at runtime rather than at compile time.

For example, consider the following trait definition:

```rust
trait MyTrait {
    fn my_method(&self);
}
```

To define a trait object with dynamic dispatch, use the dyn keyword:

```rust
fn my_function(obj: &dyn MyTrait) {
    obj.my_method();
}
```

In this example, `my_function` takes a reference to a trait object that implements the `MyTrait` trait, with dynamic dispatch specified using the `dyn` keyword. This means that at runtime, the specific implementation of `my_method` for the given object will be determined dynamically.

Using `dyn trait` allows Rust to provide runtime polymorphism, which is useful in situations where the concrete type of an object is not known at compile time, but needs to be determined at runtime. However, it can come at a performance cost compared to static dispatch, which is resolved at compile time.


## dyn trait in a type position

[Runnable project](https://github.com/SixArm/rust-guideposts/tree/main/projects/topics/dyn_trait_in_a_type_position)

You can use `&dyn` with a trait name in a type position. This is useful to abstract over a variety of implementations.

Example:

```rust
trait Speak {
    fn speak(&self);
}

type Cat;

impl Speak for Cat {
    fn speak(&self) {
        println!("meow");
    }
}

type Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("woof");
    }
}

fn main() {
    let pets: Vec<&dyn Speak> = vec![&Cat, &Dog];
    for pet in pets {
        pet.speak();
    }
}
```


## Eq, PartialEq, Ord, PartialOrd traits

In Rust, traits are used to define shared behavior for types. The following are commonly used traits for comparing types:

* Eq trait: This trait defines the equality relation between two values of a given type. The Eq trait requires that the type implements the PartialEq trait, which defines the partial equality relation. If two values of a type are equal according to the Eq trait, they must be considered indistinguishable in every way.

* PartialEq trait: This trait defines the partial equality relation between two values of a given type. The PartialEq trait requires that the type implements an eq method that takes another value of the same type as an argument, and returns a bool indicating whether the two values are equal. If two values of a type are equal according to the PartialEq trait, they must be considered indistinguishable for the purposes of the Eq trait as well.

* Ord trait: This trait defines the total order relation between two values of a given type. The Ord trait requires that the type implements the PartialOrd trait, which defines the partial order relation. If two values of a type are compared using the Ord trait, they must be completely ordered in a consistent way.

* PartialOrd trait: This trait defines the partial order relation between two values of a given type. The PartialOrd trait requires that the type implements a partial_cmp method that takes another value of the same type as an argument, and returns an Option<Ordering> indicating the ordering relationship between the two values. If two values of a type are compared using the PartialOrd trait, they must be partially ordered in a consistent way.

These traits are important for comparing types in Rust, and are used extensively in Rust's standard library.


## From and Into traits for conversions

The Rust `From` trait and `Into` trait are used to convert between types.

The `From` trait provides a `from` method that takes an argument of a different type and returns an instance of the implementing type. This allows for easy conversion between different types, especially when converting from a type that is not owned by the implementing type.

The `Into` trait provides an `into` method that takes no arguments and returns an instance of a different type. This allows for easy conversion between different types, especially when converting from a type that is owned by the implementing type.

Example:

```rust
struct MyStruct(i32);

// Convert from i32
impl From<i32> for MyStruct {
    fn from(val: i32) -> Self {
        MyStruct(val)
    }
}

// Convert into i32
impl Into<i32> for MyStruct {
    fn into(self) -> i32 {
        self.0
    }
}

fn main() {
    let my_struct = MyStruct::from(42);
    let i: i32 = my_struct.into();
}
```

This example defines a simple `MyStruct` struct. We implement the `From` trait `from` method. We implement the `Into` trait `into` method.


## Send trait for sending among threads

The Rust `Send` trait indicates that a type is safe to be sent across thread boundaries. This means that if a type implements the `Send` trait, it can be safely passed from one thread to another without causing any data races or undefined behavior. For example, the `String` type in Rust implements the `Send` trait, which means it can be safely shared across multiple threads.

To implement the `Send` trait for a custom type, all of its fields must also implement the `Send` trait. This is because if a type contains non-`Send` fields, it may be possible for data races to occur when the type is shared across threads. The `Send` trait is automatically implemented for most primitive types in Rust, as well as many standard library types like `Vec` and `String`.

Here's an example of a custom type that implements the `Send` trait:

```rust
struct Foo {
    x: i32,
    y: String,
}

unsafe impl Send for Foo {}

fn main() {
    let foo = Foo { x: 42, y: "Hello".to_string() };
    std::thread::spawn(move || {
        println!("x = {}, y = {}", foo.x, foo.y);
    }).join().unwrap();
}
```

In this example, the `Foo` struct contains an `i32` field and a `String` field. Both `i32` and `String` implement the `Send` trait, so we can implement `Send` for `Foo` using the `unsafe impl Send for Foo {}` syntax. We can safely send a `Foo` instance to a new thread using `std::thread::spawn`, and access its fields from within the thread.


## Sync trait for syncing among threads

The Rust `Sync` trait indicates that a type is safe to be shared between multiple threads. If a type implements the `Sync` trait, it can be safely accessed from multiple threads without causing any data races or undefined behavior. For example, the `Arc` type implements the `Sync` trait, so it can be safely shared between multiple threads.

Here's an example of how the Sync trait can be used:

```rust
use std::sync::Arc;

struct MyStruct {
    x: i32,
}

impl MyStruct {
    fn my_function(&self) {
        println!("x is {}", self.x);
    }
}

fn main() {
    let s = MyStruct { x: 1 };
    let shared = Arc::new(s);
    std::thread::spawn({
        let shared = shared.clone();
        move || { shared.my_function(); }
    }).join().unwrap();
}
```

In this example, we create a shared instance of `MyStruct` using the `Arc` type, which automatically implements the `Sync` trait. We can then safely access the shared instance from multiple threads, without worrying about synchronization issues.


## Send & Sync - implementations

<https://stackoverflow.com/questions/68704717/>

The `Send` trait and `Sync` trait show up frequently in multi-threaded projects. Here are implementations that are especially good to know.

`&T`: since immutable references can be copied, the ability to send one to another thread would let you perform immutable access from several threads in parallel. Thus `&T` can only be `Send` if `T` is `Sync`. There is no need for `T` to be `Send` as an `&T` doesn't allow mutable access.

`&mut T`: mutable references can't be copied, so sending them to other threads doesn't allow access from several threads in parallel, thus `&mut T` can be `Send` even if `T` is not `Sync`. Of course, `T` must still be `Send`.

`MutexGuard`: destroying a `MutexGuard` on another thread is unsound, so it can't be `Send`. However if the value inside may be immutably accessed from several threads in parallel, then such an immutable access would also be safe on the `MutexGuard` itself.

`SyncWrapper`: an immutable reference to a `SyncWrapper<T>` does not allow you to perform any actions at all; it is always safe to be `Sync`.

`Rc<T>`: if you have two clones of the same `Rc<T>`, then it would be a data race to access them from different threads in parallel. This rules out both `Send` and `Sync`, since both of them would allow immutable access from other threads, and that other thread could use that to call `.clone()` remotely and obtain an `Rc<T>` on the other thread.

`Arc<T>`: this mostly behaves like `&T`. It can be cloned, so sending it to other threads requires `T: Sync`. However, it also requires `T: Send` as the last `Arc<T>` might be dropped on a different thread than where `T` was created, which you can't do without `Send`.

`RefCell<T>`: this type can never be `Sync` because you can modify the value inside with only an immutable reference, and this would be a data race if you could do it from several threads in parallel. There's no problem with `RefCell<T>` being `Send` provided that `T` is.


## Sealed traits

In Rust, a sealed trait is a trait that can only be implemented within a particular module, and not outside it. This means that once a trait is marked as "sealed", any other code outside the module where the trait was defined cannot implement it.

Sealed traits are useful when you want to limit the set of types that can implement a particular trait to a specific set of types. This can be helpful when designing APIs or libraries where you want to restrict the use of certain traits to specific contexts or modules.

To define a sealed trait in Rust, you must declare the trait as `pub` and include a private `mod` statement with the same name as the trait. This private module should contain all the implementations of the trait.

Here's an example:

```rust
pub trait Sealed {}

mod private {
    use super::Sealed;

    impl Sealed for i32 {}
    impl Sealed for String {}
}

pub fn foo<T: Sealed>(val: T) {
    // do something with val
}
```

In this example, the Sealed trait is defined as pub and the implementations are placed in a private module called private. The foo function is generic over T where T must implement the Sealed trait.

Because the private module is private, no other code outside the module can implement the Sealed trait, ensuring that only the types explicitly listed within the module can be used with the trait.


# Keyword guideposts

This page is a section divider and intentionally blank.


## enum keyword for enumerations

In Rust, an enum (short for "enumeration") is a custom data type that allows you to define a set of named values. Each value is called a variant, and you can use an enum to represent a fixed set of possible values for a particular data type.

Here's an example of an enum in Rust:

```rust
enum Color {
    Red,
    Green,
    Blue,
}
```

In this example, we've defined an enum called `Color` with three variants: `Red`, `Green`, and `Blue`. We can use this enum to represent a color value in our Rust program.

Enums in Rust can also include data associated with each variant. Here's an example:

```rust
enum IPAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

In this example, we've defined an enum called `IPAddress` with two variants: `V4` and `V6`. The `V4` variant includes four `u8` values representing the four octets of an IPv4 address, while the V6 variant includes a single `String` value representing an IPv6 address.

Enums in Rust can be useful for a variety of programming tasks, including defining states for a state machine, representing different types of errors, and creating custom data types for your program. Rust's enums are type-safe and flexible, making them a powerful tool for Rust programmers.


## struct keyword for custom data types

A Rust `struct` is a custom data type that groups related data and functions. A struct is defined using the `struct` keyword, followed by the name of the struct, and a block of curly braces that contains the fields of the struct.

Here is an example of a struct in Rust:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}
```

Usage:

```rust
let r = Rectangle {
    width: 10,
    height: 20
};
```

Structs can also have functions associated with them, called methods. Methods are defined within the block of curly braces after the fields of the struct, and can be used to operate on the data within the struct.

Example methods:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

This example uses `impl` to define an implementation block for the `Rectangle` struct, and defines a method named `area` that calculates the area of the rectangle. The `&self` parameter indicates that the method takes a reference to the struct as its first argument.


## union keyword for multi-type memory

A Rust union is a user-defined type that is similar to a struct, but instead of each field having its own memory space, a union has a single memory space that can be interpreted as different types depending on the current value of the union.

To define a union, use the union keyword, then the name of the union, then the fields of the union. For example:

```rust
union MyUnion {
    i: i32,
    f: f32,
}
```

In this example, `MyUnion` has two fields: `i` and `f`. The union can hold one of these fields at a time, not both. When you access a field of a union, Rust ensures the field is the active field of the union.

To change the value, use the unsafe keyword and transmute function. For example:

```rust
unsafe {
    let mut my_union = MyUnion { i: 42 };
    my_union.f = std::mem::transmute(3.14f32);
}
```

In this example, we use `std::mem::transmute` to convert a `f32` value into a bit pattern that can be interpreted as an `i32`. We then assign this value to `my_union.f`. Because we haven't accessed `my_union.i` since it was set, Rust considers `my_union.f` to be the active field of the union. If try to access `my_union.i` now, it would be undefined behavior (UB).

Because unions represent different types in the same memory space, it's easy to accidentally create bugs. In general, only use unions when you need to work with low-level data structures or when you need to optimize memory usage.


## match keyword for control flow

The `match` keyword is a control flow construct that allows a program to match a value against a set of patterns and execute code based on the match result. The `match` keyword statement is similar to a `switch` keyword statement in other languages, but `match` provides more powerful pattern matching capabilities.

A `match` statement typically has the following syntax:

```rust
match <value> {
    <pattern_1> => <code_1>,
    <pattern_2> => <code_2>,
    ...
    <pattern_n> => <code_n>,
}
```

The `<value>` is the expression that is being matched against, and the `<pattern>` expressions are the patterns that are being matched. Each `<pattern>` is followed by a `=>` symbol, then a block of code that will be executed if the pattern matches the value.

In Rust, a pattern can take many forms, including literal values (e.g. `42`, "hello"), variables (e.g. `x`, `y`), wldcards (e.g. `_`), ranges (e.g. `1..=5`), enums (e.g. `Some(value)`), structs (e.g. `Point { x, y }`), tuples (e.g. `(x, y)`), and more.

The code in each match arm is executed if the pattern on the left-hand side of the `=>` operator matches the value being matched. If none of the patterns match, the `match` statement will panic at runtime.

Rust's `match` statements are powerful and flexible, allowing for complex patterns and expressions to be matched. Match statements are commonly used in Rust to handle errors, parse command-line arguments, and implement state machines, among other use cases.

Overall, match statements are a key feature of Rust's control flow syntax, and provide a powerful mechanism for pattern matching and value extraction in Rust programs.


## mod keyword for module namespaces

In Rust, namespaces are a way to organize and group related items, such as functions, types, and constants, under a common name. Namespaces are implemented using modules, which are Rust's primary mechanism for organizing code into reusable components.

Modules can be defined using the `mod` keyword, followed by the name of the module and its contents enclosed in curly braces:

```rust
mod my_module {
    fn private_function() {
        // implementation details here
    }
    pub fn public_function() {
        // implementation details here
    }
}
```

In this example, `my_module` is a module that contains two functions: `private_function`, which is not visible outside of the module, and `public_function`, which is marked as pub and can be accessed from other modules.

To use a module from another module, you can use the use keyword to bring its contents into scope:

```rust
use my_module::public_function;

fn main() {
    public_function();
}
```

In this example, we bring the `public_function` from `my_module` into the scope of main, allowing us to call it directly.

Overall, namespaces in Rust provide a powerful mechanism for organizing and structuring code, enabling developers to write more modular, reusable, and maintainable software.


## mod keyword for nested hierarchies

The Rust `mod` keyword can provide nested hierarchies, meaning that a modules can contain other modules:

```rust
pub mod outer {
    pub mod inner {
        pub fn hello() {
            println!("Hello");
        }
    }
}

fn main() {
    outer::inner::hello()
}
```

You can optionally add a `use` statement such as:

```rust
use outer::inner::hello;
fn main() {
    hello()
}
```

Module hierarchies can help test-driven development, because you can create an outer module `tests`, with an inner module for each function, to improve readability and encapsultation:

```rust
#[cfg(test)]
mod tests {
    mod my_function_1 {
        #[test]
        fn test_something() {
            assert!(/* ... */);
        }
    }
    /* ... */
}
```


## Nested-or-pattern for matching

[Source](https://www.reddit.com/r/rust/comments/12sbjyj/power_of_the_operator_in_pattern_matching/)

The nested-or-pattern for matching combines `|` expressions.

Example `if` statement without nested-or-pattern:


```rust
if let Some(2) | Some(3) | Some(5) | Some(7) = value {…}
```

And with nested-or-pattern:

```rust
if let Some(2 | 3 | 5 | 7) = value …
```

The nested-or-pattern can be useful in many kinds of statements.

Example `match` statement:

```rust
match value {
    Some(n @ (2 | 3 | 5 | 7)) => println!("{n} is a prime"),
    …
```

Example `let` statement:

```rust
let (Ok(i) | Err(i)) = [1, 2, 3].binary_search(&2);
```

Example function definition:

```rust
fn f((Ok(i) | Err(i)): Result<i32, i32>) {}
```


## async/await keywords for futures

Rust provides support for asynchronous programming through its `async`/`await` syntax. The `async` keyword defines a function that can be suspended and resumed later. The `await` keyword pauses execution of an `async` function until a condition is met.

When a function is declared with the `async` keyword, it becomes an asynchronous function. This means that the function can be paused at any point using the await keyword and resumed later when the awaited value becomes available. The async function returns a `Future` type that represents the result of the computation.

Example of an async function that returns a future:

```rust
async fn fetch(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
```

The example's `fetch` function is defined with the `async` keyword. The function uses the `reqwest` crate to make an HTTP request. The first `await` waits for the response. The second `await` waits for the body text.

Example of `await` that waits for a future:

```rust
async fn do_something() -> i32 {
    let future = get_result_async();
    let result = await!(future);
    result + 1
}
```

The example's `await!` pauses execution of the `do_something` function until `get_result_async` is completed. Once the future completes, the result is returned and the task is resumed. The value of the result is then incremented by 1 and returned as the final result.

## trait keyword for polymorphism

In Rust, a trait is a language construct that defines a set of methods that can be implemented by a type. Traits enable polymorphism, generic programming, and code reuse without sacrificing performance or safety.

Example trait that defines one method:

```rust
trait MyTrait {
    fn my_method(&self);
}
```

Example struct that implements the method:

```rust
struct MyStruct;

impl MyTrait for MyStruct {
    fn my_method(&self) {
        println!("Hello");
    }
}
```

Example function that takes the trait and calls the method:

```rust
fn foo<T: MyTrait>(item: T) {
    item.my_method();
}
```

To run it:

```rust
fn main() {
    let s = MyStruct{};
    foo(s)
}
```

Some of the common Rust traits are `Debug` and `Display` for formmating output, `Copy` and `Clone` for duplicating values, `From` and `Into` for converting values, and `Send` and `Sync` for multi-thread communication.


# Macro guideposts

This page is a section divider and intentionally blank.


## println! macro for printing output

The Rust `println!` macro is a built-in macro that is used to print text to stdout (standard output).

Here is an example code that uses the `println!` macro to print a simple message to the console:

```rust
fn main() {
    println!("Hello, World!");
}
```

In this example, we call the `println!` macro with one argument: the string `"Hello, World!"`. The macro then prints the string to the console.

The `println!` macro is similar to the `print!` macro, but adds a newline character (`\n`) to the end of the output, while the `print!` macro does not.

The `println!` macro can also accept additional arguments for string formatting. For example, we can use the `{}` placeholder to insert variables or values into the output string:

```rust
fn main() {
    let name = "Alice";
    let age = 30;
    println!("My name is {} and age is {}", name, age);
}
```

In this example, we use two placeholders (`{}`) in the output string to print the values of the `name` and `age` variables. When the macro is executed, it replaces the `{}` placeholders with the corresponding values (`"Alice"` and `30`, respectively). The resulting output would be:

```txt
My name is Alice and age is 30
```

The `println!` macro is similar to the `format!` macro for formatting strings, and the `write!` macro for writing formatted data into a buffer.


## assert! macro and friends for testing

The Rust testing framework provides macros for test assertions, such as:

* `assert!(condition)`: assert `condition` is true.

* `assert_eq!(a, b)`: assert `a` is equal to `b`.

* `assert_ne!(a, b)`: assert `a` is not equal to `b`.

Example:

```rust
let x = 1;
let y = 2;
assert!(x < y);
```

Example with an optional message:

```rust
let x = 1;
let y = 2;
assert!(x < y, "We want x to be less than y");
```

### Assertables crate

The Assertables crate provides more assert macros, such as:

* `assert_starts_with!(x, y)`: Does `x` start with `y`?

* `assert_contains!(array, element)`: Does `array` contains `element`?

* `assert_is_match(regex, string)`: Does `regex` match `string`?string.

Example:

```rust
let a = "hello world";
let b = "hello";
assert_starts_with!(&a, &b);
```


## regex! macro for lazy static optimization

<https://crates.io/crates/once-cell-regex>

The `regex!` macro takes a string literal and returns an expression that evaluates to a `&'static Regex`. This macro can be useful to avoid the problem of compiling a regex on every loop iteration.

The `regex!` macro capabilities are provided by the `once_cell` crate and `once-cell-regex` crate.

Add to `Cargo.toml`:

```toml
[dependencies]
once_cell = "*"
once-cell-regex = "*"
```

Example:

```rust
use once_cell_regex::regex;

fn main() {
    let r = regex!("hello");
    let x = r.is_match("hello world");
    println!("{}", x); // prints "true"
}
```

### once_cell crate

The `once_cell` crate can provide optimizations in many more ways, such as safe initialization of global data, general purpose lazy evaluation, runtime bytes, late initialization, and more. 

There are similar crates if you need related features: 

* If you want asynchronous capabilities, try the `async_once_cell` crate.
  
* If you want spinlocks, try the `lazy_static` crate.


## catch_unwind! macro to handle panic

The Rust `panic` `catch_unwind!` macro is a way to catch unwinding panics that can occur when a piece of code fails at runtime. When an unwinding panic happens, Rust unwinds the stack and calls the panic handler, which can be customized to do any number of things, such as print an error message or roll back a transaction.

The `catch_unwind!` macro allows you to catch these unwinding panics and handle them in a more controlled way. It returns a Result value that lets you know if the code in the block panicked or not. If it did panic, you can then handle the error in any way you see fit, such as printing an error message or returning an alternate value.

Here's an example of how to use the `catch_unwind!` macro:

```rust
use std::panic;

let result = panic::catch_unwind(|| {
    // Code that might panic goes here
});

match result {
    Ok(_) => println!("Code did not panic"),
    Err(_) => println!("Code panicked!"),
}
```

In this example, we define a closure that contains the code we want to run. We then pass that closure to the `catch_unwind!` macro. If the code within the closure panics, the result value will be an `Err` value. If it doesn't panic, the result value will be an `Ok` value.

The `catch_unwind!` macro is not guaranteed to succeed, for example when using custom panics or aborting panics. Additionally, the `catch_unwind!` macro is not generally recommended outside of FFI purposes. To help prevent panics, Rust provides many non-panic functions, such as Vec `get` instead of slice, and `checked_add` instead of operator addition. To help documentation show panics, Rust Clippy provides the lint `missing_panics_doc`.


## macro_rules! for declarative macros

<https://doc.rust-lang.org/book/ch19-06-macros.html>

The Rust `macro_rules!` macro is a powerful code generation tool that allows the developer to create custom syntax or keywords that expand into Rust code at compile time. With this macro, you can define custom syntax rules, patterns, and templates that can be used to generate code automatically.

The `macro_rules!` macro works by defining a set of rules that match the input code, similar to a regular expression. These rules are then used to generate Rust code based on the input, which can be used to reduce the amount of repetitive or boilerplate code required for a given codebase.

Syntax:

```rust
macro_rules! my_macro_name {
  // Define patterns and templates here that match the input code
}
```

Here's an example of a simple Rust macro that generates a for loop with a range of numbers:

```rust
#[macro_export]
macro_rules! number_loop {
    ($start:expr, $end:expr) => {
        for i in $start..$end {
            println!("{}", i);
        }
    }
}
```

With this macro, you can now generate a for loop by simply invoking the number_loop! macro with the desired start value and end value:

```rust
number_loop!(0, 10);
```

This will output the numbers from 0 to 9.


# Syntax

This page is a section divider and intentionally blank.


## Annotations for compiler directives

In Rust, annotations are used to provide additional information to the compiler about how code should be compiled or optimized. Annotations are usually written as attributes and are placed above the item they apply to.

There are different types of annotations in Rust, such as `derive`, `allow`, `test`, `inline`, `cfg`, and more.

`#[derive]` automatically implements the given traits for a struct or enum, such as:

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}
```

`#[allow]` silences compiler warnings, such as:

```rust
#[allow(unused_variables)]
fn foo() {
    let x = 42;
}
```

`#[test]` marks a function as a test, so it runs with `cargo test`, such as:

```rust
#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4);
}
```

Overall, annotations in Rust provide a way to add additional information to code that can help the compiler optimize and generate better code. They are a powerful tool for controlling the behavior of the compiler and improving the performance of Rust programs.


## Destructuring into components

In Rust, destructuring is the process of taking apart a complex data structure (such as a tuple, struct, or enum) into its individual components.

Destructuring a tuple:

```rust
let my_tuple = (1, 2);
let (a, b) = my_tuple; // Assign a = 1, b = 2
```

Destructuring struct fields:

```rust
struct MyStruct {
    x: i32,
    y: String,
}

let my_struct = MyStruct { x: 42, y: String::from("hello") };
let MyStruct { a, b } = my_struct; // Assign a = 42, b = "hello"
```

Destructuring an enum variant:

```rust
enum MyEnum {
    Variant1(i32),
    Variant2(String),
}

let my_enum = MyEnum::Variant1(42);
match my_enum {
    MyEnum::Variant1(n) => println!("Got a number: {}", n),
    MyEnum::Variant2(s) => println!("Got a string: {}", s),
}
```


## Iterators for traversing collections

In Rust, iterators are abstractions for traversing collections of data, such as arrays, vectors, and other sequences. Iterators access the elements of a collection, and can be used with many of Rust's built-in language features, such as loops and closures.

Iterators in Rust are defined by the `Iterator` trait, which provides methods for traversing and manipulating a sequence of elements. Some common methods on iterators include:

* `next()`: Return the next iterator element, or None.

* `filter()`: Return the elements that match a predicate.

* `map()`: Transform each element of the iterator.

* `fold()`: Reduce elements to a value via a function.

Example to traverse a vector and sum up its elements:

```rust
let v = vec![1, 2, 3, 4, 5];
let sum = v.iter().fold(0, |acc, x| acc + x);
println!("The sum is: {}", sum);
```

In this example, we create a vector `v` and use the `iter()` method to create an iterator over its elements. We then use the `fold()` method to iterate over the elements, and accumulate the sum of all the elements.

Iterators can also be used in loops, as in the following example:

```rust
let v = vec![1, 2, 3, 4, 5];
for i in v.iter().map(|x| x * 2) {
    println!("{}", i);
}
```

In this example, we create an iterator over the vector elements, and use the `map()` method to transform each element by doubling it. We use a `for` loop to iterate over the transformed elements, to print each one.

## Closures for anonymous functions

Closures are a type of anonymous function that can capture variables from their surrounding environment, and create self-contained units of behavior that can be passed around and reused.

Example of a closure in Rust:

```rust
let add = |a, b| a + b;
let result = add(3, 4);
```

In this example, we define a closure `add` that takes `a` and `b` and returns their sum. We call the closure with `3` and `4` and print the result.

Closures in Rust are defined using the `|` symbol to specify the arguments, followed by the body in braces `{}`. Rust's type inference system allows you to omit the types of the arguments, if they can be inferred.

Example of a closure that accesses a variable outside of it:

```rust
let x = 5;
let add_x = |y| x + y;
let result = add_x(3);
```

In this example, we define a closure `add_x` that takes an argument `y` and adds it to the variable `x` that is already defined outside of the closure. When we call the closure with argument `3`, it captures the value of `x` and returns `8`.

Example of a closure for collection iterator map function:

```rust
let numbers = vec![1, 2, 3, 4];
let squares = numbers.iter().map(|x| x * x);
```

In this example, we define a vector of numbers, then use the `iter` method to create an iterator over the vector's elements. We then use the `map` method to apply a closure that squares each element of the vector.


## Macros for metaprogramming

Rust macros are a powerful tool for metaprogramming, allowing you to write code that generates code at compile-time. Macros are defined using the macro_rules! macro, which allows you to match on patterns in the code and generate new code based on those patterns.

Rust macros can be used for a variety of tasks, such as creating domain-specific languages (DSLs), reducing boilerplate code, or implementing code generation tools.

There are two types of Rust macros: declarative macros and procedural macros.

Declarative macros (also known as "macro_rules! macros") use pattern matching to transform code. They are defined using the `macro_rules!` macro and operate on the tokens that make up the code. Declarative macros can be used to create new syntax or simplify existing syntax, and they are often used to create DSLs.

Procedural macros, on the other hand, operate on the AST (abstract syntax tree) of the code. They are defined using Rust's proc_macro API and allow you to write code that generates new code at compile-time. Procedural macros can be used to implement custom derive macros, attribute macros, and function-like macros.

Example of a declarative macro:

```rust
macro_rules! greet {
    (to $name:ident) => {
        println!("Hello, {}!", stringify!($name));
    };
}
```
This macro takes a value, in this case `name`, and generates a custom greeting message for it.


## Panic and how to handle it with a hook

In Rust, a `panic` occurs when a program encounters a situation where it cannot continue to run safely. This can happen for a variety of reasons, such as a failed assertion, an out-of-bounds array access, or an attempt to unwrap a `None` value. When a `panic` occurs, Rust will unwind the stack and search for a `catch_unwind` block that can handle the panic. If no such block is found, the program will terminate with an error message.

By default, Rust will print an error message and terminate the program when a panic occurs. However, it is possible to customize this behavior by adding a `panic` hook. This allows you to define your own `panic` handler that can log the error, send an alert, or perform other actions before terminating the program.

You define a `panic` hook by calling the `std::panic::set_hook` function. Pass a closure that takes a `PanicInfo` struct as an argument; the struct contains useful debugging information.

Example of a panic hook that logs the message then terminates:

```rust
use std::panic;

fn main() {
    panic::set_hook(Box::new(|panic_info| {
        let message = panic_info
            .payload()
            .downcast_ref::<String>()
            .unwrap_or(&"Unknown error".to_string());
        eprintln!("Panic occurred: {}", message);
    }));
    panic!("Yikes!"); // Deliberately trigger a panic
}
```

This sets a `panic` hook that logs the `panic` message to the standard error stream using the `eprintln` macro. When the program encounters a `panic!` macro, it will trigger the panic hook and log the error message before terminating the program.


## Pass by value or reference

Passing a value to a function can be done by value or by reference.

Pass by value means that a copy of the original value is passed to the function. Any changes made to the value inside the function will not affect the original value. In other popular languages, pass by value is typical for passing primitive data types.

Pass by reference means that a reference to the original value is passed to the function instead of a copy. This allows the function to modify the original value, as it has access to the actual memory location of the value. In other popular languages, pass by reference is typical for passing object data type pointers.

For example:

```rust
fn increment_with_pass_by_value(num: i32) {
    num + 1;
}

fn increment_with_pass_by_reference(num: &mut i32) {
    *num += 1;
}

fn main() {
    let mut x = 1;
    increment_with_pass_by_value(x);
    println!("x is {}", x); // x is still 1
    increment_with_pass_by_reference(&mut x);
    println!("x is {}", x); // x is now 2
}
```

One of the advantages of Rust is compiler warnings and help. In the pass by value function, the compiler detects that the function result is never used, and shows warnings such as "the arithmetic operation produces a value" and "note: #[warn(unused_must_use)] on by default" and "help: use let _ = ... to ignore the resulting value".


## Range syntax for a sequence of values

In Rust, a range is a way to represent a sequence of values between a start and end point. A range are defined using the syntax `start..end`, where `start` is the first value in the range, and `end` is the first value not in the range.

Here are some examples of Rust ranges:

```rust
let a = 0..10;  // range from 0 to 9 inclusive
let b = 1..=10; // range from 1 to 10 inclusive
let c = ..5;    // range from start to 5 exclusive
let d = 5..;    // range from 5 to infinity
```

Ranges can be used in many contexts in Rust, such as in for loops:

```rust
for i in 0..10 {
    println!("{}", i);
}
```

This will print the numbers from 0 to 9.

Ranges can also be used with various methods provided by the `Iterator` trait, such as `map`, `filter`, `fold`, and more:

```rust
let nums = (0..10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .collect::<Vec<_>>();
// nums is now [0, 4, 8, 12, 16]
```

This creates a range from 0 to 9, filters out any odd numbers, doubles the remaining even numbers, and collects them into a vector.

Overall, Rust ranges are a flexible and convenient way to represent sequences of values, and they are widely used throughout the language.


## Tuples for ordered collections

In Rust, a tuple is an ordered collection of values with a fixed length. Tuples can contain values of different types and are represented using parentheses with the values separated by commas.

Example of a tuple containing a string and an integer:

```rust
let person = ("Alice", 30);
```

This defines a tuple called `person` containing the string "Alice" and the integer `30`. Tuples can be assigned to variables, passed as function arguments, and returned as function results, just like any other value.

You can access individual elements of a tuple using dot notation and the index of the element you want to access, starting from zero:

```rust
let name = person.0;
let age = person.1;
```

Tuples are often used to return multiple values from a function. For example, the `std::fs::metadata` function returns a tuple that contains information including a file's length and permissions:

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    let metadata = fs::metadata("file.txt")?;
    let (len, perms) = (
        metadata.len(),
        metadata.permissions(),
    );
    println!("File len:{}, permissions:{}", len, permissions);
    Ok(())
}
```


# Memory

This page is a section divider and intentionally blank.


## Memory lifetimes

Rust has strict rules for memory management, which includes the concept of memory lifetimes. A memory lifetime is the duration for which a particular piece of memory is valid and can be accessed. Lifetimes can be defined in function signatures, struct definitions, and other code constructs. The borrow checker enforces rules around memory lifetimes, to ensure that memory is accessed safely and without any undefined behavior.

Memory lifetimes are determined by the ownership and borrowing system. Every value has an owner, which is responsible for allocating and freeing the memory associated with the value. When a value is borrowed, the borrower is given a reference to the memory owned by the owner. The borrower must return the reference before the owner goes out of scope, or else the program will not compile.

For example, consider the following code:

```rust
fn main() {
    let x = 5;
    let y = &x;
    println!("{}", y);
}
```

Here, `x` is an integer with a value of 5. The `&` operator creates a reference to `x` and assign it to `y`. The `println!()` macro prints the value of `y`. The lifetime of `x` begins when it is created and ends when it goes out of scope at the end of the `main()` function. The lifetime of `y` is the same as the lifetime of `x`, because it is a reference to the memory owned by `x`. The borrow checker ensures that `y` is returned before `x` goes out of scope.

Memory lifetimes are strict, and can be complex to learn, because they help ensure that programs are safe and free from undefined behavior, and enable high-performance memory-safe code without the need for garbage collection or other runtime memory management.


## Implicit lifetimes

Lifetimes can be implicit, meaning they do not need notations, or explicit, meaning they do need notations.

Implicit lifetimes are typical, because the Rust compiler can figure out lifetimes for many typical needs, which means the developer doesn't need to write the lifetimes.

Suppose a function has one input arg and an output reference:

```rust
fn trim_period(s: &String) -> &str {
    s.trim_matches('.')
}
```

The compiler can figure out lifetimes by using the function signature:

* The compiler sees the output string slice, and knows that the string slice memory must come from somewhere.
  
* The compiler sees the function has only one input arg, so knows that the memory must come from the input arg. 
  
* Therefore the compiler can automatically set the lifetimes: the input arg must live at least as long as the output string slice.

The compiler's implicit lifetimes are equivalent to these explicit lifetimes:

```rust
fn trim_period<'a>(s: &'a String) -> &'a str {
    s.trim_matches('.')
}
```


## Explicit lifetimes

Explicit lifetimes are atypical, and only needed when the Rust compiler can't figure out the lifetimes. For this, the developer must write explicit lifetimes so the compiler knows what to do.

Suppose a function has two input args and an output reference:

```rust
fn trim_period(s: &String, msg: &String) -> &str {
    println!(msg);
    s.trim_matches('.')
}
```

The compiler cannot figure out the lifetimes by using the function signature:

* The compiler sees the output string slice, and knows that the string slice memory must come from somewhere.
  
* The compiler sees the function has two input args, so knows that the memory must come from either of the input args, or possibly from a combination of them. 
  
* Therefore the compiler cannot automatically set lifetimes: it's unknown which input arg must live at least as long as the output string slice, or if there's something else involved from a combination.

The compiler requires the developer to write explicit lifetimes:

```rust
fn trim_period<'a>(s: &'a String, msg: &String) -> &'a str {
    println!(msg);
    s.trim_matches('.')
}
```

The explicit lifetimes instruct the compiler that the output string slice memory comes from the first input arg, not the second input arg.


## Memory on the stack or the heap

In Rust, memory is typically allocated either on the stack or the heap. The stack and heap are two different regions of memory that are used for different purposes.

The stack is a region of memory that is used for storing local variables and function call frames. Each time a function is called, a new stack frame is created to store the function's local variables and other data. When the function returns, its stack frame is destroyed, and the memory used by the stack frame is released.

Stack allocation is fast and efficient, because memory for a stack frame is allocated when the function is called, and released when the function returns. Stack allocation doesn't require any runtime overhead, making it an excellent choice for small, short-lived objects.

On the other hand, the heap is a region of memory that is used for dynamically allocated data. Data allocated on the heap persists until it is explicitly deallocated. Heap allocation can be slower and less efficient than stack allocation, because it requires additional runtime overhead to allocate and deallocate memory.

In Rust, heap allocation is typically done using the Box type, which creates a pointer to a value that is stored on the heap. For example:

```rust
fn main() {
    let x = Box::new(5);
    println!("{}", x);
}
```

Here, `x` is a pointer to a value of 5 that is stored on the heap. The `Box::new()` function allocates memory on the heap and returns a pointer to the allocated memory. The `println!()` macro prints the value of `x`.


## Memory ownership and borrowing

Rust uses a unique system for managing memory called "ownership". Ownership is a key concept in Rust, which helps ensure memory safety and prevents many common programming errors such as null pointer dereferencing, use-after-free, and data races.

In Rust, each value has an owner, which is responsible for managing the memory associated with that value. When a value is created, its ownership is assigned to the variable that holds it. Ownership can then be transferred to another variable, passed as a function argument, or returned from a function. When the variable that owns a value goes out of scope, the value is automatically deallocated.

This ownership model allows Rust to guarantee memory safety at compile-time, without the need for a garbage collector or manual memory management. It does so by enforcing a set of rules that ensure that each value is owned by only one variable at a time, that ownership can be transferred but not shared, and that every value is deallocated exactly once when it goes out of scope.

In addition to ownership, Rust also provides a mechanism for borrowing, which allows multiple variables to have temporary access to a value without taking ownership of it. This allows for efficient and flexible memory management, while still ensuring that memory safety is maintained.

Memory ownership and borrowing are important concepts in Rust, and they help ensure that Rust code is both efficient and safe. By carefully managing memory ownership and borrowing, Rust programmers can write code that is fast, reliable, and secure.

Overall, Rust's memory ownership model provides a powerful and safe way to manage memory in a concurrent and parallel programming environment.


## Memory ownership - example

Example of memory ownership and borrowing in Rust:

```rust
fn main() {
    let mut vec = vec![1, 2, 3];
    print_vec(&vec); // Pass a reference
    vec.push(4); // Modify the vector
    take_vec(vec); // Pass ownership
}

fn print_vec(vec: &Vec<i32>) {
    for num in vec {
        println!("{}", num);
    }
}

fn take_vec(vec: Vec<i32>) {
    println!("Took ownership of {:?}", vec);
}
```

In this example, we define a vector of integers and then pass a reference to the vector to a function called `print_vec`. The `print_vec` function borrows the reference to the vector and iterates over it, printing each element.

Next, we modify the vector by pushing another element onto it, and then pass ownership of the vector to a function called `take_vec`. The `take_vec` function takes ownership of the vector and prints a message to indicate that it has ownership of the vector.

Notice that we use the `&` operator to pass a reference to the vector to `print_vec`. This is an example of borrowing in Rust - we borrow a reference to the vector without taking ownership of it.

In contrast, when we pass the vector to `take_vec`, we don't use the `&` operator. This is an example of taking ownership in Rust - we transfer ownership of the vector to the `take_vec` function.


## Mutability and immutability

Rust provides strict control over mutable and immutable references to data. Rust's approach to mutability and immutability helps to prevent many common programming errors, such as null pointer references, race conditions, and other types of undefined behavior.

In Rust, a variable's mutability is determined by whether or not it was declared with the `mut` keyword. If a variable is declared with `mut`, it is mutable, meaning it can be changed. If it is not declared with `mut`, it is immutable, meaning it cannot be changed.

Here is an example of a mutable variable in Rust:

```rust
let mut x = 5;
x = 6; // This is allowed because x is mutable.
```

And here is an example of an immutable variable in Rust:

```rust
let x = 5;
x = 6; // This is not allowed because x is immutable.
```

Immutable variables are useful for ensuring that data remains constant and unchanging. They can help to prevent accidental modification of data and make programs easier to reason about. On the other hand, mutable variables can be useful for cases where data needs to be updated or changed.

In Rust, mutability is also closely tied to references to data. Rust uses a concept called borrowing to ensure that mutable and immutable references to data do not overlap in ways that could cause undefined behavior.

When a variable is borrowed as mutable, the borrowing function gains exclusive access to the data, meaning that no other function can access it until the mutable reference goes out of scope. Conversely, when a variable is borrowed as immutable, multiple functions can access the data at the same time, as long as they are not trying to modify it.


## No garbage collection

When a program creates objects or data in memory, the program must manage the memory. Some languages such as C rely on the developer to allocate memory and free it. Some languages such as Java use garbage collection. Rust has a unique approach that uses no garbage collection.

### What is garbage collection?

Garbage collection (GC) is a mechanism that automatically frees up memory that is no longer being used.

Garbage collection works by periodically scanning the memory used by a program to identify objects that are no longer being used. Once identified, the garbage collector frees up the memory used by these objects, making it available for future use by the program.

There are different types of garbage collection algorithms, such as reference counting, mark and sweep, and copying. Each algorithm has its strengths and weaknesses, depending on context.


### Rust doesn't use garbage collection

One of Rust’s key innovations is guaranteeing memory safety (meaning no segfaults) without requiring garbage collection. Rust avoids GC by tracking memory ownership and enforcing safety via the borrow checker.

By avoiding GC, Rust can offer numerous benefits: predictable cleanup of resources, lower overhead for memory management, and essentially no runtime system. These benefits make it easier to embed Rust into arbitrary contexts, and also easier to integrate Rust with languages that do have a GC.

For when single ownership does not suffice, Rust programs can use the standard library reference-counting smart pointer types: `Rc` for single-thread reference counting, and Arc for multi-thread reference counting.



## Borrow splitting

Borrow splitting, a.k.a. partial borrowing, is when you try to borrow in multiple ways that can interfere with each other.

This example fails to compile because of borrow splitting:

```rust
// Create a typical struct
struct Foo {
    a: i32,
    b: i32,
}

// Create mutabale accessors
impl Foo {
    pub fn a_mut(&mut self) -> &mut i32 {
        &mut self.a
    }
    pub fn b_mut(&mut self) -> &mut i32 {
        &mut self.b
    }
}

// Compile succeeds because `a` and `b` are independent
pub fn increment(a: &mut i32, b: &mut i32) {
    *a = *a + 1;
    *b = *b + 1;
}

// Compile error because `a` and `b` are borrow splitting:
// cannot borrow `*self` as mutable more than once at a time
impl Foo {
    pub fn increment(&mut self) {
        let a = self.a_mut();
        let b = self.b_mut();
        *a = *a + 1;
        *b = *b + 1;
    }
}
```


# Testing

This page is a section divider and intentionally blank.


## Test framework

Rust has a built-in testing framework that allows developers to write and run automated tests for their Rust code. The testing framework is designed to be easy to use, and it supports a wide range of testing scenarios, including unit tests, integration tests, and benchmark tests.

To write tests in Rust, developers create test functions that are annotated with the #[test] attribute. These functions can contain one or more test assertions that check whether a particular condition is true or false. If all assertions in a test function pass, the test is considered to have passed. If any assertion fails, the test is considered to have failed.

Here's an example of a simple test function in Rust:

```rust
#[test]
fn test_addition() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
```

In this example, the `test_addition` function tests whether the addition of two numbers results in the expected value. The `assert_eq!` macro compares the result of the addition with the expected value of 4. If the addition results in anything other than 4, the assertion will fail, and the test will fail.

To run tests in Rust, developers use the `cargo test` command, which runs all tests in a Rust project and reports the results. The cargo test command can also be used to run specific tests or groups of tests, and it provides a range of options for controlling the behavior of the testing framework.

In addition to unit tests, Rust's testing framework also supports integration tests, which test the interaction between different modules or components of a Rust application, and benchmark tests, which measure the performance of Rust code under different conditions.


## Test assertions

The Rust testing framework provides macros for test assertions, such as:

* `assert!(condition)`: assert `condition` is true.

* `assert_eq!(a, b)`: assert `a` is equal to `b`.

* `assert_ne!(a, b)`: assert `a` is not equal to `b`.

Example:

```rust
let x = 1;
let y = 2;
assert!(x < y);
```

Example with an optional message:

```rust
let x = 1;
let y = 2;
assert!(x < y, "We want x to be less than y");
```


### Assertables crate

The Assertables crate provides more assert macros, such as:

* `assert_starts_with!(x, y)`: Does `x` start with `y`?

* `assert_contains!(array, element)`: Does `array` contains `element`?

* `assert_is_match!(regex, string)`: Does `regex` match `string`?
  
Example:

```rust
use assertables;
let a = "hello world";
let b = "hello";
assert_starts_with!(&a, &b);
```


## Unit testing

Unit testing is a software testing technique where individual software components or units are tested in isolation to ensure that they behave as expected. In Rust, unit testing involves writing tests that validate the expected behavior of functions, methods, and other individual units of code.

Rust provides a built-in testing framework for unit testing called `cargo test`.

* Unit tests are typically placed in the same file as the code they are testing. These tests should be written to validate the expected behavior of each function and method.

* Use the `#[cfg(test)]` attribute indicates that a Rust module contains tests.

* Use assertions, such as the Rust standard library `assert_eq!` assertion, or Assertables crate `assert_starts_with!` assertion.

* Unit tests in Rust can be run using the `cargo test` command. This command compiles and runs all the tests in the project, including the unit tests.

* After the tests have run, the output of the tests can be analyzed to determine whether the unit tests have passed or failed. Rust's testing framework provides detailed information about the tests that have been run, including the number of tests that have passed or failed and the reason for the failures.

By following these steps, developers can use Rust's unit testing framework to validate the behavior of individual components of the software, ensuring that each unit behaves as expected and functions correctly as part of the larger system.


## Integration testing

Integration testing is a software testing technique where individual software modules are tested as a group to validate their combined functionality. In Rust, integration testing involves testing the interactions between different modules or components of the software.

Rust provides a built-in testing framework for integration testing called `cargo test`. Here are the steps involved in Rust integration testing:

* Create a separate directory for integration tests: Integration tests in Rust are typically placed in a separate directory called `tests` at the top level of the project. This directory contains Rust files that end with `_test.rs`.

* Write the integration tests: Integration tests in Rust are similar to unit tests but test the interaction between different modules or components. These tests should be written to validate the expected behavior of the system as a whole.

* Use Rust's testing framework: Rust's testing framework provides a set of macros and functions for writing and running tests. The `#[cfg(test)]` attribute indicates that a Rust module contains tests.

* Run the tests: Integration tests in Rust can be run using the `cargo test` command. This command compiles and runs all the tests in the project, including the integration tests.

* Analyze the test results: After the tests have run, the output of the tests can be analyzed to determine whether the integration tests have passed or failed. Rust's testing framework provides detailed information about the tests that have been run, including the number of tests that have passed or failed and the reason for the failures.

By following these steps, developers can use Rust's integration testing framework to validate the interactions between different modules or components of the software, ensuring that the software functions correctly as a whole.

## Documentation testing

Rust doc tests are a form of Rust's testing framework that allows developers to include tests in the documentation of their code. This enables developers to write code examples and tests in the documentation itself, ensuring that the documentation remains up-to-date and accurate.

Example:

```rust
/// This is a document comment with a doc test.
///
/// This doc test must succeed.
/// 
/// ```
/// assert!(true);
/// ```
```

To run all the doc tests:

```sh
cargo test --doc
```

To also show warnings:

```sh
cargo test --doc -- --show-output
```

Rust doc tests have a variety of options to make them more powerful and more flexible.

* Annotations enable you to specify code blocks that should be ignored, or should panic, or should be compiled but not run. 

* Embedded comments enable you to write code that is hidden, so your documentation is shorter and more readable.
  
* Trailing returns enable you to skip lengthy error handling, and instead use `?` error returns.


## Documentation testing annotations

Documentation comment code blocks can use annotations with attributes that help `rustdoc` do the right thing when testing your code. Here are the annotations.

This test must panic:

```rust
/// ```should_panic
/// assert!(false);
/// ```
```

This test must compile, but is not run:

```rust
/// ```no_run
/// assert!(true);
/// ```
```

This test must fail to compile:

```rust
///  ```compile_fail
/// snafu
/// ```
```

This test is only for Rust 2018 edition:

```rust
/// ```edition2018
/// assert!(true);
/// ```
```

This code block is ignored, and not a test:

```rust
///  ```ignore
/// This is something else besides a test.
/// ```
```

This code block is text, and not a test:

```rust
///  ```text
/// Hello, World!
/// ```
```


## Source-based code coverage

<https://doc.rust-lang.org/rustc/instrument-coverage.html>

In Rust, source-based code coverage is a way of measuring how much of a Rust codebase is executed during a test suite. This type of coverage analysis works by instrumenting the Rust code and tracking which lines of code are executed during a test run.

The process of generating source-based coverage typically involves the following steps:

* Instrumentation: The Rust code is modified to include extra code that tracks which lines of code are executed.

* Test Execution: The test suite is run against the instrumented code.

* Coverage Report Generation: The data collected during the test run generates a report that shows which lines of code were executed and which were not.

The resulting coverage report provides developers with insights into the effectiveness of their tests and helps identify areas of the code that are not being sufficiently exercised by the test suite.

One key advantage of Rust source-based coverage is that it can provide more accurate coverage measurements than alternative methods, such as binary-based coverage. This is because source-based coverage is able to account for control structures, such as branches and loops, which can lead to different paths through the code being executed.

To run unit tests with coverage:

```sh
RUSTFLAGS="-C instrument-coverage" cargo test --tests
```

After the tests run, there are a variety of ways to use the output files and view the coverage reports. The steps are detailed, so please see the link above for specifics.


## Test-driven development (TDD)

Test-driven development (TDD) is a software development approach where tests are written before the code that will be tested. The goal of TDD is to create higher quality, more maintainable code by ensuring that code is written to pass tests that validate the intended behavior.

In Rust, TDD involves creating tests that ensure that the code functions correctly and provides the expected output. Here are the steps involved in Rust TDD:

* Write a failing test: The first step is to write a test that validates the intended behavior of the code. This test should fail, indicating that the code does not yet meet the desired behavior.

* Write the simplest code possible to pass the test: After writing the failing test, write the simplest code possible to make the test pass. This code should be written with the goal of passing the test, not creating a complete solution.

* Refactor the code: After the test passes, refactor the code to improve its design and readability, while still ensuring that the test continues to pass.

* Repeat the process: Continue this process of writing failing tests, writing the simplest code possible to pass the test, and refactoring the code until the code meets the desired behavior.

In Rust, TDD can be implemented using Rust's built-in testing framework. This framework allows developers to write tests using Rust's macro syntax and provides a set of assertions that can be used to validate the output of the code being tested.

By following the TDD approach in Rust, developers can create code that is reliable, maintainable, and easier to understand, while also reducing the number of bugs and issues that arise during development.

## Test-driven development - example

For this test-driven development example, imagine you want to write a function `foo` that always returns `true`.

First write a test:

```rust
#[cfg(test)]           // Annotation: the mod is for cargo test
mod tests {            // Define a module named "tests"
    use super::*;      // Use code from the outer module

    #[test]            // Annotation: this function is a test
    fn foo_test() {    // Define a function as usual
      assert!(foo());  // The assert! test macro must be true
    }

}
```

Verify the test failure:

```sh
cargo test
```

Write the simplest code possible to pass the test:

```rust
pub fn foo() -> bool { // Define a function
    true // Always return true
}
```

Verify the test success:

```sh
cargo test
```

Rust has built-in test assertion macros such as `assert`, `assert_eq`, `assert_ne`. In practice, these are fine for simple TDD, but may be too basic for real-world TDD. In our real-world projects, we use the Assertables crate that provides many more assertions such as `assert_starts_with`, `assert_contains`, and `assert_is_match`.

<https://crates.io/crates/assertables>


# Examples

This page is a section divider and intentionally blank.


## Access a database with rusqlite

Rust example to connect to a SQLite database and execute SQL queries, by using the `rusqlite` crate:

```rust
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open(":memory:")?;
    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            age   INTEGER NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "INSERT INTO person (id, name, age) VALUES (?1, ?2, ?3)",
        ["1", "Alice", "30"],
    )?;
    let name: String = conn.query_row(
        "SELECT name FROM person WHERE id=1",
        [],
        |row| row.get(0),
    )?;
    println!("{}", name);
    Ok(())
}
```

This example creates a `Connection` to a SQLite database in memory, creates a table named "people", inserts data into it, selects data from it, and prints it out. The `rusqlite` crate provides many more SQL features, such as transactions and prepared statements.


## Benchmark times with Bencher

Benchmarking is the process of measuring the performance of code by running it multiple times under different conditions to identify areas where improvement can be made. It is an essential process for optimizing Rust code for faster execution and better resource utilization.

To conduct Rust benchmarking, the following steps are usually taken:

1. Identify the code or function to be benchmarked

2. Write a benchmarking harness to execute the code multiple times and record metrics such as execution time and memory usage.

3. Run the benchmark multiple times to obtain a baseline performance metric

4. Identify areas of improvement in the code and make changes to optimize its performance

5. Repeat the benchmarking process after making modifications to gauge the impact on performance.

A typical way is:

1. Create a top-level folder named `benches`.

2. Create a typical Rust file with your own function.

4. Annotate the function with the `#[bench]` attribute.

Rust provides built-in support for benchmarking through its libtest framework, and the function annotation `[#bench]`.

As of this writing (2023-03-23) the Rust nightly channel has benchmarking as a feature, whereas the Rust stable channel does not. See below for troubleshooing information about this.

For simple benchmarks, you can use the Bencher crate, which is a simple Rust-stable-compatible benchmark runner. For real-world projects, we suggest the Criterion crate, which is newer, more popular with current Rust teams, and provides more capabilities.


## Benchmark times - example

Edit file `Cargo.toml`, and add a dependency and configuration:

```toml
[dev-dependencies]
bencher = "*"

[[bench]]
name = "example"
harness = false
```

Create a project top-level directory `benches` then edit file `benches/example.rs`:

```rust
#[macro_use]
extern crate bencher;
use bencher::Bencher;

fn a(bench: &mut Bencher) {
    bench.iter(|| {
        (0..1000).fold(0, |x, y| x + y)
    })
}

fn b(bench: &mut Bencher) {
    const N: usize = 1024;
    bench.iter(|| {
        vec![0u8; N]
    });
    bench.bytes = N as u64;
}

benchmark_group!(benches, a, b);
benchmark_main!(benches);
```

Run `cargo bench` and you should see output such as:

```text
running 2 tests
test a ... bench:           0 ns/iter (+/- 0)
test b ... bench:          31 ns/iter (+/- 1) = 33032 MB/s
```


## Liskov substitution principle (LSP)

The Liskov substitution principle (LSP) is a fundamental principle of the object-oriented programming paradigm, and it states that any instance of a class must be replaceable with an instance of its derived classes without affecting the correctness of the program.

In the context of Rust, this principle can be seen in action through the use of trait objects. A trait in Rust is similar to an interface in other programming languages, and it defines a set of methods that a type must implement. A trait object, on the other hand, is a value that can hold any type that implements the trait.

Using trait objects in Rust ensures that the Liskov substitution principle is upheld. Since any type that implements the trait can be used interchangeably, it becomes easier to modify, extend, and re-use code. This approach also enhances the flexibility of Rust's type system since it makes it possible to store a collection of different types that share a common trait.

Overall, Rust's use of trait objects promotes a secure and robust codebase by guaranteeing at compile-time that any substitution of instances within classes is well-suited to run the program correctly.



## Liskov substitution principle - example

Example:

```rust
trait Drawable {
    fn draw(&self);
}

fn draw_anything(drawable: &dyn Drawable) {
    drawable.draw();
}

struct Circle {
    radius: i32,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Circle with radius {}", self.radius);
    }
}

fn main() {
    let circle = Circle { radius: 1 };
    draw_anything(&circle);
}
```

This defines a struct `Circle` and implements the `Drawable` trait. The `draw_anything` function takes any object that implements the `Drawable` interface, which means that it can accept circles or anything else that implements `Drawable`. interface. The function is an example of Liskov substitution principle in action, because any `Drawable` can be given   .


## List directories recursively with walkdir

Rust example code to list directories recursively with the walkdir crate.

Example:

```rust
use walkdir::WalkDir;

fn main() {
    let path = "/usr";
    for entry in WalkDir::new(path)
    .max_depth(3)
    .into_iter()
    .filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            println!("Directory: {}", entry.path().display());
        } else {
            println!("File: {}", entry.path().display());
        }
    }
}
```

The example function does these steps:

1. Start at the system directory path "/usr".

2. Create a WalkDir object. Limit the walk to maximum depth of 3 directories. Use an iterator. Filter-map the results to be entries that are ok.

3. For each entry, see if it's a directory or file, and print its path, such as "Directory: /usr/bin" or "File: /usr/bin/true", etc.


## Make HTTP request with reqwest

Rust example code to make an HTTP GET request to a URL and print the response body, with the `reqwest` crate.

In the file `Cargo.toml`, add `reqwest` and `tokio` for async functions:

```toml
[dependencies]
reqwest = "*"
tokio = { version = "*", features = ["full"] }
```

`main.rs`:

```rust
use reqwest::Error;

async fn fetch(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://www.example.com";
    let response_body = fetch(url).await?;
    println!("{}", response_body);
    Ok(())
}
```

This code defines an asynchronous function `fetch` that takes a URL as input and returns a Result containing the response body as a `String` if the request succeeds. The function uses the `reqwest::get` function to make an HTTP GET request to the specified URL, and then uses the text method of the response object to extract the response body as a string.

In the main function, we call `fetch` with a URL and then print the response body to the console. Note that this code assumes that the URL is valid and that the server responds with a successful HTTP status code. Also, we use `#[tokio::main]` attribute to execute our async main function, as we are using `async-await` in our `fetch` function.


## Memoize a function with cached

The phrase "memoize a function" means "create a cache of the function's inputs and outputs". This is called memoizaiton, and can significantly speed up some functions, especially ones that do large calculations, or ones that do recursive calculations.

An easy way to memoize a function is to use the Rust `cached` crate. 

Example:

```rust
use cached::proc_macro::cached;

#[cached]
fn fibonacci(n: usize) -> usize {
    if n == 0 || n == 1 { return n }
    fibonacci(n-1) + fibonacci(n-2)
}
```

The example defines a function named `fibonacci`. The Fibonacci sequence is a mathematical sequence in which each number is the sum of the two preceding ones: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, etc. For example, index number `0` is the sequence term `0`, and index number `12` is the  sequence term `144`. 

The `fibonacci` function takes an input which is a Fibonacci sequence index number, and returns the Fibonacci sequence term number. For example, input index `0` returns term `0`, and input index  `12` returns term `144`.

By default, the function's memoizaiton cache will be the function's name in all caps i.e. `FIBONACCI`.


## Parallelism with rayon

Rust has built-in support for parallelism, which is the ability to execute multiple tasks simultaneously on multiple processors or cores.

Rust's support for parallelism is especially easy to use by adding the Rust `rayon` crate, which provides a high-level API for parallel programming. The `rayon` crate allows developers to easily parallelize data processing tasks, such as iterating over large collections, by abstracting away the low-level details of thread creation and synchronization.

Here is an example code snippet that demonstrates Rust parallelism using rayon:

```rust
use rayon::prelude::*;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = numbers.par_iter().sum::<i32>();
    println!("Sum is {}", sum);
}
```

In this example, the `par_iter()` method creates a parallel iterator over a vector of numbers. The `sum()` method is then called on the iterator to calculate the sum of all the numbers in the vector.

`rayon` automatically divides the work among multiple threads, using as many threads as there are processors or cores available on the system. The code is executed in parallel, with each thread processing a subset of the data.

The `par_iter()` method can be used with many other methods of the standard library, such as `map()`, `filter()`, and `reduce()`, to parallelize various data processing tasks.


## Parse JSON data with Serde

Example code to parse JSON data, by using the `serde_json` crate.

```rust
use serde_json::{Result, Value};

fn parse_json(json_string: &str) -> Result<Value> {
    let json: Value = serde_json::from_str(json_string)?;
    Ok(json)
}

fn main() {
    let json_string = r#"
        {
            "name": "John Doe",
            "speaks": ["English", "Mandarin"]
        }
    "#;

    let parsed_json = parse_json(json_string).unwrap();
    let name = parsed_json["name"].as_str().unwrap();
    let languages = parsed_json["speaks"].as_array().unwrap();
    println!("Name: {}", name);
    println!("Speaks: {:?}", languages);
}
```

This code defines a function `parse_json` that takes a JSON string and returns a `serde_json::Value` object. The `serde_json::from_str` function parses the JSON string into a `Value` object. The main function demonstrates how to access the values in the parsed JSON by using the `as_*` methods on the `Value` object. In this example, we access the `name`, `age`, and `speaks` fields of the JSON object and print them to the console. This code assumes that the JSON is well-formed, and matches the expected schema.


## Read a spreadsheet with CSV

Rust example code to read CSV file, by using the `csv` crate:

```rust
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data.csv";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(contents.as_bytes());

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
```

This code reads a CSV file located at `data.csv`, reads its contents into a string, and then uses the `csv` crate's `Reader` to parse the CSV data. The `has_headers` method specifies that the CSV file contains a header row, and the delimiter method specifies that the field separator is a comma.

The for loop iterates over each record in the CSV file and prints it to the console. Each record is represented as a `csv::StringRecord`, which can be indexed or iterated over to access individual fields. The `?` operator is used throughout the code to handle errors that may occur during file I/O or CSV parsing.


## Run a terminal program with cursive

Run a simple interactive terminal user interface program, by using the `cursive` crate.

```rust
use cursive::{Cursive, CursiveExt};
use cursive::views::{Dialog, TextView};

fn main() {
    let mut siv = Cursive::default();

    siv.add_layer(
        Dialog::around(TextView::new("Hello, World!"))
            .title("Cursive Example")
            .button("Quit", |s| s.quit()),
    );

    siv.run();
    println!("Ok")
}
```

This code creates a `Cursive` object, adds a `TextView` containing the message "Hello, World!" to a `Dialog`, and then displays the dialog with a "Quit" button that will close the application when clicked.

Add the `cursive` crate dependency to the `Cargo.toml` file, then you can run this code using `cargo run`.

## Search text file lines with regex

Example to search text file lines by using the `regex` crate for regular expression pattern matching.

```rust
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    // Regular expression: word break + letters "in" + word break
    let regex = Regex::new(r"\bin\b").expect("regex");

    // Open an existing file and prepare to read its lines
    let file = File::open("example.txt").expect("file");
    let lines = io::BufReader::new(file).lines();

    // For each line, try the regex; if it matches, then print the line.
    for line in lines {
        if let Ok(string) = line {
            if regex.is_match(&string) {
                println!("match: {}", string);
            }
        }
    }
}
```

The example opens a text file, then reads each line. The example loop tries the regex `is_match` method on each line's string. If the string matches, then the function prints it.



# Tooling we use often

This page is a section divider and intentionally blank.


## rustup command-line tool

In Rust, `rustup` is a command-line tool that manages the installation and configuration of Rust toolchains. A Rust toolchain is a set of tools and libraries that are used to compile and run Rust programs.

`rustup` installs and updates Rust toolchains, including the Rust compiler and associated tools such as `cargo`. It also allows for the management of multiple toolchains and makes it easy to switch between them.

Some of the commonly used rustup commands include:

* `rustup install`: Installs a specific version of the Rust toolchain.

* `rustup default`: Sets the default Rust toolchain to use.

* `rustup update`: Updates the Rust toolchain to the latest stable release.

* `rustup self update`: Updates rustup itself to the latest version.

* `rustup component add`: Adds a component to the Rust toolchain, such as a specific target or a specific version of rustfmt.

* `rustup target add`: Adds a new target to the Rust toolchain, such as armv7-unknown-linux-gnueabihf for cross-compiling to an ARM-based Linux system.

* `rustup toolchain list`: Lists all installed Rust toolchains.

* `rustup override`: Sets a toolchain override for a specific directory or project.

`rustup` also allows for the installation of Rust-related components such as the `rust-src` component, which includes the source code for the Rust standard library, or the `rls` component, which provides support for Rust language server integration.

Overall, rustup is a powerful tool that makes it easy to manage Rust toolchains, enabling Rust developers to work with multiple versions of Rust and target different platforms.


## Cargo package manager and crates

In Rust, Cargo is the package manager and build tool that creates and manages projects and their dependencies. Cargo provides ways to easily build, test, document, and publish code.

Cargo uses a file called `Cargo.toml` to manage the configuration and dependencies of a Rust project. The `Cargo.toml` file specifies the name of the package, version information, and the dependencies of the project. Cargo also provides a command-line interface that allows developers to manage their Rust projects and dependencies easily.

A cargo package is called a "crate". A crate can be a binary or a library. A binary crate is an executable program. A library crate is code that can be used by other programs.

Cargo provides a standardized directory structure for Rust projects. By convention, the main source code of a project is placed in a directory called src, and the project configuration and dependencies are specified in a file called `Cargo.toml`. Cargo uses the `Cargo.lock` file to keep track of exact dependency versions used in the project.

Cargo also provides a number of commands to manage a Rust project. Some of the commonly used commands include:

* `cargo new`: Create a new Rust project.

* `cargo build`: Build the project and its dependencies.

* `cargo run`: Build and run the project.

* `cargo test`: Run the project tests.

* `cargo doc`: Generates documentation for the project.

* `cargo publish`: Publishes a crate to the official registry.


## cargo-install-favorites

<https://github.com/sixarm/cargo-install-favorites>

The `cargo-install-favorites` shell script is a list of our favorite Rust projects for use on our daily machines, such as enhanced command line utilities. Here are highlights.

**bat**: Show terminal text with highlights, git integration, fzf. Like `cat`.

**bottom**: A graphical process/system monitor for the terminal. Like `top`.

**broot**: A file manager with better ways to navigate directories. Like `tree`.

**difftastic**: Compare files via syntax, alignments, etc. Like `diff`.

**du-dust**: Show disk usage, with trees, colors, rollups, and more. Like `du`.

**exa**: Examine file lists, with colors, attributes, git awareness. Like `ls`.

**fd-find**: A simple, fast and user-friendly alternative to Unix `find`.

**gitui**: Blazing fast terminal user interface for git. Like `git`, `gitk`.

**gping**: Graphical ping network tracer, plus multiple hosts. Like `ping`.

**helix**: Terminal text editor, with modern capabilities built-in. Like `vim`.

**just**: Command runner for project-specific tasks. Like `make`.

**procs**: Monitor system processes, with colors, search, extras. Like `ps`.

**ripgrep**: Fast flexible regular expression text search tool. Like `grep`.

**starship**: Fast, minimal, infinitely customizable shell prompt.

**watchexec-cli**: Watch files for modifications then execute commands.

**zellij**: Terminal workspace with batteries included. Like `screen`, `tmux`.

**zoxide**: A faster way to navigate your directories. Like `cd`, `jump`.


## Blessed recommendations

<https://blessed.rs>

Blessed is an unofficial guide to the Rust ecosystem. New Rust developers frequently ask which tools and crates to use and trust.

Blessed aims to answer these questions with listings such as:

* General purpose: [regex](https://crates.io/crates/regex), [Serde](https://crates.io/crates/serde), [tempfile](https://crates.io/crates/tempfile), etc.

* Developer tooling: [Clippy](https://crates.io/crates/clippy), [Criterion](https://crates.io/crates/criterion), [cargo-release](https://crates.io/crates/cargo-release), etc.

* Language extensions: [itertools](https://crates.io/crates/itertools), [once_cell](https://crates.io/crates/syn), [syn](https://crates.io/crates/syn), etc.

* HTTP services: [axum](https://crates.io/crates/axum), [hyper](https://crates.io/crates/hyper), [reqwest](https://crates.io/crates/), etc.

* Databases: [diesel](https://crates.io/crates/diesel), [rusqlite](https://crates.io/crates/rusqlite), [sqlx](https://crates.io/crates/sqlx), etc.

* CLIs: [clap](https://crates.io/crates/clap), [walkdir](https://crates.io/crates/walkdir), [ratatui](https://crates.io/crates/ratatui), etc.

* Concurrency: [flume](https://crates.io/crates/flume), [parking_lot](https://crates.io/crates/parking_lot), [rayon](https://crates.io/crates/rayon), etc.

* Graphics: [egui](https://crates.io/crates/egui), [gtk4](https://crates.io/crates/gtk4), etc.


## Clippy linting

Rust Clippy is a popular linting tool for Rust that provides additional static analysis to help catch bugs and improve code quality. It is an external tool that runs alongside the Rust compiler and analyzes Rust code to check for common programming errors, style issues, and other potential problems.

Clippy is built on top of Rust's existing linting infrastructure and provides additional lints that are not included in the standard library. These lints are organized into several categories, including:

* Correctness: These lints check for potential errors that can cause undefined behavior, such as null pointer dereferences, out-of-bounds array access, and other common issues.

* Style: These lints check for coding style issues, such as using inconsistent indentation, unnecessary parentheses, and redundant code.

* Performance: These lints check for potential performance issues, such as using slow algorithms or redundant calculations.

* Complexity: These lints check for overly complex code, such as deeply nested functions or overly complicated expressions.

* Security: These lints check for potential security vulnerabilities, such as buffer overflows, unsafe code, and other issues.

Clippy is highly customizable, allowing developers to enable or disable specific lints, customize the severity level of lints, and even create custom lints tailored to their specific needs. It is also regularly updated with new lints and improvements, making it a valuable tool for improving Rust code quality and preventing bugs.


## Helix text editor

<https://helix-editor.com/>

Helix is terminal-based text editor written in Rust, with excellent capabilities for programming in Rust. Helix is inspired by Neovim and Kakoune, and is similar in ways to vim, emacs, and nano.

Key benefits:

* Multiple selections as a core editing primitive, so commands can manipulate selections, which allows concurrent code editing.

* Tree-sitter integration, which enables better syntax highlighting, indent calculation, and code navigation.

* Powerful code manipulation to navigate and select functions, classes, comments, etc and select syntax tree nodes instead of plain text.

* Language server support provides language-specific auto completion, goto definition, documentation, diagnostics and other IDE features with no additional configuration.

* Built in Rust, for the terminal. No Electron. No VimScript. No JavaScript. Use it over ssh, tmux, or a plain terminal. Your laptop battery life will thank you.

* Modern features such as fuzzy finder to jump to files and symbols, project wide search, beautiful themes, auto closing bracket pairs, surround integration and more.


## Rustfmt for code formatting

Rustfmt is a code formatting tool for Rust programming language. It automatically reformats Rust code according to a set of predefined formatting rules, which helps developers to maintain consistent coding styles and makes it easier to read, understand and debug the code.

Rustfmt can be used as a standalone tool, or as an integrated feature within a code editor, or via a build script. It supports formatting options, including indentation style, line wrapping, brace styles, and more.

Using Rustfmt is highly recommended by the Rust community as it helps maintain a consistent coding style across a project, which in turn makes the code easier to read, maintain and understand.

To use Rustfmt, you first need to install it on your system. Rustfmt can be installed using Cargo, the package manager for Rust, by running the following command in your terminal:

```sh
cargo install rustfmt
```

You can customize the formatting rules used by Rustfmt by creating a configuration file named `rustfmt.toml` or `.rustfmt.toml` in your project directory and specifying your preferred options.

Example `rustfmt.toml` file:

```toml
comment_width = 80
format_code_in_doc_comments = true
group_imports = "StdExternalCrate"
imports_granularity = "Crate"
imports_layout = "Vertical"
indent_style = "Block"
reorder_imports = false
wrap_comments = true
```

Overall, Rustfmt is a good tool to reformat code for consistent styles.



## Rustfmt - examples

### Rustfmt as a standalone tool

You can use Rustfmt directly from the command line:

```sh
rustfmt <filename.rs>
```
This command will format the Rust code in the specified file and print the formatted output to the terminal. If you want to save the formatted output to a file, you can use the -w option followed by the filename, like this:

```sh
rustfmt -w <filename.rs>
```

### Rusfmt within a code editor

You can use Rustfmt within a code editor such as vim, emacs, Helix, and VSCode. To do this, you install a Rustfmt extension for your editor, then configure it to format your code on save or on demand.

For example, in VSCode, you can install the "Rustfmt" extension and configure it to format your code on save by adding the following line to your `settings.json` file:

```json
"editor.formatOnSave": true
```

### Rustfmt via a build script

You can use Rustfmt as a step of your build process, before compiling it. One way to do this is to create a build script by adding the following line to your `Cargo.toml` file:

```toml
[package]
build = "rustfmt <filename.rs>"
```


## Rust mdBook for documentation

Rust mdBook is a tool for creating and publishing documentation in the form of books or websites. Rust mdBook is designed for documenting Rust projects, but it can be used for any kind of documentation. Rust mdBook supports various Markdown features and more, including syntax highlighting for code blocks, table of contents generation, cross-referencing between pages, customizable themes, and documentation by using Rustdoc comments.

To install the `mdbook` tool and the `mdbook-pdf` tool:

```sh
cargo install mdbook
cargo install mdbook-pdf
cargo install mdbook-toc
```

To use Rust mdBook, you create a book directory that contains the Markdown files and any associated assets, such as images or code samples. You can then use the mdBook command-line tool to compile the book into the desired format. The resulting output can be published as a website or distributed as an eBook or PDF.

To use Rust mdBook PDF, you may need to install additional software, such as a web browser that can render PDF. Rust mdBook PDF has installation options to automatically download and install the Chromium web browser, which can render PDF. See the Rust mdBook PDF documentation for more information.

To use Rust mdBook TOC (table of contents), you can use the default markup `<!-- tod -->`, then the build will automatically generate a table of contents. See the rust mdBook TOC documentation for more information.

Overall, Rust mdBook makes it easy to create high-quality documentation that is easy to read and understand.


## Cross-compiling for multiple platforms

Cross-compiling is the process of compiling code for a platform different from the one on which the code is compiled.

Rust supports cross-compiling, which means that you can compile Rust code on one platform and generate executable code for another platform, such as Windows, Linux, or macOS.

To cross-compile Rust code, you need to install a cross-compiler toolchain for the target platform. This toolchain includes the Rust compiler, standard library, and any other dependencies required to build the code. You can install cross-compilers for different architectures using Rust's built-in tool, rustup.

Once the cross-compiler toolchain is installed, you can use the cargo command to build your Rust project for the target platform. You can specify the target platform by setting the --target option when running the cargo build or cargo run command.

For example, to build a Rust project for the ARM architecture, you would use the following command:

```sh
cargo build --target=arm-unknown-linux-gnueabihf
```

This command tells cargo to build the project for the ARM architecture using the GNU toolchain and the Hard Float ABI.

Cross-compiling Rust code can be useful for a variety of scenarios, such as building applications for embedded systems or developing software that needs to run on multiple platforms. Rust's strong type system and memory safety guarantees make it a good choice for writing cross-platform applications that require high performance and reliability.


## Rhai script

Rhai is an embedded scripting language for Rust. Rhai is a dynamically typed language with support for high-level data types such as arrays, maps, and functions. Rhai supports Rust-style ownership and borrowing, making it easy to integrate with Rust's memory management.

One of the key features of Rhai is its safety and security. Rhai enforces sandboxing by default, which means that scripts executed within a Rhai interpreter cannot access or modify the host environment. Rhai also supports a variety of security features such as timeouts, memory limits, and access controls to ensure that scripts are safe to use.

Rhai's syntax is similar to Rust's syntax, making it easy for Rust developers to learn and use. Rhai also provides a number of built-in functions and operators that simplify common scripting tasks such as string manipulation, math operations, and control flow.

Example of using Rust as an embedded language in Rhai script:

```rust
use rhai::{Engine, EvalAltResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = Engine::new();

    let result = engine.eval::<i32>("2 + 2")?;
    println!("2 + 2 = {}", result); // should print 4

    let result = engine.eval::<f64>("3.14 * 2.0")?;
    println!("3.14 * 2.0 = {}", result); // should print 6.28

    let result = engine.eval::<i32>("10 / 3")?;
    println!("10 / 3 = {}", result); // should print 3

    Ok(())
}
```

In this example, the Rhai script evaluates arithmetic expressions, and Rust performs the actual calculations. This combines Rhai's dynamic code and Rust's strong typing and optimized performance.


# Tooling concepts

This page is a section divider and intentionally blank.


## Abstract syntax tree (AST)

An abstract syntax tree (AST) is a data structure used in computer science to represent the structure of a program in a way that can be easily analyzed and manipulated by algorithms.

An AST is created by analyzing the source code of a program and breaking it down into a tree-like structure that represents its syntax.

Each node in the tree represents a syntactic construct in the program, such as a function call, a variable declaration, or an if statement. The nodes in the tree are connected by edges that represent the relationships between the constructs.

The main advantage of using an AST is that it provides a way to analyze the program's structure and behavior without having to execute the code. This makes it possible to perform tasks such as code optimization, program transformation, and error detection without having to actually run the program.

ASTs are commonly used in compilers, interpreters, and other tools that analyze or manipulate source code. For example, a compiler may use an AST to perform optimizations such as dead code elimination or loop unrolling, while a static analyzer may use an AST to detect potential security vulnerabilities or other code quality issues.

Overall, abstract syntax trees are a powerful tool for working with programs, allowing developers to reason about their structure and behavior in a way that is both precise and efficient.


## Tree-sitter parsing library

Tree-sitter is a parsing library that allows developers to create robust and efficient parsers for programming languages, configuration files, and other structured documents. It was created by Rasmus Andersson and is written in Rust.

The library uses the tree-sitter parsing algorithm, which is a powerful parsing technique that builds an abstract syntax tree (AST) for the parsed code. The AST is a tree structure that represents the structure of the code, making it easier to analyze and manipulate.

One of the key advantages of Rust tree-sitter is its speed and efficiency. It is designed to be extremely fast, allowing it to handle large codebases and parse files in real-time. It also uses incremental parsing, which means that it can efficiently update the AST as changes are made to the code.

Rust tree-sitter is also highly modular, with a simple and flexible API that allows developers to easily create custom parsers for new languages or modify existing parsers. It supports a wide range of programming languages, including C, C++, Java, Python, Ruby, and many others.

Overall, Rust tree-sitter is a powerful and flexible parsing library that can be used to create high-performance parsers for a wide range of programming languages and structured documents.


## Language Server Protocol (LSP)

Language Server Protocol (LSP) is a communication protocol between an editor or an IDE and a language server that provides language-specific features such as code completion, error checking, and symbol search.

The Language Server Protocol is used by many popular editors and IDEs, and is supported by many programming languages.

Using the Language Server Protocol, editors and IDEs can provide consistent language features across multiple programming languages and language servers, without having to implement language-specific functionality themselves. This allows for faster and more efficient development, as developers can use their preferred editor or IDE and still have access to advanced language features.

The LSP defines a set of standard JSON-RPC methods that the client and server can use to communicate. These methods include:

* `initialize`: Iinitialize the language server and configure it.

* `shutdown`: Shut down the language server.

* `textDocument/didOpen`: Notify when a document is opened.

* `textDocument/didChange`: Notify when a document is modified.

* `textDocument/completion`: Request code completion suggestions.

* `textDocument/hover`: Request information about a symbol.

* `textDocument/references`: Request references to a symbol.

The Language Server Protocol is an open standard. The protocol is implemented in a client-server architecture, where the client is an editor or IDE that supports the LSP, and the server is a language server that provides language-specific functionality.


## Static analysis for error detection

Static analysis is the process of analyzing code without executing it, to detect potential errors or issues before the code is actually run. Rust has a strong focus on static analysis, with the goal of catching as many errors as possible at compile time, before the code is even executed.

Rust's static analysis features include:

* Static typing: Rust is a statically typed language, meaning that the type of a variable is known at compile time. This helps catch many common errors, such as trying to add a string and a number, before the code is even run.

* Ownership and borrowing: Rust's ownership and borrowing system helps prevent memory errors such as null pointer dereferences or use-after-free bugs. The compiler enforces rules around how references to data are created, modified, and used, to ensure that they are safe and sound.

* Lifetimes: Rust's lifetime system helps ensure that references to data are valid for as long as they are needed. This prevents common errors such as dangling pointers or double frees.

* Macros: Rust's macro system allows developers to write code that generates other code at compile time. This can be used to perform custom static analysis or generate repetitive code automatically.

* Clippy: Clippy is a community-maintained linter for Rust that provides additional static analysis checks beyond what the compiler itself does. Clippy checks for common coding mistakes, such as unused variables, and provides suggestions for how to fix them.

Overall, Rust's strong focus on static analysis helps catch many errors before they occur, reducing the likelihood of bugs and making it easier to write safe and reliable code.


## Debugger for diagnostics

A debugger is a software tool that allows developers to pause a program's execution at specific points in its code to examine its internal state and debug any issues.

Debuggers can be run either from the command line or through an integrated development environment (IDE).

Debuggers allow developers to step through code line by line, set breakpoints, and inspect variables and memory at runtime.

Rust's built-in debugger is called GDB (GNU Debugger), and it's available on most platforms.

Additionally, there are also third-party Rust debugging tools like Rust-GDB and Rust-LLLD, which offer specialized features like improved integration with Rust development environments, better support for multi-threaded applications, and more:

* Rust-GDB: an implementation of GNU Debugger (GDB) that is customized specifically for debugging Rust programs.

* Rust-LLDB: a debugger that allows developers to analyze and debug Rust programs using the LLVM debugger (LLDB) tool.



## Debugger for diagnostics - example

To use a debugger for diagnostics, you can use Rust's built-in debugger, which called GDB (GNU Debugger).

Activate debug symbols. GDB requires debug symbols to provide human-readable information about the code, such as function and variable names, file names, and line numbers. To enable debug symbols in a release, you can add the following lines to their Cargo.toml file:

```toml
[profile.release]
debug = true
```

Build the program with debugging information. You can then build their Rust code with the debug option.

```sh
cargo build --bin my_program --release
```

Start GDB by typing `gdb my_program` in the terminal (where my_program is the name of the executable file).

Load the executable by using the command `file my_program` to load your program into GDB.

Set breakpoints. You can place breakpoints in your Rust code using the `break` command. For instance, to stop the program when reaching line 10 of the file `main.rs`, you can type `break main.rs:10`.

Run the program. You can execute the program by typing `run`. The execution will pause at the first breakpoint, if there is one.

Inspect variables. You can use the `print` command to check the value of a variable at a specific point in the code. For example, `print x`.

Use backtrace. You can use the `bt` command to print a backtrace of the stack, which shows the sequence of function calls that led to the current state of the program.

Continue execution of the program by using the command `continue`.


# Design patterns

This page is a section divider and intentionally blank.


## Design patterns: introduction

Design patterns in programming refer to reusable solutions to common problems that arise during software development. These patterns provide a standard set of practices, templates and a recommended course of action for solving recurring problems. They are proven solutions that help developers to build software that is more modular, maintainable, and scalable.

Design patterns are used by software developers to ensure that the code follows best practices while tackling common problems. They are grouped into three categories: creational, structural, and behavioral.

Creational patterns are used to create objects and instances of classes during runtime. Structural patterns are aimed at developing the overall structure of the code, while behavioral patterns are used to manage communication between object instances.

Using a well-defined design pattern allows developers to focus on the software's functionalities rather than the design aspects of the code. Some of the commonly used design patterns in programming include Singleton, Observer, Decorator, Facade, Adapter, Iterator, Builder, and many more.

As one example, the Iterator design pattern provides a way to iterate over a collection of objects. In Rust, this is built into the language with the Iterator trait.

Example:

```rust
let numbers = vec![1, 2, 3, 4, 5];
for number in numbers.iter() {
    println!("{}", number);
}
```


## Design patterns: adapter

The "adapter" structural design pattern enables incompatible interfaces to collaborate. This can be implemented using a adapter struct that wraps an adaptee struct.

```rust
// Suppose we have a typical struct that we want to adapt.
// This struct is typically know as the "adaptee".
struct CircleWithRadius {
    radius: f32;
}

// The adapter structural design pattern typically means
// we define an outer struct that wraps an inner struct.
// The outer struct is typically known as the "adapter".
// The inner struct is typically known as the "adaptee".
struct CircleWithDiameter {
    adaptee: CircleWithRadius;
}

// We implement the adapter methods, such as these accessors,
// so the methods actually get and set the adaptee's data.
// This is similar to a proxy object, or to a facade object.
impl CircleWithDiameter {
    fn diameter(&self) -> f32 {
        adaptee.radius * 2;
    }

    fn set_diameter(&self, diameter: f32) {
        adaptee.radius = diameter / 2;
    }
}
```


## Design patterns: builder

The "builder" design pattern creates complex objects via simpler steps. This can be implemented using a struct with setter methods.

```rust
struct Foo {
    a: i32,
    b: i32,
}

struct FooBuilder {
    a: Option<i32>,
    b: Option<i32>,
}

impl FooBuilder {
    fn new() -> Self {
        FooBuilder {
            a: None,
            b: None,
        }
    }

    fn a(mut self, a: i32) -> Self {
        self.a = Some(a); self
    }

    fn b(mut self, b: u32) -> Self {
        self.b = Some(b); self
    }

    fn build(self) -> Foo {
        Foo {
            a: self.a.expect("missing field a"),
            b: self.b.expect("missing field b"),
        }
    }
}

let foo = FooBuilder::new().a(1).b(2).build();
```


## Design patterns: observer

The "observer" design pattern enables one object to notify others of its state changes. This can be implemented using Rust's channels or event emitters.

Example:

```rust
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        tx.send("Hello, World!").unwrap();
    });

    let message = rx.recv().unwrap();
    println!("{}", message);
}
```


## Design patterns: singleton

The "singleton" design pattern ensures that only one instance of a particular object is ever created. This can be implemented using a static variable or a lazy static variable.

Example:

```rust
struct Singleton;

impl Singleton {
    fn instance() -> &'static Self {
        static mut INSTANCE:
            *const Singleton = 0 as *const Singleton;
        static ONCE: Once = Once::new();
        unsafe {
            ONCE.call_once(|| {
                let singleton = Singleton {};
                INSTANCE = mem::transmute(Box::new(singleton));
            });

            &*INSTANCE
        }
    }
}
```


# Crates we like for many of our programs

This page is a section divider and intentionally blank.


## Assertables crate for assert macro tests

<https://crates.io/crates/assertables>

The Rust Assertables crate is a library of assert macros for testing, validation, and verification. If an assert macro succeeds, then it completes normally, otherwise it prints a diagnostic error message.

Edit your file `Cargo.toml`:

```toml
[dependencies]
assertables = "7"
```

Example of how to use the Assertables crate:

```rust
#[cfg(test)]
mod test_assert_x_result {
    use assertables;

    #[test]
    fn example1() {
        let x = 1;
        let y = 2;
        assert_lt!(x, y);
    }

    #[test]
    fn example2() {
        let string1 = "Hello World";
        let string2 = "He";
        assert_starts_with!(string1, string2);
    }
}
```

In the example, the macro `assert_lt!` tests that `x` is less than `y`, and the macro `assert_starts_with!` tests that `string1` starts with `string2`.

The Assertable crate provides a range of macros for compile-time testing, as well as debug macros for non-optimized runtime debugging, and runtime macros for optimized runtime validation and verification.


## cached crate for memoization

The Rust `cached` crate provides implementations of several caching structures as well as macros for defining memoized functions.

Example to cache a function's input and output:

```rust
use cached::proc_macro::cached;

/// Defines a function named `fib` that uses cached.
/// By default, the cache name will be the function's
/// name in all caps; this cache name is "FIBONACCI".
#[cached]
fn fiboncacci(n: u64) -> u64 {
    if n == 0 || n == 1 { return n }
    fiboncacci(n-1) + fiboncacci(n-2)
}
```

Example to cache a function call for a time period:

```rust
use cached::proc_macro::once;

/// Only cache the initial function call.
/// Function will be re-executed after the cache
/// expires (according to `time` seconds).
/// When no (or expired) cache, concurrent calls
/// will synchronize (`sync_writes`) so the function
/// is only executed once.
#[once(time=10, option = true, sync_writes = true)]
fn keyed(a: String) -> Option<usize> {
    if a == "a" {
        Some(a.len())
    } else {
        None
    }
}
```


## log crate for logging messages

<https://crates.io/crates/log>

The Rust log crate provides a logging framework for Rust programs. The log crate provides a simple interface for logging messages at different levels of severity, such as info, warn, error, and debug.

To use the log crate, you need to first define a logger implementation. This implementation defines how the log messages are recorded and where they are sent. There are many different logger implementations available in the Rust ecosystem, such as simple_logger, env_logger, log4rs, fern, and syslog.

Example file `Cargo.toml` with `log` and `simple_logger`:

```toml
[dependencies]
log = "*"
simple_logger = "*"
```

Example:

```rust
use log::{info, warn, error, debug};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().env().init().unwrap();
    info!("Example info message");
    warn!("Example warn message");
    error!("Example error message");
    debug!("Example debug message");
}
```

The log crate also allows you to configure the logging behavior at runtime by setting the log level and enabling or disabling specific loggers. This can be useful for debugging and troubleshooting purposes.

The log crate also provides macros and functions for working with log messages, such as for formatting and recording the runtime file name and line number.



## itertools crate for iterator extras

<https://crates.io/crates/itertools>

The Rust itertools crate is a third-party library that provides a powerful set of tools for working with iterators in Rust. It offers a wide range of functions and macros for manipulating and combining iterators, making it easier and more efficient to work with collections of data in Rust.

The itertools crate provides:

* iteration functions that can be used to manipulate and transform iterators

* combinator functions that can be used to generate new iterators from existing iterators

* macros that can be used to simplify the code required to work with iterators

Example:

```rust
use itertools::{Itertools, join};

fn main() {
    // Demo data
    let numbers = vec![1, 2, 3];
    let letters = vec!['a', 'b', 'c'];

    // Use the "join" macro to flatten lists into one string
    let joined = join(letters, ", ");
    println!("{:?}", joined);

    // Use the combinator functions to mix iterators
    for (n, l) in numbers.iter()
        .cartesian_product(letters.iter()) {
        println!("{}{}", n, l);
    }
}
```


## num crate for number types and traits

The Rust `num` crate is a collection of numeric types and traits for Rust. This includes new types for big integers, rationals (aka fractions), and complex numbers, new traits for generic programming on numeric properties like Integer, and generic range iterators.

Example of the `PrimInt` trait for primitive integers, which helps with generic traits and monomorphism:

```
pub trait FizzBuzz {
    fn fizzbuzz(&self) -> String;
}

impl<T> FizzBuzz for T
where 
    T: num::traits::int::PrimInt,
    T: std::fmt::Display,
{
    fn fizzbuzz(&self) -> String {
        let t0 = T::zero();
        let t3 = T::from(3).unwrap();
        let t5 = T::from(5).unwrap();
        match (*self % t3 == t0, *self % t5 == t0) {
            (true, true) => String::from("FizzBuzz"),
            (true, _) => String::from("Fizz"),
            (_, true) => String::from("Buzz"),
            _ => format!("{}", self),
        }
    }
}

fn main() {
    for i in 1..=100 {
        println!("{}", i.fizzbuzz())
    }
}
```


## once_cell crate for lazy global variables

<https://crates.io/crates/once_cell>

The Rust once_cell crate provides a way to create lazily evaluated, immutable, and thread-safe global variables in Rust. It is designed to provide a simple and efficient way to handle global state in Rust programs.

The main type provided by the once_cell crate is the OnceCell<T> type. This type is a container for a single value of type T that can be initialized lazily and only once. When the value is accessed for the first time, it is created using a closure that is passed to the OnceCell's get_or_init method. The closure is executed only once, and the resulting value is stored in the OnceCell for future accesses.

The OnceCell type is also thread-safe, which means that multiple threads can access the same OnceCell instance safely. If multiple threads attempt to access the OnceCell at the same time, only one of them will be allowed to execute the initialization closure, while the other threads will block until the value is fully initialized.

The OnceCell crate also provides other useful types, such as the unsync::OnceCell<T> type, which is similar to the regular OnceCell<T> but is not thread-safe, and the sync::Lazy<T> type, which is similar to the OnceCell<T> but provides an additional level of indirection that allows for even more efficient initialization and access.

Example of once_cell `Lazy` to initialize a `Regex` regular expression:

```rust
use regex::Regex;
use once_cell::sync::Lazy;

fn main() {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new("hello").unwrap());
    let matched = RE.is_match("hello world");
    println!("{}", matched);
}
```


## syn crate for syntax analysis

<https://crates.io/crates/syn>

The Rust `syn` crate is a library that enables parsing, analyzing, and processing of Rust source code syntax. It is designed to allow developers to build tools like code generators, linters, and syntax highlighters.

The `syn` crate provides an abstract syntax tree (AST) of Rust code, which represents the structure and meaning of the code without including all the details and syntax of the source code. This makes it easier for developers to work with Rust code programmatically, as they do not need to parse and analyze the code manually.

The `syn` crate also provides support for parsing Rust code in different contexts, such as macros, attributes, and expressions, making it a versatile tool for Rust developers.

Example of a derive macro:

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // ...
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
```


## regex crate for regular expressions

<https://crates.io/crates/regex>

The Rust regex crate is a regular expression library for the Rust programming language. It provides a fast and efficient way to search, match, and manipulate text using regular expressions.

The main types provided by the regex crate are the `Regex` and `Captures` types. The `Regex` type represents a compiled regular expression pattern that can be used to search for matches in a text string. The `Captures` type represents the groups captured by a successful match and allows for easy extraction of matched substrings.

The regex crate supports a wide range of regular expression syntax, including Perl-style regular expressions and POSIX extended regular expressions. It also supports Unicode character properties and provides a range of Unicode-aware matchers and modifiers.

The regex crate is highly performant and is designed to handle large inputs efficiently. It provides a range of options for controlling the matching behavior, such as case-insensitive matching, multi-line matching, and greedy or lazy quantifiers.

Example:

```rust
use regex::Regex;

fn main() {
    // Find the first occurrence of a digit char and word char
    let r = Regex::new(r"(\d)(\w)").unwrap();
    let captures = r.captures("a1b2c3").unwrap();
    println!("{:?}", captures)
}
```

Output:

```txt
Captures({0: Some("1b"), 1: Some("1"), 2: Some("b")})
```


## Serde crate for serialize/deserialize

<https://crates.io/crates/serde>

The Rust Serde crate is a widely used library for serialization and deserialization of Rust data structures to and from various data formats, such as JSON, TOML, YAML, and many others.

The Serde `derive` feature can automatically derive the serialization and deserialization code for Rust data structures, such as:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}
```

This defines a `Person` struct with two fields: `name` is a `String`, and `age` is a `u32`. The `#[derive(Serialize, Deserialize)]` attribute tells Serde to automatically generate the serialization and deserialization code for this struct.

You can then use Serde to serialize an instance of this struct to JSON:

```rust
let person = Person { name: "Alice".to_string(), age: 30 };
let json = serde_json::to_string(&person).unwrap();
```

This creates a `Person` instance and serializes it to JSON using the `serde_json::to_string` function. The `&person` argument is a reference to the `Person` instance that you want to serialize.

You can also deserialize a JSON string into a Person instance:

```rust
let json = r#"{"name":"Bob","age":25}"#;
let person: Person = serde_json::from_str(json).unwrap();
```

This deserializes the json string into a `Person` instance using the `serde_json::from_str` function.


## Strum crate for enums

The Rust Strum crate provides macros for working with enums. The crate provides several useful macros that can be used to derive implementations for various traits on enums.

* EnumString - Derives the ability to parse strings into enum variants using the FromStr trait.

* EnumVariantNames - Derives a method that returns a list of the enum's variant names as strings.

* Display - Derives the ability to convert enum variants to strings using the Display trait.

* AsRefStr - Derives the ability to convert enum variants to string slices using the AsRef trait.

Example:

```rust
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let color = "Red".parse::<Color>().unwrap();
    println!("{:?}", color);
}
```

This example uses the `EnumString` macro to derive the `FromStr` trait for the `Color` enum. This enables us to parse the string "Red". The `unwrap()` method handles any parse errors.


## rand crate for random numbers

The Rust `rand` crate is a library that provides various random number generators (RNGs), algorithms, and utilities. It can generate random numbers of different types (such as integers or floating-point numbers), as well as random bytes and strings.

The crate offers several algorithms for generating random numbers:

* Xorshift: a fast, non-cryptographic algorithm that generates random numbers with a period of 2^128 - 1.

* ChaCha: a stream cipher that can be used to generate random numbers with a very long period, suitable for cryptographic applications.

* Hc128: another stream cipher that can be used for random number generation.

In addition to generating random numbers, the "rand" crate also provides utilities for shuffling arrays, generating random values from enums, and more.

### rand crate example

```rust
use rand::Rng;

fn main() {
    // Create a random number generator
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..=100);
    println!("{}", number);
}
```

In this example, we use the `Rng` trait from the rand crate to generate a random number between 1 and 100. The `thread_rng()` function returns a new instance of the generator. The `gen_range()` function generates a random number in the specified range (inclusive of the lower bound and exclusive of the upper bound). We print the result to the console.

# Crates we like for command line interfaces

This page is a section divider and intentionally blank.


## CLAP crate for commands

<https://crates.io/crates/clap>

The Rust CLAP crate is for command line argument parsing. CLAP provides a flexible and intuitive way to define command-line interfaces (CLIs) for Rust programs, with support for a wide range of features and options.

Add a dependency in your file `Cargo.toml` file, along with features you want:

```rust
[dependencies]
clap = { version = "4", features = ["derive"] }
```

Defining your CLI options by using CLAP derive `Parser`:

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "None")]
struct Args {
   #[arg(short, long)]
   name: String,
   #[arg(short, long)]
   age: i32,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
```

Example run:

```sh
cargo run -- --name Alice --age 22
```

Example output:

```sh
Args { name: "Alice", age: 22 }
```


## CLAP command builder - example

See code directory `/projects/crates/clap`

The CLAP command builder pattern is one way set up CLAP:

```rust
use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("My Program")
    .version("1.0.0")
    .author("Alice Adams")
    .about("This is my program")
    .arg(
        Arg::new("name")
        .help("Set the name to use")
        .short('n')
        .long("name")
        .action(ArgAction::Set)
    )
    .after_help("Longer explanation")
    .get_matches();

    // Process the command line arguments
    if let Some(x) = matches.get_one::<String>("name") {
        println!("Name is {}", x);
    }
}
```

The command builder defines introduction program information, then the `name` argument, then any conclusion program information. The example uses the `get_matches()` method to parse command-line arguments into a `matches` struct, then prints the `name` argument.

Example run:

```sh
cargo run -- --name Alice
```

Example output:

```sh
Name is Alice
```


## Textwrap crate for text wrapping

<https://crates.io/crates/textwrap>

The Rust Textwrap crate is a library for wrapping and formatting text in Rust. It provides a simple API for wrapping text to a specified width, as well as support for indentation, alignment, and hyphenation.

The Textwrap crate can be used for a variety of text formatting tasks, such as formatting text for display in a terminal, wrapping text for printing to a file, or formatting text for display in a GUI application.

Some of the key features of the Textwrap crate include:

* Support for wrapping text to a specified width, with options for indenting and aligning the wrapped text.

* Support for hyphenation, which can improve the readability of text by breaking long words across lines.

* Support for custom line breaking rules, which can be used to handle special cases such as URLs or email addresses.

* A simple and easy-to-use API, with sensible defaults that make it easy to get started with text wrapping in Rust.

* Support for a variety of text input and output formats, including plain text, HTML, and Markdown.

Overall, the Rust Textwrap crate is a powerful tool for formatting and wrapping text in Rust. Its flexible API and support for advanced features like hyphenation and custom line breaking rules make it a great choice for developers looking to format text for a variety of applications.


## Textwrap crate - example

[Runnable project](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/textwrap/fill_wrap)

Example of textwrap fill and wrap:

```rust
use textwrap::{fill, wrap};

fn main() {
    let s = "Rust is a great programming language for us";
    println!("{}", fill(s, 22));
    println!("{:?}", wrap(s, 22));
}
```

Example output:

```text
Rust is a great
programming language
for us
["Rust is a great", "programming language", "for us"]
```

In this example, we import the `fill` and `wrap` functions from the textwrap crate. 

We use `fill` to fill one string with lines that are each 22 characters maximum per line. 

We use `wrap` to create a vector of strings that are each 22 characters maximum per line.


## Cursive crate for text user interfaces

<https://crates.io/crates/cursive>

The Rust Cursive crate is a TUI (text user interface) library for building interactive terminal applications. It allows developers to create rich terminal UIs with features such as customizable layouts, event handling, input handling, and styling.

Cursive is built on top of the Rust `ncurses` library, which provides low-level terminal I/O and screen rendering. Cursive provides a higher-level API than ncurses, making it easier to build complex UIs without worrying about the details of terminal control.

Some of the key features of cursive include:

* A flexible layout system that allows developers to create complex, dynamic UIs with ease.

* Support for a wide range of input events, including mouse input and keyboard shortcuts.

* A rich set of widgets, including buttons, checkboxes, text fields, and more.

* Customizable theming and styling, with support for colors, fonts, and text effects.

* Support for Unicode and UTF-8 input and display.

* A robust documentation and community resources.

Overall, the Rust Cursive crate is a powerful tool for building interactive terminal applications in Rust. Its high-level API, flexibility, and rich feature set make it an excellent choice for developers looking to build text-based UIs with ease.


## Cursive crate - example

[Runnable project](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/cursive/hello_world)

Run a simple interactive terminal user interface program, by using the `cursive` crate.

```rust
use cursive::{Cursive, CursiveExt};
use cursive::views::{Dialog, TextView};

fn main() {
    let mut siv = Cursive::default();

    siv.add_layer(
        Dialog::around(TextView::new("Hello, World!"))
            .title("Cursive Example")
            .button("Quit", |s| s.quit()),
    );

    siv.run();
    println!("Ok")
}
```

This code creates a `Cursive` object, adds a `TextView` containing the message "Hello, World!" to a `Dialog`, and then displays the dialog with a "Quit" button that will close the application when clicked.


## console, dialoguer, indicatif for CLIs

<https://crates.io/crates/console>

<https://crates.io/crates/dialoguer>

<https://crates.io/crates/indicatif>

[project](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/console/hello_world)
  
The `console` crate provides access to terminal features so you can build nicer looking command line interfaces. 

The `dialoguer` crate helps you build small user inputs for the command line, such as prompts, inputs, selections, history, and more.

The `indicatif` crate helps you build command line interfaces that report progress to users. It helps format anything that indicates progress.

Example:

```rust
use console::Term;
use dialoguer::Input;
use indicatif::ProgressBar;

fn main() -> std::io::Result<()> {
    // console example
    let term = Term::stdout();
    term.write_line("Hello, World!")?;

    // dialoguer example
    let input = Input::<String>::new().interact_text()?;
    term.write_line(&input)?;

    // indicatif example
    let bar = ProgressBar::new(10);
    for _ in 0..10 { bar.inc(1); }
    bar.finish();

    Ok(())
}
```


## TUI crate for text user interfaces

The Rust TUI (Text User Interface) crate provides user interface widgets and utilities for building command-line tool interfaces. The TUI crate includes components such as text input fields, progress bars, tables, and menus, which can be used to create interactive and dynamic command-line interfaces.

The Rust TUI crate is built on top of the Rust ncurses library and provides an abstraction layer that simplifies the creation of user interfaces. The library is cross-platform and can be run on a variety of operating systems.

Example:

```rust
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;

fn main() {
    // Create a Terminal with the TermionBackend
    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Draw the UI
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    }).unwrap();
}
```

This code creates a new Terminal with TermionBackend and draws a simple block on it.

## tui crate - examples

[Runnable project](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/tui/terminal_draw_block)

Example to draw a block on the screen:

```
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::widgets::{Borders, Block};
use termion::raw::IntoRawMode;

fn main() {
    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Draw the UI
    terminal.clear();
    terminal.draw(|f| {
        let size = Rect { x: 8, y: 8, width: 8, height: 8 };
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    }).unwrap();
}
```


## walkdir crate for traversing directories

The Rust `walkdir` crate provides ways to iterate over directories and their contents. It is useful for traversing directories, such as for file managers, build systems, or search engines. It is built on top of the `std::fs` module.

Key features of the `walkdir` crate include: recursive directory iteration with configurable maximum depth; filtering options based on file attributes or name patterns; error handling and recovery mechanisms for I/O errors or permission issues; configurable follow-symlinks behavior; support for custom sorting and ordering of entries; optional support for cross-platform path handling and case sensitivity.

Example of how to use the walkdir crate:

```rust
use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("/path/to/directory")
    .into_iter()
    .filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            println!("Directory: {}", entry.path().display());
        } else {
            println!("File: {}", entry.path().display());
        }
    }
}
```

The `WalkDir::new` function creates a new directory walker, and `into_iter` returns an iterator that can be filtered and mapped over. The `ok` method filters out any errors that may occur during iteration. Then the `file_type` method on the `entry` variable checks if the entry is a directory or a file. Finally, we print out the name of the entry using the `display` method.


# Crates we like for development

This page is a section divider and intentionally blank.


## cargo-cache crate for caching builds

<https://crates.io/crates/cargo-cache>

The Rust cargo-cache crate provides a command-line interface (CLI) for managing the cache directory used by the Cargo package manager.

When you use Cargo to build a Rust project, it downloads and caches dependencies, build artifacts, and other files related to the build process in a directory called "cargo-cache". Over time, this directory can become quite large, taking up valuable disk space on your system.

The `cargo-cache` crate provides several commands that allow you to manage the cache directory. Some of the key features:

* Listing the contents of the cache directory

* Clearing the cache directory

* Showing the size of the cache directory

* Displaying information about individual cached packages

Example listing:

```
Cargo cache '~/.cargo':
Total:                               4.41 GB
  75 installed binaries:           481.75 MB
  Registry:                          3.92 GB
    Registry index:                503.26 MB
    2563 crate archives:           403.60 MB
    2563 crate source checkouts:     3.02 GB
  Git db:                            2.67 MB
    1 bare git repos:              905.51 KB
    1 git repo checkouts:            1.77 MB
```

Using `cargo-cache`, you can easily clear out old or unnecessary cached files, reclaiming valuable disk space on your system. You can also use the `cargo-cache` CLI to better understand the contents of the cache directory and diagnose any issues related to the build process.


## cargo-crev for community-driven trust

<https://crates.io/crates/cargo-crev>

The Rust cargo-crev crate helps developers build a community-driven trust system for their packages. The crate provides a way for developers to create signed reviews of their dependencies and share them with other developers. These reviews can include information about the quality of the code, how well the documentation is written, and any security concerns.

The idea behind cargo-crev is to create a trusted network of developers who can vouch for the quality and safety of each other's code. This can help prevent malicious packages from being added to the Rust ecosystem and can provide a sense of security for developers who rely on Rust packages in their projects.

Using cargo-crev, developers can create public or private reviews of their dependencies, and other developers can use these reviews to make informed decisions about which packages to use in their projects. The crate also provides a command-line interface that makes it easy to manage reviews and share them with the community.

Key features:

* Build a web of trust of users to help verify the code you use

* Warn you about untrustworthy crates and security vulnerabilities

* Increase trustworthiness of your own code


## cargo-dist crate for distribution archives

<https://crates.io/crates/cargo-dist>

The Rust cargo-dist crate is a Rust crate that provides a simple and convenient way to package a Rust project as a distributable archive. The crate is designed to work with the Rust cargo build system, and provides a number of features that make it easy to create archives for various platforms.

One of the main features of `cargo-dist` is its support for cross-compiling. The crate can automatically build and package your Rust project for a number of different platforms, including Windows, macOS, Linux, and Android, all from a single command. This can save a lot of time and effort when distributing your project to users on multiple platforms.

Another useful feature of `cargo-dist` is its support for packaging dependencies. When you create a distributable archive with `cargo-dist`, it will automatically include all of the dependencies for your Rust project, so users don't have to manually install them. This can help simplify the installation process for your project and reduce the risk of dependency conflicts.

Finally, `cargo-dist` provides a number of options for customizing the packaging process. You can specify the format of the archive (e.g. `.tar.gz`, `.zip`, etc.), include or exclude specific files or directories, and more. This can help ensure that the distributable archive contains exactly what you want, and nothing more.


## cargo-release crate for publishing

<https://crates.io/crates/cargo-release>

The Rust cargo-release crate provides a set of tools for releasing Rust crates to repositories such as crates.io. It automates many of the steps involved in releasing a new version of a crate, making it easier and more efficient to manage the release process.

To use the `cargo-release` crate in your Rust project, you'll need to add it as a dependency in your `Cargo.toml` file. Once you've done that, you can configure the crate by creating a `.cargo` directory in your project root, then adding a `config.toml` file with the following contents:

```toml
[package]
version = "0.1.0"

[dependencies]
cargo-release = { version = "0.15", features = ["procmacro"] }

[release]
## ... configure release options here ...
```

Overall, the `cargo-release` crate provides a powerful and flexible set of tools for managing the release process for Rust crates. It can help to streamline the release process, reduce the risk of errors and inconsistencies, and ensure that your crates are published to repositories like crates.io in a consistent and reliable manner.


Features: Ensures you are in a good state for release, such as with your git branch, remote, and tree. Supports workspaces using cargo's native flags, like --workspace, --exclude and --package.  Handles cargo publish, tagging, and pushing.


## cargo-release crate - examples

The cargo-release crate provides many features and functions, including these examples.

Release Management: The `cargo-release` crate provides a range of tools for managing the release process, including the ability to automatically generate a new version number based on a specified release type (e.g. major, minor, or patch), update the changelog and version number in your crate's Cargo.toml file, tag the release in Git, and publish the crate to crates.io:

```bash
cargo release --dry-run  # preview the release process
cargo release            # perform the release
```

Pre-Release Management: The `cargo-release` crate also provides tools for managing pre-releases, including the ability to create and publish pre-release versions of your crate (e.g. 0.2.0-alpha.1), and to promote pre-release versions to stable releases:

```bash
cargo release --pre-release  # create a pre-release version
cargo release --continue     # promote a pre-release to stable
```

Customization: The `cargo-release` crate is highly configurable, allowing you to customize the release process to suit your needs. For example, you can specify which branches to release from, configure the changelog format and location, and specify additional steps to perform during the release process:

```toml
[release]
branches = ["main"]
changelog = "docs/CHANGELOG.md"
pre-release = false

[release.steps.post]
## ... additional steps to perform after the release ...
```


## cargo-make crate for task runners

<https://crates.io/crates/cargo-make>

The Rust cargo-make crate is a tool that extends the functionality of the Cargo package manager by providing a way to define complex build processes in a simple, declarative way.

Here are some of the key features of the cargo-make crate:

* Declarative build scripts: With cargo-make, you define your build process in a Toml configuration file, which makes it easy to understand and modify the build process.

* Cross-platform support: cargo-make runs on Linux, macOS, and Windows, making it easy to maintain consistent build processes across different platforms.

* Task management: You can define a set of tasks, each of which can be executed individually or as part of a larger build process.

* Dependency management: cargo-make ensures that tasks are executed in the correct order based on their dependencies, which helps avoid build errors and improve build performance.

* Pre and Post Hooks: cargo-make supports pre- and post-hooks, to perform actions before and after the build process, such as cleaning artifacts, setting environment variables, etc.

* Plugins: cargo-make supports plugins to extend functionality, such as adding new tasks or modifying the build process.

Install:

```sh
cargo install cargo-make
```

After installation, you can define your build process in a Toml configuration file named `Makefile.toml`.


## cargo-make crate - example

Here's an example `cargo-make` configuration file `Makefile.toml`:

```toml
[tasks.build]
command = "cargo build --release"

[tasks.test]
command = "cargo test"

[tasks.lint]
command = "cargo clippy"

[tasks.default]
dependencies = ["build", "test", "lint"]
```

In this example, we've defined three tasks: `build`, `test`, and `lint`. Each task has a command that specifies what action to perform when the task is executed. The `default` task depends on the `build`, `test`, and `lint` tasks, and is executed when no task is specified.

You can then run your build process using the following command:

```sh
cargo make
```

This will execute the default task and all its dependencies in the correct order.

If you want to execute a specific task, you can use the following command:

```sh
cargo make <task-name>
```


## Criterion crate for benchmarks

<https://crates.io/crates/criterion>

The Rust Criterion crate, titled Criterion.rs, is a popular benchmarking library. It is used to measure and analyze the performance of Rust programs by running multiple iterations of a benchmark and collecting statistical data.

Criterion.rs provides a simple and intuitive API for writing benchmarks, allowing developers to create and run benchmarks quickly and easily. It supports a range of benchmarking options, including measuring CPU time, wall-clock time, memory usage, and more.

The crate uses statistical techniques to calculate benchmark results, which provides more accurate and reliable results than simple timing measurements. It also supports reporting and visualization of benchmark results, making it easier for developers to analyze and compare their code's performance.

Features:

* Statistics: Statistical analysis detects if, and by how much, performance has changed since the last benchmark run.

* Charts: Uses gnuplot to generate detailed graphs of benchmark results; see the gnuplot website for installation instructions.

* Stable-compatible: Benchmark your code without installing nightly Rust.

Overall, Criterion.rs is an essential tool for Rust programmers who want to optimize the performance of their programs and ensure they are running efficiently.



## Criterion crate - example

Add to your file `Cargo.toml`:

```toml
[dev-dependencies]
criterion = { version = "0.4", features = ["html_reports"] }

[bench]
name = "my_benchmark"
harness = false
```

Create a file `$PROJECT/benches/demo.rs` with this code:

```rust
use criterion::{
    black_box, criterion_group, criterion_main, Criterion
};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "fib 20",
        |b| b.iter(|| fibonacci(black_box(20)))
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

Run this benchmark with `cargo bench`. You should see output like below.

```txt
fib 20   time:   [26.029 us 26.251 us 26.505 us]
Found 11 outliers among 99 measurements (11.11%)
  6 (6.06%) high mild
  5 (5.05%) high severe
```


# Crates we like for concurrency and parallelism

This page is a section divider and intentionally blank.


## Crossbeam crate for concurrency

<https://crates.io/crates/crossbeam>

The Rust crossbeam crate provides low-level primitives for concurrent programming, such as locks, channels, and memory fences. These primitives are useful when fine-grained synchronization is required, or when working with non-standard concurrency patterns.

The crossbeam crate makes concurrent programming easier by providing:

* Atomic types: such as `AtomicBool`, `AtomicI32`, and `AtomicUsize`, which can be used to perform atomic operations on shared variables without the need for locks. This allows for efficient and safe concurrent access to shared data.

* Locks: such as `Mutex`, `RwLock`, and `Semaphore`, which can be used to protect shared resources from concurrent access. These locks are highly efficient and can be used in both single-threaded and multi-threaded contexts.

* Channels: such as `unbounded()`, `bounded()`, and `select()`, which can be used to communicate between threads. These channels are highly efficient and can be used to implement many common concurrency patterns, such as producer-consumer and pipeline processing.

* Memory fences: such as `atomic::fence()`, which can be used to enforce ordering constraints on memory accesses. This is useful when working with non-standard concurrency patterns or when fine-grained synchronization is required.

Overall, the crossbeam crate provides a powerful set of low-level primitives for concurrent programming in Rust, allowing developers to build complex and efficient concurrent applications with ease.

## epoll crate for event polling

Epoll is a Linux kernel interface for efficient I/O event notification by allowing user-space applications to monitor multiple file descriptors or sockets for events. 

Epoll works by registering a set of file descriptors with the kernel, and then waiting for events to occur on those descriptors. It uses a "polling" approach, where the application waits for the kernel to signal that events are ready, rather than actively polling the file descriptors itself.

The epoll API provides three system calls:

* `epoll_create`: creates a new epoll instance and returns a file descriptor that can be used to refer to it.

* `epoll_ctl`: modifies the set of file descriptors that are being monitored by the epoll instance. It can be used to add or remove file descriptors from the set, or to change the events that the kernel should watch for (e.g., read, write, or error).

* `epoll_wait`: waits for events to occur on the file descriptors that are being monitored by the epoll instance. It blocks until at least one event occurs, and then returns information about the file descriptor(s) that triggered the event.

One of the main advantages of epoll over other I/O notification mechanisms (such as `select` and `poll`) is its ability to scale well in high-concurrency scenarios, where there are many file descriptors being monitored at once. Epoll achieves this scalability by using a "red-black" tree data structure to efficiently keep track of the set of file descriptors, rather than linearly searching through them like `select` and `poll` do.

Rust is able to interact with the operating system interfaces for input-output (I/O) operations, specifically with the epoll interface and epoll crate.


## epoll crate - example

[Runnable project](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/epoll/tcp_listener)

Example to create a TCP listener and register it with epoll:

```rust
use std::{net::TcpListener, os::unix::io::AsRawFd};
use epoll::Epoll;
use epoll::{ControlOptions, Event, Events};

fn main() -> std::io::Result<()> {
    // Create an epoll instance and a listener fd
    let epoll = Epoll::new()?;
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let listener_fd = listener.as_raw_fd();

    // Register the listener with epoll
    let mut event = Event::new(
        ControlOptions::EPOLLIN,
        listener_fd as u64
    );
    epoll.ctl_add(&mut event)?;

    let mut events = Events::with_capacity(128);
    loop {
        // Wait for events to occur
        let num_events = epoll.wait(&mut events, -1)?;
        for i in 0..num_events {
            let event = events.get(i).unwrap();
            if event.data() == listener_fd as u64 {
                // Accept the connection
                let (stream, _) = listener.accept()?;
                println!("Accepted new connection");
            }
        }
    }
}
```

The code creates an epoll instance, and a TCP listener, and registers it with epoll. Then the code uses an event loop, which waits for events to occur and processes them. The code accepts incoming connections and prints a message.


## Flume crate for channels

<https://crates.io/crates/flume>

The Rust Flume crate provides multi-producer, multi-consumer channels, including unbounded, bounded, and rendezvous queues.

Flume is fast, flexible, and a drop-in replacement for std::sync::mpsc, with additional features like MPMC support, send timeouts/deadlines, and an ergonomic select-like interface.

Example to spawn and sum:

```rust
use std::thread;

fn main() {
    println!("Spawn");
    let (tx, rx) = flume::unbounded();
    thread::spawn(move || {
        (0..10).for_each(|i| {
            tx.send(i).unwrap();
        })
    });
    let received: u32 = rx.iter().sum();
    assert_eq!((0..10).sum::<u32>(), received);
}
```


## parking_lot crate for synchronization

<https://crates.io/crates/parking_lot>

The Rust `parking_lot` crate is a that provides synchronization primitives for Rust programs. Specifically, the crate provides a set of concurrent data structures that are designed to be faster and more efficient than the ones provided by Rust's standard library.

The `parking_lot` crate includes several types of synchronization primitives, such as locks, mutexes, and semaphores. These primitives can be used to coordinate access to shared resources in a multithreaded program, ensuring that multiple threads can safely access the same data without causing data races or other synchronization issues.

One of the key advantages of the `parking_lot` crate is its performance. The crate is designed to be highly optimized for multithreaded access, using techniques like spinlocking and memory barriers to minimize the overhead of synchronization operations. As a result, programs that use the `parking_lot` crate can often achieve significantly better performance than those that use the synchronization primitives provided by Rust's standard library.

In addition to its performance benefits, the `parking_lot` crate is also designed to be easy to use. The crate provides a simple and consistent API for working with its various synchronization primitives, and includes extensive documentation and examples to help developers get started.

Overall, the `parking_lot` crate is a valuable tool for Rust developers who need to coordinate access to shared resources in a multithreaded program. Its high performance and ease of use make it a popular choice for a wide range of applications, from low-level systems programming to high-performance web servers and beyond.


## Rayon crate for parallelism

<https://crates.io/crates/rayon>

The Rust rayon crate provides a high-level API for data parallelism. It allows developers to write code that can automatically be parallelized across multiple threads, without needing to manage low-level details of thread creation and synchronization.

The rayon crate provides several features that make parallelism easier:

* Parallel iterators: The rayon crate provides parallel versions of many of the standard iterators in Rust, such as `map()`, `filter()`, and `fold()`. These parallel iterators allow developers to write code that can automatically be parallelized, without needing to write low-level threading code.

* Parallel collections: The rayon crate provides parallel versions of several standard Rust collections, such as `Vec` and `HashMap`. These collections allow developers to work with large data sets and automatically parallelize their code, without needing to manually split the data into chunks and manage thread synchronization.

* Work stealing: The rayon crate uses a work stealing algorithm to dynamically load balance the work across all available threads. This means that if one thread finishes its work early, it can automatically start working on tasks that are still pending on other threads, improving overall performance.

* Crossbeam integration: The rayon crate integrates seamlessly with the `crossbeam` crate, which provides low-level primitives for concurrent programming, such as locks and channels. This allows developers to combine high-level parallelism with low-level concurrency, as needed.

Overall, the `rayon` crate provides easy data parallelism in Rust, allowing developers to take advantage of modern hardware and achieve high performance in their applications without sacrificing safety and correctness.


# Crates we like for data

This page is a section divider and intentionally blank.


## arrow-csv crate for loading CSV to Arrow

<https://crates.io/crates/arrow-csv>

The Rust arrow-csv crate is a library that provides support for reading and writing CSV (Comma-Separated Values) files in the Arrow data format in Rust. The Arrow format is a columnar data format that is designed to be efficient and interoperable across different programming languages and systems.

The main types provided by the arrow-csv crate are the `CsvReader` and `CsvWriter` types. The `CsvReader` type represents a CSV reader that can be used to read CSV data from a file or a stream and convert it to an Arrow record batch. The `CsvWriter` type represents a CSV writer that can be used to write Arrow record batches to a CSV file or a stream. Both types support a wide range of options for controlling the CSV parsing and formatting behavior, such as delimiter, quoting, escaping, and encoding.

The arrow-csv crate also provides support for schema inference, which means that it can automatically infer the data types and column names from the CSV data, making it easier to work with CSV files that do not have a predefined schema.

The arrow-csv crate is highly performant and is designed to handle large CSV files efficiently. It provides a range of optimizations, such as parallel processing and memory-mapped files, to minimize memory usage and improve performance.

Overall, the Rust arrow-csv crate is a powerful and efficient library that provides a way to work with CSV data in the Arrow data format in Rust. It is widely used in a variety of applications, including data analysis, data processing, and data exchange.


## CSV crate for comma-separated values

<https://crates.io/crates/csv>

The Rust CSV crate is a library for reading and writing Comma-Separated Values files. The crate provides a fast and efficient way to work with CSV data, and supports a wide range of formats and options.

The main types provided by the CSV crate are the `Reader` and `Writer` types. The `Reader` type represents a CSV reader that can be used to read CSV data from a file or a stream. The `Writer` type represents a CSV writer that can be used to write CSV data to a file or a stream. Both types support a wide range of options for controlling the CSV parsing and formatting behavior, such as delimiter, quoting, escaping, and encoding.

The CSV crate also provides a range of other useful types and functions, such as the `ByteRecord` type for representing CSV records as byte arrays, the `StringRecord` type for representing CSV records as UTF-8 strings, and the Serde integration for easy serialization and deserialization of CSV data.

The CSV crate is highly performant and is designed to handle large files efficiently. It provides optimizations, such as lazy parsing and zero-copy parsing, to minimize memory usage and improve performance.

Example:

```rust
let mut file = File::open("spreadsheet.csv").unwrap();
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();
let mut reader = ReaderBuilder::new()
    .has_headers(true)
    .delimiter(b',')
    .from_reader(contents.as_bytes());
for result in reader.records() {
    let record = result.unwrap();
    println!("{:?}", record);
}
```


## Diesel crate for ORM

<https://crates.io/crates/diesel>

The Rust Diesel crate is a high-level, type-safe ORM (Object-Relational Mapping) library for Rust that provides a convenient and safe way to interact with relational databases. It provides a set of tools and abstractions for working with SQL databases, allowing developers to write safe and efficient code when working with databases.

Some of the key features of Rust Diesel include:

* Type-safe queries: Rust Diesel allows developers to write SQL queries using Rust code, making it easy to construct complex queries while ensuring that they are type-safe.

* Easy to use: Rust Diesel provides a simple and intuitive API for working with databases, making it easy to get started with database programming in Rust.

* High performance: Rust Diesel uses Rust's zero-cost abstractions and compile-time code generation to provide high performance when interacting with databases.

* Support for multiple databases: Rust Diesel supports a wide range of databases, including PostgreSQL, MySQL, and SQLite.

* Schema migrations: Rust Diesel provides a simple and powerful schema migration system, making it easy to manage changes to the database schema over time.


## Diesel crate - example

[Runnable project](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/diesel/hello_world_with_sqlite)

The Diesel crate is sophisticated because its typical use involves SQL migrations, database connections, automatic conversions from records to structs, and much more. The Diesel tutorial is excellent and well worth reading. This page has elided excerpts, to give you a taste.

Diesel uses schema macros, typically in a file `schema.rs`:

```rust
diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
    }
}
```

Diesel uses model structs, typically in a file `models.rs`:

```rust
#[derive(Debug, Queryable, Identifiable, AsChangeset)]
pub struct Post {
    pub id: i32,
    pub title: String,
}
```

Diesel uses database connections such as:

```rust
let mut connection = 
    SqliteConnection.establish("db.sqlite")
    .expect("Error connecting to database");
```

Diesel uses Domain-Specific Language (DSL) such as:

```rust
use self::schema::posts::dsl::*;
let results = posts
    .load::<Post>(connection)
    .expect("Error loading posts");
for post in results {
    println!("{}", post.title);
}
```


## Polars crate for data analysis

<https://crates.io/crates/polars>

The Rust Polars crate is a data manipulation and analysis library for the Rust programming language. It is designed to provide a fast, efficient, and easy-to-use interface for working with large datasets.

At its core, Rust Polars is built on top of the Apache Arrow memory format, which provides a standard way of representing data in memory. This allows Rust Polars to take advantage of the performance benefits of Arrow, such as zero-copy data access and efficient memory utilization.

Rust Polars provides a DataFrame API, which is similar to the DataFrame API in popular data analysis tools like Pandas for Python and the tidyverse in R. This API allows users to perform a wide variety of operations on their data, such as filtering, aggregation, grouping, and merging.

Some of the key features of Rust Polars include:

* Fast performance: Rust Polars is designed to be as fast as possible, with many operations being implemented using parallel processing to take advantage of multi-core CPUs.

* Easy-to-use API: The DataFrame API is designed to be easy to learn and use, with many common operations being implemented using a fluent, chainable syntax.

* Flexible data types: Rust Polars supports a wide variety of data types, including strings, numbers, dates, times, and more.

* Integration with other Rust libraries: Rust Polars can be easily integrated with other Rust libraries, such as Serde for serialization and deserialization.

Overall, Rust Polars is a powerful data analysis library that provides a fast, efficient, and easy-to-use interface for working with large datasets in Rust.


## Rusqlite crate for SQLite databases

<https://crates.io/crates/rusqlite>

The Rust Rusqlite crate is a library for working with SQLite databases. It provides many methods for querying and modifying data in SQLite databases, including prepared statements, transactions, and more.

Example of how to use Rusqlite to create create a table, insert data into the table, and select data from the table:

```rust
fn main() -> Result<()> {
    let conn = Connection::open(":memory:")?;
    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            age   INTEGER NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "INSERT INTO person (id, name, age) VALUES (?1, ?2, ?3)",
        ["1", "Alice", "30"],
    )?;
    let name: String = conn.query_row(
        "SELECT name FROM person WHERE id=1",
        [],
        |row| row.get(0),
    )?;
    println!("{}", name);
    Ok(())
}
```


## sqlx crate for SQL databases

<https://crates.io/crates/sqlx>


The Rust sqlx crate provides a type-safe, asynchronous, and composable SQL interface for working with databases. It is designed to make working with databases in Rust easier and more productive, while still being fast and efficient.

sqlx supports a wide range of databases, including PostgreSQL, MySQL, SQLite, and Microsoft SQL Server. It uses Rust's type system to provide a safe and ergonomic way to write SQL queries, while still allowing for raw SQL queries if needed.

One of the key features is support for async/await syntax, which allows for non-blocking database queries and operations. This makes it easy to write efficient, high-performance database code in Rust that can handle large numbers of concurrent requests.

In addition to its core functionality, sqlx provides a number of other useful features, such as support for migrations, prepared statements, transactions, automatic type conversions, and more. It also has excellent documentation and a friendly and helpful community, which makes it easy to get started and solve problems as they arise.


### Dependencies

When you add sqlx to `Cargo.toml`, you must choose feature providers for your specific database, and for Transport Layer Security (TLS). Also, sqlx versions have different options, so be sure to check the documentation.

```toml
[dependencies]
sqlx = {
    version = "0.6",
    features = [ "sqlite", "runtime-tokio-rustls" ]
}
tokio = { version = "1", features = ["full"] }
```


## sqlx crate - example

[Runnable project](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/sqlx/create_table_insert_into_select)

Example to create a table, insert data, and select data:

```rust
use sqlx::{SqlitePool, Row};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create an in-memory database
    let db_url = "sqlite::memory:";

    // Create a database connection pool
    let pool = SqlitePool::connect(db_url).await?;

    // Create a table
    sqlx::query("\
        CREATE TABLE IF NOT EXISTS users (\
        id INTEGER PRIMARY KEY NOT NULL,\
        name VARCHAR NOT NULL );\
    ").execute(&pool).await?;

    // Insert data
    sqlx::query("\
        INSERT INTO users values (1, 'Alice'), (2, 'Bob');\
    ").execute(&pool).await?;

    // Select data
    let rows = sqlx::query("\
        SELECT * from users;"
    ).fetch_all(&pool).await?;

    // Print data
    for row in rows {
        println!("id {}, name {:?}",
            row.get::<i32, &str>("id"),
            row.get::<String, &str>("name")
        );
    }
    Ok(())
}
```


## ed25519_dalek crate

<https://crates.io/crates/ed25519_dalek>

The Rust ed25519_dalek crate provides cryptographic message signing, verifying,
and related techniques to ensure the authenticity and integrity of a message. 

Ed25519 is a type of elliptic curve cryptography (ECC) that uses the EdDSA
(Edwards-curve Digital Signature Algorithm) signature scheme, and named after
the prime field of order 2^255-19, which is the field over which the elliptic
curve is defined.

Ed25519 was designed to be a fast, secure, and efficient signature algorithm for
use in a variety of cryptographic applications, including key agreement,
encryption, and authentication. It was developed by Daniel J. Bernstein, Niels
Duif, Tanja Lange, Peter Schwabe, and Bo-Yin Yang.

The crate creates a digital signature, which is a mathematical algorithm that
creates a unique code that represents the contents of the message. The digital
signature is then attached to the message, indicating that the message has not
been altered or tampered with, and that it originated from a particular sender.

When the recipient receives the message, they can use the sender's public key to
decrypt the digital signature, and verify that the message is not altered or corrupted,
and that the message originated from the sender who possesses the private key.

Cryptographic message signing is widely used in electronic communication to
ensure the authenticity and integrity of messages, including email, digital
documents, and online transactions. It provides a way for parties to verify the
identity of each other and ensure that the information exchanged is trustworthy.


## ed25519_dalek crate - example

[Runnable project](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/ed25519-dalek/signing_and_verifying)

Example:

```rust
use rand::rngs::OsRng;

use ed25519_dalek::{
    Signer, SigningKey, Signature, 
    Verifier, VerifyingKey
};

pub fn main() {
    // Create a message that we will sign.
    let message: &[u8] = b"Hello, World!";

    // Load a random number generator (RNG).
    let mut rng = OsRng{};
    
    // Generate a signing key a.k.a. a keypair
    let signing_key: SigningKey = 
        SigningKey::generate(&mut rng);

    // Sign the message
    let signature: Signature = signing_key.sign(message);
    
    // Get the verifying key a.k.a. the keypair private key.
    let verifying_key: VerifyingKey = 
        signing_key.verifying_key();

    // Verify the signature.
    assert!(verifying_key.verify(message, &signature).is_ok());
}
```


# Crates we like for web applications

This page is a section divider and intentionally blank.


## axum crate for web services

<https://crates.io/crates/axum>

The Rust axum crate provides a fast, low-level web framework for building microservices and APIs. Axum is designed to be easy to use, performant, and composable, meaning you can mix and match components to build a custom web application that meets your needs.

Axum is built on top of Rust's async/await syntax and uses Tokio as its underlying async runtime. This means that Axum is well-suited for building high-performance, non-blocking web services that can handle a large number of concurrent requests.

Axum provides a number of features that make it a powerful tool for building web applications, including:

* Routing: Axum makes it easy to define routes for your web application, allowing you to map URLs to specific functions or handlers.

* Middleware: Axum supports middleware, which are functions that can be run before or after a request is processed. Middleware can be used for things like logging, authentication, and authorization.

* Error handling: Axum provides a flexible error handling system that allows you to handle errors in a way that makes sense for your application.

* Testing: Axum includes tools for testing your web application, making it easy to write automated tests for your code.

Overall, Rust axum is well-suited for building microservices and APIs. If you're looking for a fast, low-level framework that gives you complete control over your web application, then Axum is definitely worth considering.


## axum crate - example

[Runnable project](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/axum/hello_world)

Example of using the axum crate to build a web service in Rust:

```rust
use axum::{Router, routing::get};
use std::net::SocketAddr;

async fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

In this example, we define a web service that receives HTTP GET requests, and responds with "Hello, World!".

We define an asynchronous function `hello`. It returns the static string "Hello, World!".

We define a router using the `Router::new()` function, and use the `route()` method to define a route that maps the root URL (`"/"`) to the hello_world handler function.

We create a SocketAddr object representing the address and port on which the web service will listen (`127.0.0.1:3000`), and print a message indicating that the service is listening on that address.

We use the `axum::Server` type to bind the address to the web service, and serve it using the `serve()` method.


## hyper crate for HTTP clients/servers

<https://crates.io/crates/hyper>

The Rust Hyper crate is a popular library for writing HTTP clients and servers in the Rust programming language. It provides a high-level and efficient API for handling HTTP requests and responses, as well as low-level control over the details of the HTTP protocol.

With the Hyper crate, developers can easily build custom HTTP clients and servers, handle HTTP authentication, manage cookies, and perform SSL/TLS encryption. It supports both synchronous and asynchronous programming styles, and is compatible with Rust's built-in async/await syntax.

One of the key advantages of using the Hyper crate is its performance. It's built using Rust's memory safety and zero-cost abstractions, which makes it fast and efficient. Additionally, the Hyper crate is designed to be modular and extensible, which makes it easy to add custom functionality and plugins.

hyper is a relatively low-level library, meant to be a building block for libraries and applications.

* If you want a convenient HTTP client, then consider `reqwest`.

* If you want a convenient HTTP server, then consider `warp`.

Both are built on top of hyper.


## prost crate for protocol buffers

The Rust prost crate is a code generator and runtime library for Protocol Buffers, a language-agnostic data serialization format originally developed by Google. The crate enables Rust developers to define Protocol Buffer messages in Rust code, which are then compiled into serialization and deserialization code at build time.

The generated code provides a strongly-typed API for working with Protocol Buffer messages, enabling efficient serialization and deserialization of data. The runtime library provides functions for reading and writing Protocol Buffer messages to and from files or streams, as well as utilities for working with message fields, extensions and enumerations.

In summary, the Rust prost crate simplifies the process of working with Protocol Buffers in Rust, enabling efficient and safe data serialization and deserialization, while also providing a granular API for working with message fields and structures.



## prost crate - example

[Runnable project](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/prost/serialize_deserialize_protobuf)

Example code to serialize and deserialize a simple protobuf message:

```rust
use prost::Message;

// Define a simple protobuf message
#[derive(Clone, PartialEq, Message)]
pub struct MyMessage {
    #[prost(int32, tag="1")]
    pub my_field: i32,
}

// Serialize the message to bytes
let message = MyMessage { my_field: 42 };
let mut bytes = Vec::new();
message.encode(&mut bytes).unwrap();

// Deserialize the message from bytes
let decoded = MyMessage::decode(bytes.as_slice()).unwrap();

// Verify the decoded message is the same as the original
assert_eq!(message, decoded);
```

In this example code, we define a simple protobuf message `MyMessage` with a single field `my_field`. We use Prost's `#[derive(Message)]` macro to generate the serialization and deserialization code for this message.

We then create an instance of `MyMessage`, serialize it to bytes using the `encode()` method, and deserialize it back to a `MyMessage` instance using the `decode()` method.

Finally, we verify that the serialized and deserialized messages are equal using the `assert_eq()` macro.


## reqwest crate for HTTP requests

<https://crates.io/crates/reqwest>

The Rust reqwest crate is for making HTTP requests. It is built on top of the Rust async runtime, which makes it efficient and suitable for high-performance networking applications.

With reqwest, you can easily make HTTP requests and handle responses in a synchronous or asynchronous manner. The crate provides a set of simple and intuitive APIs for performing HTTP GET, POST, PUT, DELETE, and other types of requests. It also includes support for request/response headers, URL parameters, and request/response bodies.

One of the key features of reqwest is its ability to handle HTTPS connections by default, using the native TLS implementation in Rust. This means that you can securely connect to HTTPS endpoints without having to add any additional dependencies or configuration.

The reqwest crate also includes support for more advanced features like connection pooling, timeouts, cookies, authentication, and logging.

Example:

```rust
use reqwest::Error;

async fn make_request(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://www.example.com";
    let response_body = make_request(url).await?;
    println!("{}", response_body);
    Ok(())
}
```


## Sycamore crate for reactive front-end

<https://crates.io/crates/sycamore>

The Rust sycamore crate provides a reactive front-end web development framework. It uses a Virtual DOM (VDOM) and a declarative syntax that is similar to ReactJS.

The sycamore crate provides various features and functionalities that help developers build fast and high-performance web applications. It supports various event handling, state management, and data flow features.

* Lightning speed: Sycamore harnesses the full power of Rust via WebAssembly, giving you full control over performance.

* Ergonomic and intuitive: Write code that feels natural. Everything is built on reactive primitives without a cumbersome virtual DOM.

* No JavaScript: Create apps using Sycamore without touching a single line of JS.

With sycamore, you can create dynamic and responsive web applications that allow efficient dynamic changes without the need for full-page refreshes. Moreover, sycamore has been designed to be compatible with most of the modern web browsers and it provides a scalable API that can be extended easily.

Example of a sycamore component:

```rust
#[component]
fn Hello<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Hello, World!" }
    }
}
```


## Sycamore crate - example

[Runnable project](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/sycamore/hello_world)

You may want/need to add a WASM target:

```sh
rustup target add wasm32-unknown-unknown
```

Edit file `main.rs` which will contain your website code:

```rust
use sycamore::prelude::*;

#[component]
fn Hello<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Hello, World!" }
    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx, Hello {} }
    });
}
```

Create top-level file `index.html`:

```html
<!DOCTYPE html>
<html>
  <body></body>
</html>
```

Serve the website by using the `trunk` build tool:

```sh
trunk serve
```

Browse <http://localhost:8000>


## Tokio crate for async/concurrency

<https://crates.io/crates/tokio>

The Rust Tokio crate is a widely used library for building asynchronous and concurrent applications. It provides a runtime for executing asynchronous tasks and a set of libraries for building networking and other I/O-heavy applications.

Edit file `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

Example to connect to a mini-redis server:

```rust
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
```


## Tokio crate - example HTTP server

[Runnable project](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/tokio/http_server)

You can use Tokio to build network applications, such as an HTTP server:

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let n = socket.read(&mut buf).await.unwrap();
            let request = String::from_utf8_lossy(&buf[..n]);
            println!("Received request:\n{}", request);

            let response = "HTTP/1.1 200 OK\r\n\r\nHello!";
            socket.write_all(response.as_bytes())
            .await.unwrap();
        });
    }
}
```

This defines a main function that binds to port 8080 and listens for incoming TCP connections.

When a connection is accepted, a new task is spawned to handle the request asynchronously.

The task reads the incoming data from the socket, prints it to the console, and sends a response back to the client.


## tonic crate for gRPC

<https://crates.io/crates/tonic>

The Rust tonic crate is a popular asynchronous gRPC framework that enables developers to quickly build high-performance and scalable client and server applications using Rust programming language.

gRPC is an open-source, high-performance Remote Procedure Call (RPC) framework. It is designed to be efficient and lightweight and uses Protocol Buffers for serialization, which can give 10 times faster performance over REST with JSON. gRPC allows two services to communicate with each other across different languages and platforms.

The tonic crate provides a set of tools for creating and managing gRPC-based APIs, with support for streaming, bidirectional communication, and response compression. Features also include load balancing, custom metadata, authentication, and health checking. It uses asynchronous programming concepts to enable efficient handling of large numbers of concurrent user requests, making it ideal for building real-time communication applications.


### tonic on Tokio

<https://crates.io/crates/tokio>

The Rust tonic crate is built on top of the Tokio runtime, which provides a high-performance, asynchronous, event-driven architecture that enables Rust applications to run efficiently and reliably. 

Tokio also supports a range of programming languages and platforms, including C++, Python, Java, and .NET, making it a versatile solution for building cross-platform applications. F


## tonic crate - example server

Example tonic gRPC server; this code depends on protocol definitions that are in a *.proto file, not shown here. For more, see the Rust Guideposts git repository projects, which has a complete tonic "helloworld" server and client.

```rust
use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
```


## tonic crate - example client

Example tonic gRPC client; this code depends on protocol definitions that are in a *.proto file, not shown here. For more, see the Rust Guideposts git repository projects, which has a complete tonic "helloworld" server and client.

```rust
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
```


## yew crate for client-side web app

The Rust yew crate is a web development framework that allows developing fast and efficient client-side web applications using Rust programming language. The Yew crate uses a virtual DOM implementation to efficiently handle changes and updates to the user interface. 

Key features include:

- Single-file components: This feature allows developers to write reusable code components in a single file, making it easier to manage and maintain the codebase.

- Rust type system: With Rust's type system, developers can write safer, more efficient code with fewer bugs and memory errors.

- Reactive programming: The Yew crate allows developers to use reactive programming, which makes it easier to manage and update the UI automatically as state and data changes.

- Web assembly support: The Rust yew crate supports web assembly, which makes it easier to write web apps that can run natively on desktop and mobile devices.

The Yew crate is similar to React in terms of concepts and usage, but it is written entirely in Rust, and it is optimized for performance and safety.


## yew crate - example

[Runnable project](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/yew/hello_world)

Example of a web app that says "Hello, World!":

```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello, World!" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

This code creates a Yew component that returns HTML, then renders the component to the web page.



# Crates we like for graphics & games

This page is a section divider and intentionally blank.


## gtk4 crate for GTK GUIs

<https://crates.io/crates/gtk4>

The Rust gtk4 crate is a set of bindings to the GTK4 graphical user interface toolkit. GTK4 is a widely used toolkit for creating cross-platform graphical user interfaces. GTK helps developers build modern desktop applications that are runnable on Linux, Windows, and macOS.

The crate provides a set of APIs for building GTK GUIs, including building widgets such as buttons, labels, text fields, menus, scrolling windows, styled look and feel, and more. The crate also offers some Rust-specific abstractions, such as closures and iterators, to make working with the GTK4 toolkit more ergonomic for Rust.

For a gentle introduction to Rust and GTK otegher,  we recommend the online book GUI development with Rust and GTK 4.

<https://gtk-rs.org/gtk4-rs/stable/latest/book/>

Caveats:

* You may need to install additional software. For example, on macOS you can install GTK4 via brew by running `brew install gtk4`.

* GTK is not thread-safe. Accordingly, none of the crate's structs implement the `Send` trait or `Sync` trait.

* The GTK bindings are well supported, although you'll often need to use the C documentation, according to Blessed.rs.

* By default this crate provides only GTK 4.0 APIs. You can access additional functionality by selecting one of the `v4_2`, `v4_4`, etc. features. Take care when choosing the version to target: some of your users might not have easy access to the latest ones.


## gtk4 crate - example

[Runnable project](https://github.com/SixArm/rust-guideposts/tree/main/projects/crates/gtk4/hello_world)

Example "Hello, World!" using GTK from the docs:

```rust
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        // Show the window.
        window.present();
    });

    app.run()
}
```

Notes:

* The gtk4 crate is usually renamed to gtk. You can find an example in the features section for how to do this globally in your Cargo.toml.

* GTK needs to be initialized before use by calling the function `init`. When you create an `Application` struct, this will call `init` for you.


## egui crate for pure Rust GUIs

<https://crates.io/crates/egui>

The Rust egui crate is easy-to-use, lightweight solution for creating graphical user interfaces (GUIs) using pure Rust. The crate uses OpenGL for rendering, which enables it to offer high performance and low resource usage. It also supports a wide range of widgets, such as buttons, text inputs, sliders, and checkboxes, making it a versatile option for creating various types of user interfaces.

One of the key features of the Rust egui crate is its ability to run completely in a web browser without needing any server-side code. This makes it ideal for creating browser applications or web-based dashboards. Additionally, it offers a range of customization options, including the ability to change colors, fonts, and layout of the user interface.

Example excerpt of egui:

```rust
egui::CentralPanel::default().show(ctx, |ui| {
    ui.heading("Hello, World!");
    ui.horizontal(|ui| {
        let name_label = ui.label("Your name: ");
        ui.text_edit_singleline(&mut self.name)
            .labelled_by(name_label.id);
    });
    ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
    ui.label(format!("name '{}', age {}", self.name, self.age));
});
```

The Rust eframe crate is the official egui framework crate. If you are planning to write an app for web or native, and want to use egui for everything, then eframe is for you.

Example excerpt of eframe:

```
let options = eframe::NativeOptions {
    initial_window_size: Some(egui::vec2(320.0, 240.0)),
    ..Default::default()
};
eframe::run_native(
    "My App",
    options,
    Box::new(|_cc| Box::new(MyApp::default())),
)
```


## Bevy crate for game programming

<https://crates.io/crates/bevy>

The Rust bevy crate provides ways to develop high-performance games with modern graphics and audio capabilities. Bevy offers a data-driven, component-based architecture that makes it easy to create and modify game mechanics, systems, and entities in a flexible and scalable manner.

Bevy use an Entity-Component-System (ECS) architecture, which organizes game data into entities that are made up of components, and systems that process those components. ECS allows for powerful and flexible game logic that can be easily extended and modified.

Bevy uses event-driven design. Events that trigger specific actions and behaviors, such as when a user clicks a button or a new object is spawned in the game world. Events can be used to control game flow, trigger animations, or handle user input.

Bevy includes an asset management system that enables developers to easily load and use resources such as textures, meshes, and sound effects within their games.

Bevy includes utility functions and components that make it easy to work with common game development tasks such as handling player movement, collision detection, and input devices.

In addition to its core features, Rust bevy also offers a growing ecosystem of plugins and extensions that expand its capabilities. These include plugins for physics simulations, networking, and more.

Bevy's official website has a Getting Started guide, which provides a step-by-step approach to build a simple game using the Bevy ecosystem, including graphics and user interaction. Bevy's Example Gallery showcases a variety of games and applications, providing the source code to explore and learn from. The Bevy Book is an open-source book that covers in-depth topics on Bevy, including game but also interactive applications design and code.


## Bevy crate - example

<!-- [Runnable project](!projects/crates/bevy) -->

Example sprite animation with a 2D camera view:

```rust
use bevy::{prelude::*, render::camera::OrthographicProjection};
use bevy::render::pass::ClearColor;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(animate.system())
        .run();
}

/// Set up a 2D camera view and a simple white sprite
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let material_handle = materials.add(Color::WHITE.into());
    let sprite_bundle = SpriteBundle {
        material: material_handle,
        sprite: Sprite::new(Vec2::new(100.0, 100.0)),
        ..Default::default()
    };
    commands.spawn_bundle(sprite_bundle);
}

/// Animate a sprite's rotation and scale
fn animate(time: Res<Time>, mut query: Query<&mut Sprite>) {
    for mut sprite in query.iter_mut() {
        let sec = time.seconds_since_startup() as f32;
        let scale = (0.5 * ((secs * 0.5).sin() + 1.0)).max(0.1);
        let rotation = secs * 2.0;
        sprite.scale = Vec2::new(scale, scale);
        sprite.rotate(rotation);
    }
}
```


## macroquad crate for simple games

<https://crates.io/crates/macroquad>

The Rust macroquad crate is a simple and easy to use game library for Rust programming language.

macroquad attempts to avoid any rust-specific programming concepts like lifetimes/borrowing, making it very friendly for rust beginners.

Features:

* Same code for all supported platforms, no platform dependent defines required
* Efficient 2D rendering with automatic geometry batching
* Minimal amount of dependencies: build after cargo clean takes only 16s on x230(~6years old laptop)
* Immediate mode UI library included
* Single command deploy for both WASM and Android build instructions

Example:

```rust
use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
```


# Rust going forward

This page is a section divider and intentionally blank.


## Rust governance

<https://www.rust-lang.org/governance>

Rust governance refers to the system in place for managing and directing the development and maintenance of the Rust programming language. Rust governance is characterized by the following key components:

* RFC process: The Rust RFC (Request for Comments) process is how changes are proposed and accepted into Rust. An RFC is a document proposing a significant change or new feature to Rust, which is rigorously reviewed and discussed by the community before it is accepted or rejected.

* Core team: The Rust core team is responsible for the overall strategy and direction of Rust development, managing the Rust project's infrastructure, and overseeing various Rust teams.

* Teams: There are many different teams within the Rust community that work on specific areas of Rust development, such as the compiler, Cargo (the Rust package manager), documentation, and community outreach. Each team has a leadership structure and is responsible for driving the development of their respective area.

* Working groups: There are a working groups of contributors focused on specific aspects of Rust development, such as asynchronoous programming, command-line interfaces, embedded devices, security responses, and WebAssembly.

* Stewardship: The Rust community has a concept of "stewardship," in which individuals or groups take responsibility for specific areas of Rust development. Stewards are responsible for ensuring that their area of Rust is maintained and developed in a way that aligns with the overall goals and values of Rust.


## The Rust Foundation

The Rust Foundation is a nonprofit organization that was formed in 2020 to manage the development and direction of the Rust programming language.

The Rust Foundation aims to provide an official structure for the Rust community, protect the intellectual property of the language, and oversee the development of Rust. The foundation will allow Rust to be more widely adopted in industry settings, and provide financial support for key initiatives and infrastructure projects.

The Rust Foundation is governed by a board of directors, made up of individuals and organizations that have an interest in the success of Rust. These directors are responsible for setting the direction of the foundation, managing financial resources, and ensuring the continued success of the Rust programming language.

Link: <https://foundation.rust-lang.org/>


### Community Grants Program

The Rust Foundation runs a grants program for the Rust community. These grants are complementing the existing ecosystem of funding for the Rust community, not replacing it. The objectives are to support the maintainers of Rust, and to support and grow the community of Rust users.

Grants include event support grants, Rust Foundation Fellowships, project grants for discrete pieces of work, and more.

Link: <https://foundation.rust-lang.org/grants/>


## The Rust RFC process

<https://rust-lang.github.io/rfcs/>

The Rust RFC (Request For Comments) process is the formal mechanism for proposing, discussing, and deciding on changes to Rust. The process is intended to be transparent, collaborative, and community-driven, and it involves several stages.

1. RFC submission: Anyone can submit an RFC by creating a new issue on the Rust GitHub repository, describing the proposed change, its motivation, potential impact, and alternative solutions.

2. Initial triage: The Rust team evaluates its feasibility, alignment with the language's vision and goals, and potential impact. If the RFC is deemed appropriate for further consideration, it is assigned a tracking issue number and enters the "active" state.

3. Community feedback: The RFC is then open for community feedback and discussion on GitHub. The community can provide comments, suggestions, concerns, and questions, which the RFC author should address and update the RFC accordingly.

4. FCP (Final Comment Period): If there's consensus on the proposed change, then the RFC enters the Final Comment Period (FCP). Stakeholders review and incorporate the feedback, evaluate the consensus, and decide whether to accept, reject, or postpone the RFC.

5. Implementation: If an RFC is accepted, it is assigned to a team or an individual to implement it. The implementation is done on a separate branch or fork of the Rust repository, and it is subject to code review, testing, and community feedback.

6. Merge: Once the implementation is deemed stable, it is merged into the Rust main branch, and becomes part of the next Rust release.


## The Rust roadmap

The Rust roadmap is a high-level plan that outlines the goals, priorities, and direction of the Rust programming language for the next several years. The roadmap is developed by the Rust core team and is informed by community feedback, user needs, and technical challenges.

The Rust roadmap is divided into three main categories:

* Language: This category includes improvements to the Rust language itself, such as adding new syntax, improving performance, and simplifying usage. Some of the specific goals in this category include stabilizing async/await, improving const generics, and adding better support for embedded systems.

* Ecosystem: This category includes improvements to the Rust tooling, libraries, and community, such as making it easier to use Rust for web development, improving the Rust packaging and distribution system, and enhancing the Rust documentation and learning resources.

* Community: This category includes initiatives to make the Rust community more diverse, inclusive, and welcoming, such as improving the Rust governance and decision-making process, promoting Rust education and outreach, and supporting the Rust community events and conferences.

The Rust roadmap is not a fixed plan, and it is subject to change based on feedback and new developments. The Rust core team periodically updates the roadmap to reflect new priorities, challenges, and opportunities. Developers who want to contribute to Rust or use Rust for their projects can consult the roadmap to understand the direction and focus of the language and the community.


# Backmatter

This page is a section divider and intentionally blank.


## About the author

I'm Joel Parker Henderson. I'm a software developer and writer.

https://linkedin.com/in/joelparkerhenderson

https://github.com/joelparkerhenderson


### Professional

For work, I consult for companies that seek to leverage technology capabilities and business capabilities, such as hands-on coding and growth leadership. Clients range from venture capital startups to Fortune 500 enterprises to nonprofit organizations.

For technology capabilities, I host repositories for developers who work with architecture decision records, functional specifications, system quality attributes, git workflow recommendations, monorepo versus polyrepo guidance, and hands-on code demonstrations.

For business capabilities, I host repostitories for managers who work with objectives and key results (OKRs), key performance indicators (KPIs), strategic balanced scorecards (SBS), value stream mappings (VSMs), statements of work (SOWs), and similar practices.
 

### Personal

I'm a strong believer in free libre open source software (FLOSS). I'm an avid traveler and enjoy getting to know new people, new places, and new cultures. I love music and play guitar.

I advocate for chariable donations to help improve our world. Some of my favorite charities are Apache Software Foundation (ASF), Electronic Frontier Foundation (EFF), Free Software Foundation (FSF), Amnesty International (AI), Center for Environmental Health (CEH), Médecins Sans Frontières (MSF), and Human Rights Watch (HRW).


## About the ebook PDF

The Rust Guideposts ebook PDF is generated from the repository markdown files. The process uses custom book build tools, fonts thanks to Adobe, our open source tools, and the program `pandoc`.


### Book build tools

The book build tools are in the repository, in the directory `book/build`. The tools select all the documentation links, merge all the markdown files, then process everything into a PDF file.


### Book fonts

The book fonts are Source Serif Pro, Source Sans Pro, and Source Code Pro. The fonts are by Adobe and are free open source. 


### markdown-text-to-link-urls

<https://github.com/sixarm/markdown-text-to-link-urls>

This is a command-line parsing tool that we maintain. The tool reads markdown text, and outputs all markdown link URLs. We use this to parse the top-level file `README.md`, to get all the links. We filter these results to get the links to individual guidepost markdown files, then we merge all these files into one markdown file.


### pandoc-from-markdown-to-pdf

<https://github.com/sixarm/pandoc-from-markdown-to-pdf>

This is a command-line pandoc tool that we maintain. The tool providers our preferred pandoc settings in order to convert from an input markdown text file to an output PDF file. The tool adds a table of contents, loads our preferred fonts, configures source code syntax highlighting, sets the page size and margins, and more.


## About related projects

Several Rust projects by the same author may be helpful.


### Demo Rust Axum

<https://github.com/joelparkerhenderson/demo-rust-axum>

This project demonstrates Rust and the axum web framework. The project also shows tower for clients and servers, hyper for HTTP, tokio for asynchronous I/O, and Serde for serialization/deserialization.


### Assertables Rust crate

<https://github.com/sixarm/assertables-rust-crate>

The Assertables Rust crate provides many assert macros, such as `assert_starts_with`, `asserts_contains`, and `asserts_is_match`. There are also macros for testing arrays and vectors, function results, readers and streams, and more.


### Collectables Rust crate 

<https://github.com/sixarm/collectibles-rust-crate>

The Collectables Rust crate provides helpers for standard library collections. The crate provides two general-purpose collections helpers: `BTreeMapToSet` based on `BTreeMap` and `BTreeSet`, and `HashMapToSet` based `HashMap` and `HashSet`. 


### checkline Rust crate

<https://github.com/sixarm/checkline-rust-crate>

The `checkline` crate is a command-line checkbox line picker: it reads lines from stdin, prompts the user with a checkbox per line, then outputs lines to stdout. This crate is a good introduction to the Cursive TUI crate.

