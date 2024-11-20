/*
This example code counts the frequency of each number in the vector.
 */
use std::collections::HashMap;

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        // * dereferences the pointer to the value
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let mut result = logic(numbers);
    // sort result by frequency
    result.sort_by(|a, b| b.1.cmp(&a.1));
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logic() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
        let mut result = logic(numbers);
        result.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(result, vec![(1, 2), (2, 1), (3, 2), (4, 1), (5, 1), (6, 1), (7, 1), (8, 1), (9, 1)]);
    }

    #[test]
    fn test_logic_empty_numbers() {
        let numbers = vec![];
        let result = logic(numbers);
        assert_eq!(result, vec![]);
    }
}
