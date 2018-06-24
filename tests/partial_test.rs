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
        self.red = Some(100);
        // self.green              <-- this field is uninitialized
        self.blue = Some(200);
    }
}

#[test]
#[should_panic]
fn it_panics_if_a_struct_is_partially_initialized() {
    Color::new();
}
