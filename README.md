# rust-os-learning

## Rust Fundamental (2024.04.08 - 2024.04.14)

#### Fundamental

	1. Performance
	2. Memory Safety
	3. Type System 
	4. Polymorphism
	5. Extensibility
	6. Build & Package management

	1. Performance
	Minimum Runtime
	Zero cost abstraction
	No Garbage Collector
	2. Memory Safety with ownership and Borrowing
	Ownership Rules
		a. Each value in Rust has a variable that's called its owner
		b. There can only be one owner at a time
		c. When the owner goes out of scope, the value will be dropped
		RC -> Reference Counter
	Borrowing Rules
		a. At any given time, you can have either one mutable reference or any number of immutable references
		b. References must always be valid
	Avoid Data race


	3. Type System
	Immutability & Privacy By Default
	
	No Null Values Options
	
	Enum Option<T> {
		None,
		Some(T),
	}
	
	Mod Environment {
	
		Struct Person {
			
		} 
	}
	
	Explicit Error Handling
	
	Recoverable error, unrecoverable error
	Panic!()
	
	Enum Result<T, E>{
		Ok(T),
		Err€,
	}
	
	Function
	
	Pattern Matching
	
	Closures
	
	let adder :|i32, i32| -> i32;
	
	
	
	Type - enum, struct

	4. Polymorphism with Generics & Traits

	Polymorphism - The ability to substitute multiple objects for each other if they share certain characteristics
	No classical Inheritance
	
	
	Trait
	Trait object
	Box<dyn Item>

	5. Extensibility With Macros
	Meta programming

	6. Building & Package Management

	Cargo - Build Tool, Package Manager
	
	Cargo new
	Cargo run
	Cargo test
	Cargo publish
	
	Crates.io


Start small, grow gradually
![image](https://github.com/liuyanjun/rust-os-learning/assets/1484246/f981d7f9-b2f1-4bc2-8ce6-4f8c067cd556)
#### Raw Pointer


#### rust learning - data structure and algorithm (2024.04.15 - 2024.04.19)
1. Array
2. LinkedList
3. Stack
4. Tree
5. Graph
6. SkipList

#### Rust Async, Await, Futures

#### Rust library Tokio

#### RUST Labmda Programming

#### RISC-V Architecture

#### RISC-V Special Register

#### RISC-V Assumbly Languge




