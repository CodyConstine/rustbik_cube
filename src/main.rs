mod cube;

fn main() {
    let test_cube = crate::cube::Cube::make_normal(5);
    println!("Cube {}", test_cube);
}
