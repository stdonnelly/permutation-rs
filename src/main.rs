use std::io::{stdin, stdout, Write};

fn main() {
    let mut n = String::new();
    print!("n: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().expect("Not an integer");

    let mut r = String::new();
    print!("r: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut r).unwrap();
    let r: i32 = r.trim().parse().expect("Not an integer");

    let my_vec: Vec<i32> = (0..n).collect();
    let permuted_vecs: Vec<Vec<i32>> = permute(&my_vec, r);

    println!("Permuted vector: {}", print_nested_vec(&permuted_vecs));
    println!("Length: {}", permuted_vecs.len());
}

/// Generate permutations recursively
fn permute(elements: &Vec<i32>, r: i32) -> Vec<Vec<i32>> {
    let mut set_set = Vec::<Vec<i32>>::new();

    // Length of list
    let n = elements.len();

    // n is the length of the input, r is the length of the output.
    // If n < r, panic
    if n < r as usize {
        panic!("The size of the input set must be >= r");
    }

    // If r is 0, return [[]]
    if r <= 0 {
        return set_set;
    }

    // For every element in the list, let that element be first
    for index in 0..n {
        // Take an element from the set
        let mut my_set = elements.to_owned();
        let my_element = my_set.remove(index);

        // At this point, my_set is A and not {my_element}
        // Now permute the rest
        let other_sets = permute(&my_set, r - 1);
        if other_sets.len() <= 0 {
            set_set.push(vec![my_element]);
        } else {
            for mut other_set in other_sets {
                other_set.insert(0, my_element);

                set_set.push(other_set);
            }
        }
    }

    return set_set;
}

/// Turns a vector into a string in the format \[1, 2, 3,...\]
fn print_vec(input: &Vec<i32>) -> String {
    let mut output = String::new();

    output.push('[');

    if input.len() > 0 {
        output.push_str(&input[0].to_string());
        for in_str in &input[1..] {
            output.push_str(", ");
            output.push_str(&in_str.to_string());
        }
    };

    output.push(']');

    return output;
}

/// Nested vector version of print_vec()
fn print_nested_vec(input: &Vec<Vec<i32>>) -> String {
    let mut output = String::new();

    output.push_str("[\n");

    if input.len() > 0 {
        output.push_str("  ");
        output.push_str(&print_vec(&input[0]));
        for in_vec in &input[1..] {
            output.push_str(",\n  ");
            output.push_str(&print_vec(&in_vec));
        }
    };

    output.push_str("\n]");

    return output;
}
