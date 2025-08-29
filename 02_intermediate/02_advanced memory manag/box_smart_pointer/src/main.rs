trait UIComponent {
    fn render(&self) {
        println!("Rendering component...");
    }
}

struct Button {
    text: String,
}

impl UIComponent for Button {}

struct Container {
    name: String,
    child: Box<Container>, // allow recursive behavior
}

impl UIComponent for Container {}

fn main() {
    let button_a = Button { text: "button a".to_owned()}; // stored on the stack
    let button_b = Box::new(Button{text: "button b".to_owned()}); // stored on the heap

    let button_c = button_a; // when transferring ownership, the data is copied around on the stack
    let button_d = button_b; // when transferring ownership, only the Box Smart Point will be copied.

    let components: Vec<Box<dyn UIComponent>> = vec![ // with combination with trait objects
        Box::new(button_c),
        button_d,
    ];
}
