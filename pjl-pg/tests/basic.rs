use std::time::Duration;

use postgres::{Client, NoTls};

#[test]
fn connect() {
    let mut client = Client::connect("host=localhost user=postgres password=Kennwort01", NoTls)
        .expect("connect");
    assert!(client.is_valid(Duration::from_secs(1)).is_ok());
}

#[test]
fn simple_select() {
    let mut client = Client::connect(
        "host=localhost user=postgres password=Kennwort01 dbname=rk",
        NoTls,
    )
    .expect("connect");
    assert!(client.is_valid(Duration::from_secs(1)).is_ok());

    for (idx, row) in client.query("SELECT id, name FROM actor", &[]).expect("query").iter().enumerate() {
        let id: &str = row.get(0);
        let name: &str = row.get(1);

        println!("{} {} {}", idx, id, name);
    }
}
