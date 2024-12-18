use std::fs::File;
use std::path::Path;
use std::io::{self, Read};
use num_bigint::{ToBigUint, BigUint};

fn decompress(s: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut i = 0;
    let mut space = false;
    for c in s.chars() {
        let n : usize = c.to_digit(10).unwrap().try_into().unwrap();
        let digit = if space { '.'.to_string() } else { i.to_string() };
        // res.push(digit.to_string().repeat(n));
        for _ in 0..n {
            res.push(digit.to_string());
        }
        if !space {
            i += 1;
        }
        space = !space;
    }
    res
}

fn move_blocks(s: &mut Vec<String>) {
    loop {
        // get last block after spaces
        // if there's one, swap with first space found
        // repeat until theres no block after spaces
        let mut first_space = 0;
        let mut last_block = 0;
        for (i, c) in s.iter().enumerate() {
            if c == "." {
                if first_space == 0 {
                    first_space = i;
                }
            } else {
                last_block = i;
            }
        }
        if first_space > last_block {
            break;
        }
        // let bytes = unsafe { s.as_bytes_mut() };
        // bytes.swap(first_space, last_block);
        s.swap(first_space, last_block);
    }
}

fn checksum(s: &Vec<String>) -> BigUint {
    let mut sum = BigUint::ZERO;
    for (i, c) in s.iter().enumerate() {
        if c == "." {
            continue;
        }
        sum += i * (c.to_string().parse::<u32>().expect(&String::from("should be a number ".to_string() + c))).to_biguint().unwrap();
    }
    sum
}

fn read_input() -> io::Result<String> {
    let mut s = String::new();
    // let path = Path::new("09.example");
    let path = Path::new("09.input");
    let mut file = File::open(&path)?;
    file.read_to_string(&mut s)?;
    Ok(s.trim().to_string())
}

fn part1() -> io::Result<()> {
    let input = read_input()?;
    let mut decompressed = decompress(&input);
    move_blocks(&mut decompressed);
    // println!("{:?}", decompressed);
    let sum = checksum(&decompressed);
    println!("sum = {}", sum);
    Ok(())
}

fn main() -> io::Result<()> {
    part1()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[inline(always)]
    fn split_to_vec_string(s: &str) -> Vec<String> {
        s.chars().map(|c| c.to_string()).collect()
    }

    #[test]
    fn test_decompress() {
        assert_eq!(decompress("12345").join(""), "0..111....22222");
        assert_eq!(decompress("2333133121414131402").join(""), "00...111...2...333.44.5555.6666.777.888899");
        assert_eq!(decompress("233313312141413140212").join(""), "00...111...2...333.44.5555.6666.777.888899.1010");
    }

    #[test]
    fn test_move_blocks() {
        let mut input = split_to_vec_string("0..111....22222");
        move_blocks(&mut input);
        assert_eq!(input.join(""), "022111222......");

        let mut input2 = split_to_vec_string("00...111...2...333.44.5555.6666.777.888899");
        move_blocks(&mut input2);
        assert_eq!(input2.join(""), "0099811188827773336446555566..............");

        let mut input3 = "0 0 . . . 1 1 1 . . . 2 . . . 3 3 3 . 4 4 . 5 5 5 5 . 6 6 6 6 . 7 7 7 . 8 8 8 8 9 9 10"
            .split_whitespace().map(|s| s.to_string()).collect();
        move_blocks(&mut input3);
        assert_eq!(input3.join(""), "001099111888287733374465555666..............");
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum(&split_to_vec_string("0099811188827773336446555566..............")), 1928.to_biguint().unwrap());
    }
}
