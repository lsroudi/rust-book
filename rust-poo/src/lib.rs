
pub fn trait_common_behavior() {

    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        //This vector is of type Box<dyn Draw>, which is a trait object; it’s a stand-in for any type inside a Box that implements the Draw trait.
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self){

            for component in self.components.iter() {
                component.draw();
            }

        }
    }

    pub struct Button{
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self){
            println!("draw in Button was called");
        }
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }
    
    impl Draw for SelectBox {
        fn draw(&self) {
            println!("draw in SelectBox was called");
        }
    }
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                 ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
      ],

   };

   screen.run();
}

pub fn trait_common_behavior_alternative() {
    
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen<T:Draw>
    {
        pub components: Vec<T>,
    }

    impl<T> Screen<T>
    where T:Draw{
        pub fn run(&self){

            for component in self.components.iter() {
                component.draw();
            }

        }        
    }

    
}