use traitobject::gui::{Screen, Button, Draw};
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe")
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Okay")
            })
        ]
    };
    screen.run();
}


struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        //implementation details
    }
}