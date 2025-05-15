#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::str;

    #[test]
    fn test_hello_world_output() {
        // Build the project
        assert!(Command::new("cargo")
            .args(&["build", "--quiet"])
            .current_dir("..")
            .status()
            .expect("Failed to build the project")
            .success());

        // Run the binary and capture its output
        let output = Command::new("../target/debug/hello_world")
            .current_dir(".")
            .output()
            .expect("Failed to execute the binary");

        // Convert the output to a string
        let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");
        
        // Check that "Hello, World!" appears exactly 5 times
        let hello_count = stdout.matches("Hello, World!").count();
        assert_eq!(hello_count, 5, "Expected 'Hello, World!' to appear exactly 5 times");
        
        // Check that there are exactly 5 lines (one for each greeting)
        let line_count = stdout.lines().count();
        assert_eq!(line_count, 5, "Expected exactly 5 lines of output");
    }
}