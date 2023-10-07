

#[cfg(test)]
mod test_learn {
    use std::collections::BTreeSet;

    #[test]
    fn test1() {
        // 内部使用BTree实现，有序的，没有重复的元素
        let mut set = BTreeSet::new();
        set.insert(2);
        set.insert(1);
        set.insert(3);
        set.insert(4);
        set.insert(2);

        set.iter().for_each(|x| println!("{}", x));
    }
}