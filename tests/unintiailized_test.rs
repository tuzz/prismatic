#[macro_use]
extern crate prismatic;

#[derive(New)]
#[allow(dead_code)]
struct Color {
    red: usize,
    green: usize,
    blue: usize,
}

impl Init {
    fn init(&mut self) {
        let x = self.green().clone(); //    <-- this field is uninitialized

        self.red = Some(x);
        self.green = Some(150);
        self.blue = Some(200);
    }
}

#[test]
#[should_panic]
fn it_panics_when_trying_to_access_an_uninitialized_field() {
    Color::new();
}
