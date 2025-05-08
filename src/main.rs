fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello_world() {
        // This is a simple test to verify the application compiles
        // In a real-world scenario, we would test actual functionality
        assert_eq!(2 + 2, 4);
    }
}