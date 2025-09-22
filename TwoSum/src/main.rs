// src/solution.rs
mod solution;

fn main() {
    println!("\nRunning Tests For TwoSum!!!");

    // Initialize Test Case 1
    let test1_input = vec![2,7,11,15];
    let test1_expected = vec![0,1];

    // Initialize Test Case 2
    let test2_input = vec![3,2,4];
    let test2_expected = vec![1,2];

    // Initialize Test Case 3
    let test3_input = vec![3,3];
    let test3_expected = vec![0,1];

    // Add all "test#_input" vectors to "inputs" vector
    let mut inputs: Vec<Vec<i32>> = Vec::new();
    inputs.push(test1_input);
    inputs.push(test2_input);
    inputs.push(test3_input);

    // Add all "test#_expected" vectors to "expected_outputs" vector
    let mut expected_outputs: Vec<Vec<i32>> = Vec::new();
    expected_outputs.push(test1_expected);
    expected_outputs.push(test2_expected);
    expected_outputs.push(test3_expected);

    if inputs.len() != expected_outputs.len() {
        println!("number of inputs does not equal number of outputs")
        
    } else {
        // Run Each Test Case
        for indx in 0..inputs.len() {
            // TODO: implement
        }
    }
    
    
}
