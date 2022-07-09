fn main() {
    println!("Hello World");
    // the `{}` will be automatically replaced with any arguments.
    println!("{} days", 31);

    // Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments
    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");

    // different formatting
    println!("Base 10 repr: {}", 69420);
    println!("Base 2 (binary) repr: {:b}", 69420);
    println!("Base 8 (octal) repr: {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);

    // This will output "     1". 5 white spaces and a "1".
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:0>5}", number=1);
    
    // You can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number=1, width=5);

    println!("My name is {0}, {1} {0}", "James","Bond");
}