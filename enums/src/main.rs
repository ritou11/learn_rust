fn main() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(u8, u8, u8),
    }

    impl Message {
        fn call(&self) {
            println!("{:?}", &self);
            match &self {
                Message::Move { x, y } => println!("Move to {} {}", x, y),
                Message::Write(s) => println!("Write {}", s),
                Message::ChangeColor(r, g, b) => println!("Change to {} {} {}", r, g, b),
                _ => println!("Quit!"),
            }
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::Move{
        x: 3, y: 4,
    };
    m.call();
    let m = Message::ChangeColor(205, 201, 019);
    m.call();
    Message::Quit.call();
}
