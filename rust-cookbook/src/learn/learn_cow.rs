#[cfg(test)]
mod test_learn {
    
    use std::borrow::Cow;

    #[test]
    fn test_cow1() {
        let mut cow = Cow::Borrowed("foo");
        let cow_mut = cow.to_mut();
        cow_mut.make_ascii_uppercase();
        println!("{:}", cow_mut);

        assert_eq!(
        cow,
        Cow::Owned(String::from("FOO")) as Cow<'_, str>
        );

    }
}