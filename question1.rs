// Function to concatenate two string slices and return a new String
fn concatenate_strings(str1: &str, str2: &str) -> String {
    // Create a new String called result
    let mut result = String::new();

    // Append the contents of the first input string slice
    result.push_str(str1);

    // Append the contents of the second input string slice
    result.push_str(str2);

    // Return the result string
    result
}

fn main() {
    // Initialize two String variables
    let string1 = String::from("Hello, ");
    let string2 = String::from("Rust!");

    // Call the concatenate_strings function with references to string1 and string2
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the result to the console
    println!("{}", concatenated_string);
}
