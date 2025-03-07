use edm::*;

fn main() {
    let mut s = Schema::new();
    s.new_entity("todo");
    s.new_property("todo", "id");
    s.new_property("todo", "title");
    s.new_property("todo", "descriptions");
    s.new_key("todo", &["id"]);
    println!("{s:#?}");
}
