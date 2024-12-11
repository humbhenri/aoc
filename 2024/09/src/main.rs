use std::fs::File;
use std::path::Path;
use std::io::{self, Read};
use num_bigint::{ToBigUint, BigUint};

fn decompress(s: &str) -> String {
    let mut res = String::new();
    let mut i = 0;
    let mut space = false;
    for c in s.chars() {
        let n : usize = c.to_digit(10).unwrap().try_into().unwrap();
        let digit = if space { '.'.to_string() } else { i.to_string() };
        res.push_str(&digit.to_string().repeat(n));
        if !space {
            i += 1;
        }
        space = !space;
    }
    res
}

fn move_blocks(s: &mut str) {
    loop {
        // get last block after spaces
        // if there's one, swap with first space found
        // repeat until theres no block after spaces
        let mut first_space = 0;
        let mut last_block = 0;
        for (i, c) in s.bytes().enumerate() {
            if c == b'.' {
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
        let bytes = unsafe { s.as_bytes_mut() };
        bytes.swap(first_space, last_block);
    }
}

fn checksum(s: &str) -> BigUint {
    let mut sum = BigUint::ZERO;
    for (i, c) in s.bytes().enumerate() {
        if c == b'.' {
            continue;
        }
        sum += i * (c - b'0').to_biguint().unwrap();
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
    println!("{}", decompressed);
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

    #[test]
    fn test_decompress() {
        assert_eq!(decompress("12345"), "0..111....22222");
        assert_eq!(decompress("2333133121414131402"), "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn test_move_blocks() {
        let mut input = "0..111....22222".to_string();
        move_blocks(&mut input);
        assert_eq!(input, "022111222......");

        let mut input2 = "00...111...2...333.44.5555.6666.777.888899".to_string();
        move_blocks(&mut input2);
        assert_eq!(input2, "0099811188827773336446555566..............");
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum("0099811188827773336446555566.............."), 1928.to_biguint().unwrap());
    }
}
