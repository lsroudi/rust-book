pub fn demo_let_match(){

    let favorite_color:Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
          println!("Using your favorite color, {}, as the background", color);
      } else if is_tuesday {
          println!("Tuesday is green day!");
      } else if let Ok(age) = age {
          if age > 30 {
              println!("Using purple as the background color");
           } else {
              println!("Using orange as the background color");
           }
      } else {
          println!("Using blue as the background color");
      }
}

pub fn demo_while_match(){

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);


    while let  Some(v) = stack.pop() {
        println!("{} is on the top of the stack", v);
    }

    stack.push(4);
    stack.push(5);
    stack.push(6);
    for (index, value) in stack.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

pub fn demo_match_expr(){
    struct Point {x:i32,y:i32}
    let p = Point { x: 0, y: 7 };

    //Destructuring and matching literal values in one pattern
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }
    
    let origin = Point3d { x: 0, y: 0, z: 0 };
    
    match origin {
        Point3d { x, .. } => println!("x is {}", x),
    }
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32)
     }
     
     enum Message {
         Quit,
         Move { x: i32, y: i32 },
         Write(String),
         ChangeColor(Color),
     }
     

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    // Matching on nested enums
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
    },
    Message::ChangeColor(Color::Hsv(h, s, v)) => {
        println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h,
            s,
            v
        )
    }
    _ => ()
    }

    let numbers = (2, 4, 8, 16, 32);
    // Ignoring multiple parts of a tuple
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");

    }

    println!("{:?}", s);

    match numbers {
        (first,..,last) => {println!("the first element is {} and the last element is {}",first,last);}
    }
     
    let num = Some(4);
    // Extra Conditionals with Match Guards
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let point_s = Point3d { x: 0, y: 0, z: 30 };
    match point_s {
        Point3d{x:0,y:0,z:d_point @ 30..=32} => {
            println!("last number is {}", d_point)
        },
        _=>()
    }


}