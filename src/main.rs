use crate::cube::Cube;

mod cube;

fn main() {
    let cube = Cube::default();
    cube.print_terminal();
}
