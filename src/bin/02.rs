use std::io::Read;

fn load_data(fname: &str) -> Result<Vec<[u64; 2]>, std::io::Error> {
    let mut file = std::fs::File::open(fname)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let mut data: Vec<[u64; 2]> = Vec::new();
    for id_pair in text.split(',') {
        let split = id_pair.split('-').collect::<Vec<&str>>();
        if split.len() >= 2 {
            let pair = [
                split[0].parse::<u64>().unwrap(),
                split[1].parse::<u64>().unwrap(),
            ];
            data.push(pair);
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid data",
            ));
        }
    }
    Ok(data)
}

fn is_valid_id_first(id: &str) -> bool {
    let half = id.len() / 2;
    let pattern = &id[0..half];
    return pattern.repeat(2) == id;
}

fn is_valid_id_second(id: &str) -> bool {
    for i in 1..(id.len() / 2) + 1 {
        let pattern = &id[0..i];
        let factor = id.len() / pattern.len();
        if factor * pattern.len() != id.len() {
            continue;
        }
        if pattern.repeat(factor) == id {
            return true;
        }
    }
    return false;
}

fn solve_first(data: Vec<[u64; 2]>) -> u64 {
    data.iter()
        // generate all ids between start and end
        .flat_map(|pair| {
            let start = pair[0];
            let end = pair[1];
            (start..=end).collect::<Vec<u64>>()
        })
        // convert to strings
        .map(|id| id.to_string())
        // get only silly patterns
        .filter(|id| is_valid_id_first(&id))
        // convert to numbers
        .map(|id| id.parse::<u64>().unwrap())
        // sum them
        .sum()
}

fn solve_second(data: Vec<[u64; 2]>) -> u64 {
    data.iter()
        // generate all ids between start and end
        .flat_map(|pair| {
            let start = pair[0];
            let end = pair[1];
            (start..=end).collect::<Vec<u64>>()
        })
        // convert to strings
        .map(|id| id.to_string())
        // get only silly patterns
        .filter(|id| is_valid_id_second(&id))
        // convert to numbers
        .map(|id| id.parse::<u64>().unwrap())
        // sum them
        .sum()
}

fn main() {
    println!("{}", solve_first(load_data("data/02.txt").unwrap()));
    println!("{}", solve_second(load_data("data/02.txt").unwrap()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data() {
        let data = load_data("data/02_test.txt").unwrap();
        assert_eq!(data.len(), 11);
    }

    #[test]
    fn test_solve_first() {
        let data = load_data("data/02_test.txt").unwrap();
        assert_eq!(solve_first(data), 1227775554);
    }

    #[test]
    fn test_solve_second() {
        let data = load_data("data/02_test.txt").unwrap();
        assert_eq!(solve_second(data), 4174379265);
    }
}
