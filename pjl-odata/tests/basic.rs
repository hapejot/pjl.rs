#[test]

fn query() {
    let q = Person().where( |x| x.name == "peter" );
}
