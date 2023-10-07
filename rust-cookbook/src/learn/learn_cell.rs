
#[cfg(test)]
mod test_learn {
    use std::{cell::{Cell, RefCell, OnceCell}, ops::Add};

    #[test]
    fn test_cell() {
        let c = Cell::new(5);

        // 只能得到cell内部值的拷贝，无法获取可变引用
        let mut five = c.get();
        five += 1;
        assert_eq!(five, 6);
        assert_eq!(c.get(), 5);  // 原始值不变

        c.set(10);
        let ten = c.get();
        println!("{}", ten);
    }

    #[test]
    fn test_refcell() {
        let mut c = RefCell::new(5);

        // 存在可变借用，不能再次借用，要控制可变借用的生命周期
        {
            // 可以得到cell内部值的可变引用
            let mut five = c.borrow_mut();
            *five += 1;
            assert_eq!(*five, 6);
        }
        // 再次借用
        assert_eq!(*c.borrow(), 6);  // 原始值也变了

        let c_add = c.get_mut().add(10);
        assert_eq!(c, RefCell::new(6));
        assert_eq!(c_add, 16);

        {
            // 可以得到cell内部值的可变引用
            let mut five = c.borrow_mut();
            *five += 1;
            assert_eq!(*five, 7);
        }
    }

    #[test]
    fn test_oncecell() {
        // 只能被初始化一次
        let o: OnceCell<_> = OnceCell::new();
        assert!(o.get().is_none());
        assert_eq!(o.get_or_init(|| 92), &92);
        assert_eq!(o.set(10), Err(10));

        // 只有get之后才能重新set
        // 被当作只有一个容器的生产者消费者？
        // 组合起来可以当作有很多个容器？
        // 效率？
        o.get().map(|v| assert_eq!(v, &92));
        assert!(o.set(10).is_ok());

    }

}