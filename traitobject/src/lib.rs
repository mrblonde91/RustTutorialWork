//17.02  - Data cannot go into trait objects, they abstract common behaviour
pub mod gui{
    pub trait  Draw {
        fn draw(&self);
    }

    pub struct Screen{
        // draw implementation a requirement for screen
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self){
            for component in self.components.iter(){
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            //implementation
        }
    }
}