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
        self.set_red(100);

        let green = self.red() + 50;
        self.set_green(green);

        let blue = self.red() + self.green() - 50;
        self.set_blue(blue);
    }
}

#[test]
fn it_uses_getters_and_setters_to_initialize_the_struct() {
    let color = Color::new();

    assert_eq!(color.red, 100);
    assert_eq!(color.green, 150);
    assert_eq!(color.blue, 200);
}
