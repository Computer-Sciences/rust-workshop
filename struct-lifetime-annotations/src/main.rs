struct Spaceship<'a> {
    // the struct has ownership of that string
    // when the struct goes out of scope,
    // that string data will be dropped and removed from the heap
    name: String,

    // the struct does not own the data referenced by this field
    // there is ambiguity if the struct can still use that reference
    // need to add explicit lifetime annotation
    nickname: &'a str,
}

impl<'a, 'b> Spaceship<'a> {
    // because of lifetime elision rule #3,
    // no need for explicit lifetime annotation
    // the output lifetimes will have the lifetime as the &self parameter

    fn send_transmission(&self, msg: &str) -> (&str, &str) {
        println!("Transmitting message: {}", msg);
        (&self.name, self.nickname)
    }

    // here, because the output lifetime is different from the &self input lifetie,
    // need for explicit lifetime annotation

    fn test_send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message: {}", msg);
        msg
    }
}

fn main() {
    let saucer = Spaceship {
        name: String::from("TR95 MCC Enterprise 9"),
        nickname: "TR95",
    };

    let sender = saucer.send_transmission("All aboard!");
    println!("sender is {}", sender.1);

    let test_msg = saucer.test_send_transmission("this is a test");
    println!("test_msg is {}", test_msg);
}
