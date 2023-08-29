use std::any::Any;

pub trait Animal {
    fn voice(&self);

    // 是否能够直接在Trait里面实现ass_any()方法，而不是在具体的实现里面实现
    fn as_any(&self) -> &dyn Any;
}

pub struct Dog;

impl Dog {
    pub fn checkout_house(&self) {
        println!("Dog checkout house");
    }
}

impl Animal for Dog {
    fn voice(&self) {
        println!("Wang Wang Wang");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Cat;

impl Cat {
    pub fn catch_mouse(&self) {
        println!("Cat catch mouse");
    }
}

impl Animal for Cat {
    fn voice(&self) {
        println!("Miao Miao Miao");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}



pub fn r#do(cat: &Cat) {
    cat.catch_mouse();
    cat.voice();
}

pub fn do_impl(ani: &impl Animal) {
    ani.voice();

    // impl Animal 没有实现checkout_house方法，所以不能使用
    // 只能调用Trait里面定义的方法
    // ani.catch_mouse();
    // ani.checkout_house();

    ani.as_any().downcast_ref::<Cat>().map(|cat| cat.catch_mouse());
    ani.as_any().downcast_ref::<Dog>().map(|dog| dog.checkout_house());
}


pub fn do_box(ani: Box<dyn Animal>) {
    ani.voice();

    // dyn Animal 没有实现checkout_house方法，所以不能使用
    // 只能调用Trait里面定义的方法
    // ani.catch_mouse();

    // 但是可以使用as_any()方法，转换成Any类型，然后再使用downcast_ref转换成具体的类型
    ani.as_ref()
       .as_any()
       .downcast_ref::<Cat>()
       .map(|cat| cat.catch_mouse());

    ani.as_ref()
       .as_any()
       .downcast_ref::<Dog>()
       .map(|dog| dog.checkout_house());
}

#[cfg(test)]
mod test_learn {
    use super::*;

    #[test]
    fn test1() {
        let cat = Cat;
        cat.catch_mouse();
        cat.voice();

        let dog = Dog;
        dog.checkout_house();
        dog.voice();
        do_impl(&dog);
        do_box(Box::new(dog));  // Box::new(dog)会转换成Box<dyn Animal>,拿走dog的所有权
    }

    #[test]
    fn test2() {
        let cat = Box::new(Cat);
        let dog = Box::new(Dog);
        
        r#do_box(cat);
        r#do_box(dog);
    }

    #[test]
    fn test3() {
        let cat = Cat;
        let dog = Dog;
        
        do_impl(&cat);
        do_impl(&dog);
    }
}