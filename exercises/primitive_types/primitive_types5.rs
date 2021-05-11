// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let new_cat = cat;
    let name = new_cat.0;
    let age = new_cat.1;

    println!("{} is {} years old.", name, age);
}
