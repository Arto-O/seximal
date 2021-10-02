# Seximal

Seximal is a collection of structs that represent the seximal (base6) equivalents of all the number types in Rust. This project was inspired by [a better way to count](https://www.youtube.com/watch?v=qID2B4MK7Y0) by jan Misali and his [website](https://www.seximal.net) on the subject.

## How to use

Every type in this crate supports arithmetic operations with itself or the equivalent decimal type using the normal arithmetic operators. Every type can also be compared with itself using `std::cmp`.

### Initializing

You can use the `new` function in each struct to create a new instance from a decimal number. Alternatively a new instance can be created from a string representation of a seximal number with the `from` function. Becuse the value is stored internally as a decimal number type, `new` is always the quicker option. However, `from` should be used when creating a new instance from user input, for example, as it performs the conversion from seximal to decimal for you.

### Getting the value

The `value` function in each struct gives you the value of the number in decimal form. Each struct implements `fmt::Display` which returns a string representation of the value in seximal form.

### Conversion

All the integer types have functions for converting between them. You can even convert between signed and unsigned types. The two floating point types support conversions between each other. Be careful, however, as these functions perform just like the `as` keyword, which means that overflow will result in a panic.