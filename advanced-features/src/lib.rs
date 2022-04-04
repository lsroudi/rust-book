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