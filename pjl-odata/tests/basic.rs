use std::collections::HashMap;
use pjl_odata::ODataQuery;

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
            format!("{:?}", q.conditions().get("alter")),
            "[Condition { op: Var(\"ge\"), value: \"18\" }]"
        );
        assert_eq!(
            format!("{:?}", q.conditions().get("name")),
            "[Condition { op: Equals, value: \"Hans\" }]"
        );
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
        // assert_eq!(
        //     "ConditionBag { fields: {\"alter\": [Condition { op: Var(\"ge\"), value: \"18\" }], \"name\": [Condition { op: Equals, value: \"Hans\" }, Condition { op: Equals, value: \"Peter\" }]} }",
        //     format!("{:?}", q.conditions())
        // );
        assert_eq!(2, q.conditions().field_count());
        assert_eq!(
            format!("{:?}", q.conditions().get("alter")),
            "[Condition { op: Var(\"ge\"), value: \"18\" }]"
        );
        assert_eq!(format!("{:?}",q.conditions().get("name")),"[Condition { op: Equals, value: \"Hans\" }, Condition { op: Equals, value: \"Peter\" }]");
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
