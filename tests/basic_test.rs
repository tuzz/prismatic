#[macro_use]
extern crate prismatic;

#[derive(New)]
struct Color {
    red: usize,
    green: usize,
    blue: usize,
}

impl Init {
    fn init(&mut self) {
        self.red = Some(100);
        self.green = Some(150);
        self.blue = Some(200);
    }
}

#[test]
fn it_initializes_the_struct() {
    let color = Color::new();

    assert_eq!(color.red, 100);
    assert_eq!(color.green, 150);
    assert_eq!(color.blue, 200);
}
