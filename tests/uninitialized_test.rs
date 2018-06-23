#[macro_use]
extern crate prismatic;

#[derive(New)]
struct Color {
    _red: usize,
    _green: usize,
    _blue: usize,
}

impl Init {
    fn init(&mut self) {
        self._red = Some(100);
        // self._green              <-- this field is uninitialized
        self._blue = Some(200);
    }
}

#[test]
#[should_panic]
fn it_panics_if_a_field_has_not_been_initialized() {
    Color::new();
}
