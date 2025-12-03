use std::io::Read;

fn load_data(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text.split('\n').map(|line| line.to_string()).collect())
}

fn main() {
    let data = load_data("data/03.txt").unwrap();
    println!("{}", solve_first(data.clone()));
    println!("{}", solve_second(data));
}

fn solve_first(data: Vec<String>) -> u32 {
    data.iter()
        .map(|line| {
            line.chars().fold(('0', '0'), |(a, b), c| {
                // pick the biggest of (a,b), (a, c), (b, c)
                let (a, b) = {
                    if b > a {
                        (b, c)
                    } else if c > b {
                        (a, c)
                    } else {
                        (a, b)
                    }
                };
                (a, b)
            })
        })
        .map(|(a, b)| a.to_digit(10).unwrap() * 10 + b.to_digit(10).unwrap())
        .sum()
}

fn solve_second(data: Vec<String>) -> u64 {
    // now keep any 12 digits out of the N existing, in order
    let result: Vec<String> = data
        .iter()
        .map(|line| get_biggest_out_of_n(line.clone(), 12))
        .collect();

    //println!("result: {:?}", result);

    result.iter().map(|line| line.parse::<u64>().unwrap()).sum()
}

fn get_biggest_out_of_n(data: String, n: usize) -> String {
    // Recursive solution.
    // To start, get the data.len() - n first characters.
    // Then, get the biggest character in the data and its index.
    // Throw away anything to the left of the biggest character and apply recursively
    // with n - 1

    //println!("data: {}, n: {}", data, n);

    let to_compare_length = (data.len() + 1) - n;
    let str_to_look_at = &data[..to_compare_length];
    //println!("str_to_look_at: {}", str_to_look_at);

    let biggest = str_to_look_at
        .chars()
        .fold('0', |a, c| if c > a { c } else { a });

    if n == 1 {
        return biggest.to_string();
    }

    let start_index = str_to_look_at.chars().position(|c| c == biggest).unwrap();
    //println!("start_index: {}, biggest: {}", start_index, biggest);

    biggest.to_string() + &get_biggest_out_of_n(data[start_index + 1..].to_string(), n - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data() {
        let data = load_data("data/03_test.txt").unwrap();
        assert_eq!(data.len(), 4);
    }

    #[test]
    fn test_solve_first() {
        let data = load_data("data/03_test.txt").unwrap();
        assert_eq!(solve_first(data), 357);
    }

    #[test]
    fn test_solve_second() {
        let data = load_data("data/03_test.txt").unwrap();
        assert_eq!(solve_second(data), 3121910778619);
    }
}
