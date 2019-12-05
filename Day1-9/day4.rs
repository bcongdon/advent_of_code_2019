fn is_sorted(s: &String) -> bool {
    s.as_bytes().windows(2).all(|w| w[0] <= w[1])
}

fn p1_valid(s: &String) -> bool {
    s.as_bytes().windows(2).any(|w| w[0] == w[1])
}

fn p2_valid(s: &String) -> bool {
    let mut i = s.as_bytes().iter().peekable();

    while let Some(ch) = i.next() {
        let mut count = 1;
        while let Some(n) = i.peek() {
            if ch == *n {
                count += 1;
                i.next();
            } else {
                break;
            }
        }
        if count == 2 {
            return true;
        }
    }
    false
}

fn main() {
    let sorted: Vec<String> = (236491..713787)
        .map(|x| x.to_string())
        .filter(|x| is_sorted(x))
        .collect();
    let part1 = sorted.to_vec().iter().filter(|x| p1_valid(x)).count();
    println!("Part 1: {}", part1);

    let part1 = sorted.iter().filter(|x| p2_valid(x)).count();
    println!("Part 1: {}", part1);
}
