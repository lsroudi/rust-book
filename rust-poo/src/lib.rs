
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

pub fn blog_state_demo(){

    trait State{
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Draft{}
    struct PendingReview {}
    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self

        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self

        }
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
          }
    }
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State>{
            Box::new(PendingReview {})
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State>{
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    pub struct Post {
        state : Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new()-> Self{
            Self{
                state : Some(Box::new(Draft {})),
                content : String::new(),
            }
        }

        pub fn add_text(&mut self,text: &str){
            self.content.push_str(text);
        }

        pub fn content(&self)->&str{
            self.state.as_ref().unwrap().content(&self)
        } 

        pub fn request_review(&mut self){
            if let Some(s) = self.state.take(){
                self.state = Some(s.request_review())
            }
        }
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }


    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

