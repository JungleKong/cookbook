
#[cfg(test)]
mod test_learn {
    
    #[test]
    fn test_pos() {
        let s = "今天周五";
        let mut pos = 0;
        for c in s.chars() {
            println!("{}: {}", pos, c);
            pos += c.len_utf8();
        }
        
        let mut index = s.char_indices();
        assert_eq!(Some((0, '今')), index.next());
        
        // 快速定位字符串中的第三个字符
        let index = s.char_indices().nth(2);
        assert_eq!(Some((6, '周')), index);

        // 快速定位字符串中的最后一个字符
        let index = s.char_indices().last();
        assert_eq!(Some((9, '五')), index);
    }
}