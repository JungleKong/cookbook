
#[allow(dead_code)]
fn insert_1000000() {
    let mut v = Vec::new();
    for i in 0..1000000 {
        v.push(i);
        v.push(i + 1);
    }
}

#[allow(dead_code)]
fn insert_1000000_with_capacity() {
    let mut v = Vec::with_capacity(2000000);
    for i in 0..1000000 {
        v.push(i);
        v.push(i + 1);
    }
}


#[cfg(test)]
mod test_vec {
    use std::time::Instant;

    use super::*;

    // 测试插入1000000个元素，
    // 1. 使用Vec::new()创建Vec
    // 2. 使用Vec::with_capacity(1000000)创建Vec避免重新分配与拷贝
    // 有编译器的优化？？？
    #[test]
    fn test_insert() {
        let start = Instant::now();
        for _ in 0..1 {
            insert_1000000();
        }
        println!("insert_1000000: {:?}", start.elapsed().as_millis());

        let start = Instant::now();
        for _ in 0..1 {
            insert_1000000_with_capacity();
        }
        println!("insert_1000000_with_capacity: {:?}", start.elapsed().as_millis());
    }


    // 使用assert提前判断vec大小，避免每次get都检查边界
    // 但是效果貌似不理想
    // 另外assert只会在debug模式下生效，release模式下不会生效
    #[test]
    fn test_get() {
        let mut v = Vec::new();
        for i in 0..1000000 {
            v.push(i as f64);
        }

        let start = Instant::now();
        assert!(v.len() >= 1000000);
        for i in 0..1000000 {
            if let Some(_value) = v.get(i) {
                // do something
            }
        }
        println!("test_get: {:?}", start.elapsed().as_millis());

        let start = Instant::now();
        for i in 0..1000000 {
            if let Some(_value)  = v.get(i) {
                // do something
            }
        }
        println!("test_get: {:?}", start.elapsed().as_millis());
    }

    #[test]
    fn test_vec() {
        let start = Instant::now();
        for i in 0..1 {
            let _v = std::vec::from_elem(i, 1000000);
        }
        println!("std::vec::from_elem: {:?}", start.elapsed().as_millis());

        let v = std::vec::from_elem(10, 5);
        assert_eq!(v, [10, 10, 10, 10, 10]);
        assert_eq!(v.len(), 5);
        
    }
}