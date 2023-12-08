fn main() {
    println!("Hello, world!");
    // check value of usize::MAX
    println!(
        "usize::MAX == {}, usize::MAX > 0 == {}",
        usize::MAX,
        usize::MAX > 0
    );
    // check value of usize::MIN
    println!(
        "usize::MIN == {}, usize::MIN > 0 == {}",
        usize::MIN,
        usize::MIN > 0
    );

    //check_ownership();
}

pub fn take_ownership_01(s: String) {
    println!("{}", s);
}

pub fn take_ownership_02(s: &String) {
    println!("{}", s);
}

pub fn fn_01(words: Vec<String>) -> Vec<String> {
    let mut tmp = words;
    let s = String::from("hello");
    tmp.push(s);
    tmp
}

pub fn fn_02(mut words: Vec<String>) -> Vec<String> {
    let s = String::from("hello");
    words.push(s);
    words
}

pub fn fn_03(words: &mut Vec<String>) -> &mut Vec<String> {
    let tmp = words;
    let s = String::from("hello");
    tmp.push(s);
    tmp
}

pub fn check_ownership_01() -> u32 {
    let y;
    {
        let x = 5u32;
        y = x;
    } // x is destructed at the end of closure

    println!("y == {}", y);
    y
}

pub fn check_ownership_02() -> u32 {
    let x = 5u32;
    println!("y == {}", x);
    5u32
}

pub fn check_usize_max(max: u64) -> (u64, bool) {
    (max, max > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_ownership_01() {
        let s: String = String::from("hello");
        //take_ownership_01(s);
        //println!("{}", s);
        assert_eq!(take_ownership_01(s), ());
    }

    #[test]
    fn test_take_ownership_02() {
        let s: String = String::from("hello");
        take_ownership_02(&s); // only copies the reference of s, s itself is not destroyed
        println!("{}", s);
        assert_eq!(take_ownership_02(&s), ());
    }

    #[test]
    fn test_fn_01() {
        let mut words: Vec<String> = Vec::new();
        for _ in 1..=10 {
            words = fn_01(words);
        }
        assert_eq!(words.len(), 10);
    }

    #[test]
    fn test_fn_02() {
        let mut words: Vec<String> = Vec::new();
        for _ in 1..=10 {
            words = fn_02(words);
        }
        assert_eq!(words.len(), 10);
    }

    #[test]
    fn test_fn_03() {
        let mut words: &mut Vec<String> = &mut Vec::new();
        for _ in 1..=10 {
            words = fn_03(words);
        }
        assert_eq!(words.len(), 10);
    }


    #[test]
    fn test_check_ownership_01() {
        assert_eq!(check_ownership_01(), 5u32);
    }

    #[test]
    fn test_check_ownership_02() {
        assert_eq!(check_ownership_02(), 5u32);
    }

    #[test]
    fn test_check_usize_max() {
        assert_eq!(
            check_usize_max(usize::MAX.try_into().unwrap()),
            (usize::MAX.try_into().unwrap(), true)
        );
    }
}
