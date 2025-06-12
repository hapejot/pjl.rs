
use tokio_postgres::NoTls;

// #[tokio::test]
#[allow(dead_code)]
async fn connect() {
    let (client, _conn) =
        tokio_postgres::connect("host=localhost user=postgres password=Kennwort01", NoTls)
            .await
            .unwrap();
    assert!(!client.is_closed());
}

// #[tokio::test]
#[allow(dead_code)]
async fn simple_select() {
    let (client, _conn) =
        tokio_postgres::connect("host=localhost user=postgres password=Kennwort01", NoTls)
            .await
            .unwrap();
    assert!(!client.is_closed());
    eprintln!("about to select...");
    // let v = client
    //     .query("SELECT count(*) FROM actor", &[])
    //     .await;
    // // for (idx, row) in client
    // //     .query("SELECT id, name FROM actor", &[])
    // //     .await
    // //     .expect("query")
    // //     .iter()
    // //     .enumerate()
    // // {
    // //     let id: &str = row.get(0);
    // //     let name: &str = row.get(1);
    // eprintln!("select {} rows", v.is_ok());

    //     eprintln!("{} {} {}", idx, id, name);
    // }
}
