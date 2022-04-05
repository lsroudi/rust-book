pub fn demo_raw_pointers(){
    let mut num = 10;

    // create a raw pointer from i32 type
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    //create a raw pointer to an arbitrary memory address
    let address = 0x012545usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        println!("r is: {}", *r);
    
    }

}

pub fn demo_trait_associated_type(){
    
    struct Counter {}
    pub trait Iterator {
        type Item;
    
        fn next(&mut self) -> Option<Self::Item>;
    }

    impl Iterator for Counter {
        type Item = u32;
    
        fn next(&mut self) -> Option<Self::Item> {
            Some(5)
        }
    }

    let mut count = Counter { };
    println!("associated type demo is {:?} ",count.next());

    trait Animal {
        fn baby_name() -> String;
    }
    
    struct Dog;
    
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
       }
    }
    
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    
}

pub fn demo_super_trait(){
    
    use std::fmt;
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    #[derive(Debug)]    
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl OutlinePrint for Point {}

    impl fmt::Display for Point{

        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
       }
    }

    let p = Point{x:3,y:6};
    p.outline_print();
}

// function pointer
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn demo_function_pointer() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}


