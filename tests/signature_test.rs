#[macro_use]
extern crate prismatic;

#[derive(New)]
#[Sig = "green: usize, blue: usize"]
struct Color {
    red: usize,
    green: usize,
    blue: usize,
}

impl Init {
    fn init(&mut self, green: usize, blue: usize) {
        self.red = Some(100);
        self.green = Some(green);
        self.blue = Some(blue);
    }
}

#[test]
fn it_can_use_a_custom_method_signature() {
    let color = Color::new(150, 200);

    assert_eq!(color.red, 100);
    assert_eq!(color.green, 150);
    assert_eq!(color.blue, 200);
}
