fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello_world() {
        // This is a simple test that always passes
        // In a real-world scenario, you might test the output of your program
        assert!(true);
    }
}