pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub fn not(a: bool) -> bool {
    nand(a, a)
}

pub fn and(a: bool, b:bool) -> bool {
    nand(nand(a, b), nand(a, b))
}

pub fn or(a: bool, b: bool) -> bool {
    nand(nand(a, a), nand(b, b))
}

pub fn xor(a: bool, b: bool) -> bool {
    nand(nand(nand(a, b), a), nand(nand(a, b), b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand() {
        assert_eq!(false, nand(true, true));
        assert_eq!(true, nand(true, false));
        assert_eq!(true, nand(false, true));
        assert_eq!(true, nand(false, false));
    }

    #[test]
    fn test_not() {
        assert_eq!(false, not(true));
        assert_eq!(true, not(false));
    }

    #[test]
    fn test_and() {
        assert_eq!(true, and(true, true));
        assert_eq!(false, and(true, false));
        assert_eq!(false, and(false, true));
        assert_eq!(false, and(false, false));
    }

    #[test]
    fn test_or() {
        assert_eq!(true, or(true, true));
        assert_eq!(true, or(true, false));
        assert_eq!(true, or(false, true));
        assert_eq!(false, or(false, false));
    }

    #[test]
    fn test_xor() {
        assert_eq!(false, xor(true, true));
        assert_eq!(true, xor(true, false));
        assert_eq!(true, xor(false, true));
        assert_eq!(false, xor(false, false));
    }
}
