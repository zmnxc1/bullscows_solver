use std::io;
use std::error::Error;
use std::io::Write;

fn count_element_in_array<T: std::cmp::PartialEq>(list: [T; 4], element: T) -> i32 {
    let mut counter = 0;
    for q in list {
        if q == element {
            counter += 1;
        }
    }

    counter
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut solver = Solver::new();
    
    'gameloop: loop {
        if solver.step() == 4 {
            break 'gameloop;
        }
    }
    
    println!("I won.");

    Ok(())
}
// 4798
struct Solver {
    valid_numbers: Vec<String>,
    guess: String,
}

impl Solver {
    fn new() -> Solver {
        Solver {
            valid_numbers: gen_numbers(),
            guess: String::from("0123"),
        }
    }

    fn renew_valid_numbers(&mut self, result_standard: [i32; 2]) {
        self.valid_numbers.retain(|potential_answer| result_standard == count_bulls_cows_in_guess(&potential_answer, &self.guess));
        // self.step();
    }

    fn step(&mut self) -> i32 {
        let mut input = String::new();
        let quantity_of_valid_numbers = self.valid_numbers.len();
        let mut string_of_quantity_of_valid_numbers = String::new();
        for _q in 0..3 - quantity_of_valid_numbers.ilog10() {
            string_of_quantity_of_valid_numbers.push('0');
        }
        string_of_quantity_of_valid_numbers.push_str(&quantity_of_valid_numbers.to_string());

        print!("valid_numbers: {}. guess: {}. result: ", string_of_quantity_of_valid_numbers, self.guess);
        io::stdout().flush().expect("flush error");
        io::stdin().read_line(&mut input).expect("Input error.");
        input = input.trim().to_string();
        let y: [i32; 2]  = input
            .split('.')
            .map(|q| q.parse::<i32>()
            .expect("Wrong format of result of guess."))
            .collect::<Vec<i32>>()
            .try_into()
            .expect("Error converting vector to array.");
        
        self.renew_valid_numbers(y);
        self.guess = self.valid_numbers[0].clone();

        y[0]
    }
}

fn count_bulls_cows_in_guess(answer: &str, guess: &str) -> [i32; 2] {
    let answer = answer.chars().collect::<Vec<char>>();
    let guess = guess.chars().collect::<Vec<char>>();
    let mut result = [0, 0];

    for (i, digit) in guess.iter().enumerate() {
        if answer.contains(digit) {
            if answer[i] == *digit {
                result[0] += 1;
            } else {
                result[1] += 1;
            }
        }
    }

    result
}

fn gen_numbers() -> Vec<String> {
    let mut results: Vec<String> = Vec::new();

    'iter_all_applicants: for n in 0000..1_0000 {
        let array_of_digits = number_to_array(n);
        for digit in array_of_digits {
            if count_element_in_array(array_of_digits, digit) > 1 {
                continue 'iter_all_applicants;
            }
        }
        results.push(number_to_string(n));
    }

    results
}

fn number_to_string(n: i32) -> String {
    let mut result = String::new();

    if n < 1000 {
        result.push('0');
    }

    result.push_str(&n.to_string());

    result
}

fn number_to_array(n: i32) -> [i32; 4] {
    let mut results = [0; 4];
    for power in 0..4 {
        let digit = n / i32::pow(10, power) - 10 * (n / i32::pow(10, power + 1));
        results[3 - power as usize] = digit;
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_counter_of_entries_of_element_in_array() {
        assert_eq!(count_element_in_array([0, 1, 2, 3], 0), 1);
        assert_eq!(count_element_in_array([5, 5, 5, 5], 5), 4);
        assert_eq!(count_element_in_array([1, 2, 4, 4], 7), 0);
        assert_eq!(count_element_in_array([1, 2, 4, 4], 4), 2);
    }

    #[test]
    fn test_counter_of_bulls_cows_in_guess() {
        assert_eq!(count_bulls_cows_in_guess("3275", "9032"), [0, 2]);
        assert_eq!(count_bulls_cows_in_guess("5287", "5902"), [1, 1]);
        assert_eq!(count_bulls_cows_in_guess("1234", "1234"), [4, 0]);
        assert_eq!(count_bulls_cows_in_guess("7890", "0987"), [0, 4]);
    }
}