// src/solution.rs
mod solution;

fn main() {
    println!("\nRunning Tests For TwoSum!!!");

    // Initialize Test Case 0
    let test0_input: Vec<i32> = vec![2,7,11,15];
    let test0_expected: Vec<i32> = vec![0,1];
    let test0_target: i32 = 9;

    // Initialize Test Case 1
    let test1_input: Vec<i32> = vec![3,2,4];
    let test1_expected: Vec<i32> = vec![1,2];
    let test1_target: i32 = 6;

    // Initialize Test Case 2
    let test2_input: Vec<i32> = vec![3,3];
    let test2_expected: Vec<i32> = vec![0,1];
    let test2_target: i32 = 6;

    // Add all "test#_input" vectors to "inputs" vector
    let mut inputs: Vec<Vec<i32>> = Vec::new();
    inputs.push(test0_input);       // Test 0
    inputs.push(test1_input);       // Test 1
    inputs.push(test2_input);       // Test 2

    // Add all "test#_target" vectors to targets vector
    let mut targets: Vec<i32> = Vec::new();
    targets.push(test0_target);     // Test 0
    targets.push(test1_target);     // Test 1
    targets.push(test2_target);     // Test 2

    // Add all "test#_expected" vectors to "expected_outputs" vector
    let mut expected_outputs: Vec<Vec<i32>> = Vec::new();
    expected_outputs.push(test0_expected);      // Test 0
    expected_outputs.push(test1_expected);      // Test 1
    expected_outputs.push(test2_expected);      // Test 2


    if (inputs.len() != expected_outputs.len()) || 
                        (inputs.len() != targets.len()) || 
                        (expected_outputs.len() != targets.len()) {
        println!("number of inputs does not equal number of outputs. Terminating Program.")
        
    } else {
        // Run Each Test Case
        for indx in 0..inputs.len() {
            // pull input and expected output for this test
            let input: &Vec<i32> = &inputs[indx];
            let target: &i32 = &targets[indx];
            let expected: &Vec<i32> = &expected_outputs[indx];

            println!("\n***************************************************");
            println!("{}", String::from("\nTest Case ") + &indx.to_string());

            run_test(input, target, expected);

            println!("\n***************************************************");
        }
    }
}

fn run_test(input: &Vec<i32>, target: &i32, expected: &Vec<i32>) {
    println!("{}{:?}", String::from("\nInput: "), &input);
    println!("{}", String::from("Target: ") + &target.to_string());
    println!("{}{:?}", String::from("Expected: "), &expected);

    let result = solution::solution::Solution::two_sum(input.clone(), target.clone());
    println!("{}{:?}", String::from("Result: "), &result);
}
