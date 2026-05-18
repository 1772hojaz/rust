# Types part 1

## Primitive
* u32 is one of the most primitive types in rust
* Primitive types are the most basic building block of a language, They are built into the language itself thus, they are not defined in terms of other types
* You can combine these primitive types to create more complex types

## Integers
* u32, in particular is an unsigned 32-bit integer
* An integer is a number that can be written without a fractional component. Thus,  1 is an integer and 1.5 is not

## Signed vs Unsigned
* An integer that is unsigned can only represent non-negative(Positive) numbers and signed can represent both positive and negative integers
* The u in u32 is unsigned and the equivalent for signed is i32.

## Bit width
* The 32 in u32 is the number of bits used to represent the number in memory
* the more the bits the larger the range of the numbers that can be represented
* Rust supports multiple bit ranges of integers 8, 16, 32, 64, 128.

## Literals

* A literal is a notation for representing a fixed value in source code for example 42 is a Rust literal for the number forty two


## Type annotations for literals

* But all values in Rust have a type, so whats the type of 42?
* The Rust compiler will try to infer the typr of a literal based on how it is used.
* If no context is provided the compiler wil default to i32
* If you want to use a different type, you can used the desired type as a suffix eg 2u64 is a 2 that is explicitly  typed as a u64

## Underscores in literals
* You can use underscores _ to improvr the readability of large numbers for example 1_000_000 is the same as 1000000

## Arithmetic operators

### Rust supports the following arithmetic operators for integers
* + for addition
* - for sub
* * for mult
* / for div
* % for remainder

## No automatic type coercion

* Rust is very strict when it comes to type coecion, It will not automatically convert a value from one type to another, even if conversion is lossless.
* You have to do it explicitly
* for example you can not assign a u8 value to a variable with type u32 even though all u8 values are valid u32 values







