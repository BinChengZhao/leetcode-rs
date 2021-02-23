mod thread;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_match() {
        let a = match 1 {
            1 => 2,
            2 => 3,
            _ => 4,
        };
        println!("{:?}", a);
        assert_eq!(a, 2);
    }
}
