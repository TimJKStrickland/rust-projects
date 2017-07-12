fn main() {

	// In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is,
    // with a suffix.
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob" );
}
