fn main() {
    println!("Hello, world!");
}

fn next_value(seq: &[u32]) -> u32 {
    if seq.iter().all(|&x| x == 0) {
        return 0;
    }
    seq.last().unwrap() + next_value(&sequence_below(seq))
}

fn sequence_below(seq: &[u32]) -> Vec<u32> {
    if seq.iter().all(|&x| x == 0) {
        return Vec::from(seq);
    }
    let rest = &seq[1..];
    seq.iter()
        .zip(rest.iter())
        .map(|(a, b)| (b - a))
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_below() {
        assert_eq!(vec![1, 1], sequence_below(&[1, 2, 3]));
        assert_eq!(vec![3, 3, 3, 3, 3], sequence_below(&[0, 3, 6, 9, 12, 15]));
        assert_eq!(vec![0, 0, 0, 0], sequence_below(&[0, 0, 0, 0]));
    }

    #[test]
    fn test_next_value() {
        assert_eq!(18, next_value(&[0, 3, 6, 9, 12, 15]));
        assert_eq!(0, next_value(&[0, 0, 0]));
        assert_eq!(28, next_value(&[1, 3, 6, 10, 15, 21]));
    }
}
