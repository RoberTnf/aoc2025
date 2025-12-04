use std::io::Read;

fn main() {
    let data = load_data("data/04.txt").unwrap();
    println!("{}", solve_first(data.clone()));
    println!("{}", solve_second(data));
}

fn load_data(fname: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let mut file = std::fs::File::open(fname)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in text.split('\n') {
        data.push(line.chars().collect());
    }
    Ok(data)
}

fn solve_first(data: Vec<Vec<char>>) -> u64 {
    let mut counter: i64 = 0;
    for i in 0..data.len() {
        let row = &data[i];
        for j in 0..row.len() {
            if row[j] != '@' {
                continue;
            }

            let mut neighbors: i64 = 0;
            for di in -1..=1 {
                for dj in -1..=1 {
                    let new_i = i as i64 + di;
                    let new_j = j as i64 + dj;
                    if new_i < 0
                        || new_i >= data.len() as i64
                        || new_j < 0
                        || new_j >= row.len() as i64
                        || (new_i == i as i64 && new_j == j as i64)
                    {
                        continue;
                    }
                    if data[new_i as usize][new_j as usize] == '@' {
                        neighbors += 1;
                    }
                }
            }
            if neighbors < 4 {
                counter += 1;
            }
        }
    }
    counter.try_into().unwrap()
}

fn solve_second(mut data: Vec<Vec<char>>) -> u64 {
    let mut result = 0;
    loop {
        let counter = count_and_remove(&mut data);
        if counter == 0 {
            break;
        }
        result += counter;
    }
    result
}

fn count_and_remove(data: &mut Vec<Vec<char>>) -> u64 {
    let mut counter: i64 = 0;
    let static_data = data.clone();
    for i in 0..static_data.len() {
        let row = &static_data[i];
        for j in 0..row.len() {
            if row[j] != '@' {
                continue;
            }

            let mut neighbors: i64 = 0;
            for di in -1..=1 {
                for dj in -1..=1 {
                    let new_i = i as i64 + di;
                    let new_j = j as i64 + dj;
                    if new_i < 0
                        || new_i >= static_data.len() as i64
                        || new_j < 0
                        || new_j >= row.len() as i64
                        || (new_i == i as i64 && new_j == j as i64)
                    {
                        continue;
                    }
                    if static_data[new_i as usize][new_j as usize] == '@' {
                        neighbors += 1;
                    }
                }
            }
            if neighbors < 4 {
                counter += 1;
                data[i][j] = '.';
            }
        }
    }
    counter.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data() {
        let data = load_data("data/04_test.txt").unwrap();
        assert_eq!(data.len(), 10);
    }

    #[test]
    fn test_solve_first() {
        let data = load_data("data/04_test.txt").unwrap();
        assert_eq!(solve_first(data), 13);
    }

    #[test]
    fn test_solve_second() {
        let data = load_data("data/04_test.txt").unwrap();
        assert_eq!(solve_second(data), 43);
    }
}
