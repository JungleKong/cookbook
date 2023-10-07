

#[cfg(test)]
mod test_learn {
    use std::fs;

    #[test]
    fn test_fs() {
        let metadata = fs::metadata("src/learn/learn_fs.rs").unwrap();
        println!("metadata: {:#?}", metadata);

    }
}