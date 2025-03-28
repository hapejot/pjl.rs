use pjl_odata::ODataQuery;
use std::collections::HashMap;

#[test]
fn query() {
    // let q = Person().filter( |x| x.name == "peter" );
}

#[test]
fn filter() {
    let params: HashMap<String, String> =
        HashMap::from([("$filter".into(), "Name eq 'Hans' and Alter ge 18".into())]);
    let q = ODataQuery::new_from("person", &params);
    assert_eq!(None, q.skip());
    assert_eq!(None, q.orderby());
    assert_eq!(2, q.conditions().field_count());
    assert_eq!(
        format!("{}", q.conditions().get("alter").first().unwrap()),
        "... ge 18"
    );
    assert_eq!(
        format!("{}", q.conditions().get("name").first().unwrap()),
        "... = 'Hans'"
    );
}

fn format_vector<A>(v: Vec<A>) -> String
where
    A: std::fmt::Display,
{
    let mut r = String::new();
    r.push_str("[");
    let mut sep = "";
    for x in v.iter() {
        r.push_str(&format!("{}{}", sep, x));
        sep = ", ";
    }
    r.push_str("]");
    r
}

#[test]
fn filter2() {
    let params: HashMap<String, String> = HashMap::from([(
        "$filter".into(),
        "Name eq 'Hans' or Name eq 'Peter' and Alter ge 18".into(),
    )]);
    let q = ODataQuery::new_from("person", &params);
    assert_eq!(None, q.skip());
    assert_eq!(None, q.orderby());
    assert_eq!(2, q.conditions().field_count());
    assert_eq!(format!("{}", format_vector(q.conditions().get("alter"))), "[... ge 18]");
    assert_eq!(
        format!("{}", format_vector(q.conditions().get("name"))),
        "[... = 'Hans', ... = 'Peter']"
    );
}

#[test]
fn filter_sql_where() {
    let params: HashMap<String, String> = HashMap::from([(
        "$filter".into(),
        "Name eq 'Hans' or Name eq 'Peter' and Alter ge 18".into(),
    )]);
    let q = ODataQuery::new_from("person", &params);
    let w = q.get_where_sql();
    assert_eq!("alter >= 18 and (name = 'Hans' or name = 'Peter')", w);
}

#[test]
fn boolean_expression() {
    let params: HashMap<String, String> =
        HashMap::from([("$filter".into(), "active eq true or active eq 1".into())]);
    let q = ODataQuery::new_from("person", &params);
    let w = q.get_where_sql();
    assert_eq!("(active = true or active = 1)", w);
}
