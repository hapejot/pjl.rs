use dbx::data::model::FieldType::Lookup;
use dbx::data::model::FieldType::Text;
use dbx::data::Query as DBXQuery;
use dbx::data::WhereCondition;
use dbx::data::WhereExpr;
use dbx::{
    data::model::{DataModel, Table},
    Database,
};

// use tower_http::{ServeDir, ServerFile};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    rc::Rc,
    sync::{Arc, Mutex},
};

use axum::{
    body::Body,
    extract::{Extension, Path, Query},
    http::{header::CONTENT_TYPE, Request},
    response::Response,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use serde_json::Value;

use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::fmt::Write;

mod controllers;
mod error;
mod models;


use clap::Parser;

#[derive(Parser)]
struct CmdArgs {
    #[clap(short, default_value="0.0.0.0:8001")]
    port: String,
    root_path: String,
}


#[allow(dead_code)]
#[derive(Clone)]
struct State {
    last_call: usize,
}

fn make_person_model() -> DataModel {
    let mut model = DataModel::new("Person");
    let tab = Table::new("person")
        .field("id", true, Text(20))
        .field("name1", false, Text(100))
        .field("name2", false, Text(100))
        .field("name3", false, Text(100))
        .field("name4", false, Text(100));
    model = model
        .table(tab)
        .table(
            Table::new("email")
                .field("id", true, Text(20))
                .field(
                    "person",
                    false,
                    Lookup {
                        table: "person".into(),
                        as_field: "communications".into(),
                    },
                )
                .field("role", false, Text(100))
                .field("address", false, Text(100)),
        )
        .table(
            Table::new("phone")
                .field("id", true, Text(20))
                .field(
                    "person",
                    false,
                    Lookup {
                        table: "person".into(),
                        as_field: "communications".into(),
                    },
                )
                .field("role", false, Text(100))
                .field("number", false, Text(100)),
        );
    // let mut meta = Meta::new();
    // meta.define_relation(One, "person", "Communication.email", "email");
    // meta.define_relation(One, "person", "Communication.phone", "phone");
    // model.set_meta(meta);
    model.build();
    model
}

#[allow(dead_code)]
async fn get_handler(Path(objtype): Path<String>) -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!([format!(
        "hello, I am here {:?}.",
        objtype
    )]))
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct SampleData {
    value: usize,
}

#[allow(dead_code)]
async fn handler_out() -> (Extension<SampleData>, String) {
    (
        Extension(SampleData { value: 42 }),
        String::from("output handler was here"),
    )
}

#[allow(dead_code)]
async fn handler_in(e: Extension<Arc<Mutex<State>>>) -> String {
    if let Ok(mut x) = e.try_lock() {
        x.last_call += 1;
        format!("state last call: {}\n", x.last_call)
    } else {
        format!("race condition\n")
    }
}

async fn metadata( _e: Extension<Arc<Mutex<State>>>) -> Response<Body> {
    let mut r = String::new();
    r.push_str(
        r#"<?xml version="1.0" encoding="utf-8"?>
<edmx:Edmx xmlns:edmx="http://docs.oasis-open.org/odata/ns/edmx" Version="4.0">
  <edmx:DataServices>
    <Schema xmlns="http://docs.oasis-open.org/odata/ns/edm" Namespace="PJL">
      <EnumType Name="PersonGender" xmlns="http://docs.oasis-open.org/odata/ns/edm">
        <Member Name="Male" Value="0" />
        <Member Name="Female" Value="1" />
        <Member Name="Unknown" Value="2" />
      </EnumType>
      <ComplexType Name="Location" OpenType="true">
        <Property Name="Address" Type="Edm.String" Nullable="false" />
        <Property Name="City" Type="PJL.City" Nullable="false" />
      </ComplexType>
      <ComplexType Name="City">
        <Property Name="CountryRegion" Type="Edm.String" Nullable="false" />
        <Property Name="Name" Type="Edm.String" Nullable="false" />
        <Property Name="Region" Type="Edm.String" Nullable="false" />
      </ComplexType>
      <EntityType Name="Person" OpenType="true">
        <Key>
          <PropertyRef Name="UserName" />
        </Key>
        <Property Name="UserName" Type="Edm.String" Nullable="false">
          <Annotation Term="Org.OData.Core.V1.Permissions">
            <EnumMember>Org.OData.Core.V1.Permission/Read</EnumMember>
          </Annotation>
        </Property>
        <Property Name="FirstName" Type="Edm.String" Nullable="false" />
        <Property Name="LastName" Type="Edm.String" Nullable="false" />
        <Property Name="Emails" Type="Collection(Edm.String)" />
        <Property Name="AddressInfo" Type="Collection(PJL.Location)" />
        <Property Name="Gender" Type="PJL.PersonGender" />
        <Property Name="Concurrency" Type="Edm.Int64" Nullable="false">
          <Annotation Term="Org.OData.Core.V1.Computed" Bool="true" />
        </Property>
        <NavigationProperty Name="Friends" Type="Collection(PJL.Person)" />
      </EntityType>
      <EntityContainer Name="DefaultContainer">
        <EntitySet Name="People" EntityType="PJL.Person">
          <NavigationPropertyBinding Path="Friends" Target="People" />
        </EntitySet>
      </EntityContainer>
    </Schema>
  </edmx:DataServices>
</edmx:Edmx>"#,
    );

    let b = Body::from(r);
    let mut res = Response::new(b);
    res.headers_mut()
        .append(CONTENT_TYPE, "application/xml".parse().unwrap());
    debug!("response: {:#?}", res);
    res
}

#[allow(dead_code)]
async fn handle_entity_set(
    Path(entity): Path<String>,
    Query(q): Query<HashMap<String, String>>,
    // req: Request<Body>,
    _e: Extension<Arc<Mutex<State>>>,
) -> Response<Body> {
    // info!("request: {req:#?}");
    info!("Entity: {entity}");
    info!("Query: {q:?}");
    let mut r = String::new();
    let base = "http://localhost:3000/";

    let mut person = BTreeMap::<String, Value>::new();
    person.insert(
        "@odata.id".into(),
        format!("{base}People('russellwhyte')").into(),
    );
    person.insert("@odata.etag".into(), "W/\"08DC19CCEE96CE9B\"".into());
    person.insert(
        "@odata.editLink".into(),
        format!("{base}People('russellwhyte')").into(),
    );
    person.insert(
        "Friends@navigationLink".into(),
        format!("{base}People('russellwhyte')/Friends").into(),
    );
    person.insert("UserName".into(), "russellwhyte".into());
    person.insert("FirstName".into(), "Russell".into());
    person.insert("LastName".into(), "Whyte".into());
    person.insert("Gender".into(), "Male".into());
    person.insert("Concurrency".into(), "638413615146651300".into());
    person.insert(
        "Emails".into(),
        vec!["Russell@example.com", "Russell@contoso.com"].into(),
    );
    // {{
    //     "@odata.id": "{base}People('russellwhyte')",
    //     "@odata.etag": "W/\"08DC19CCEE96CE9B\"",
    //     "@odata.editLink": "{base}People('russellwhyte')",
    //     "Friends@navigationLink": "{base}People('russellwhyte')/Friends",
    //     "UserName": "russellwhyte",
    //     "FirstName": "Russell",
    //     "LastName": "Whyte",
    //     "Emails": [
    //         "Russell@example.com",
    //         "Russell@contoso.com"
    //     ],
    //     "AddressInfo": [
    //        {{
    //             "Address": "187 Suffolk Ln.",
    //             "City": {{
    //                 "CountryRegion": "United States",
    //                 "Name": "Boise",
    //                 "Region": "ID"
    //             }}
    //         }}
    //     ],
    //     "Gender": "Male",
    //     "Concurrency": 638413615146651300
    // }},

    let pers_str = serde_json::to_string_pretty(&person).unwrap();

    r.push_str(
        format!(
            r#"{{
    "@odata.context": "{base}$metadata#People",
    "@odata.nextLink": "{base}People?%24skiptoken=8",
    "value": [{pers_str},
        {{
            "@odata.id": "{base}People('scottketchum')",
            "@odata.etag": "W/\"08DC19CCEE96CE9B\"",
            "@odata.editLink": "{base}People('scottketchum')",
            "UserName": "scottketchum",
            "FirstName": "Scott",
            "LastName": "Ketchum",
            "Emails": [
                "Scott@example.com"
            ],
            "AddressInfo": [
                {{
                    "Address": "2817 Milton Dr.",
                    "City": {{
                        "CountryRegion": "United States",
                        "Name": "Albuquerque",
                        "Region": "NM"
                    }}
                }}
            ],
            "Gender": "Male",
            "Concurrency": 638413615146651300
        }},
        {{
            "@odata.id": "{base}People('ronaldmundy')",
            "@odata.etag": "W/\"08DC19CCEE96CE9B\"",
            "@odata.editLink": "{base}People('ronaldmundy')",
            "UserName": "ronaldmundy",
            "FirstName": "Ronald",
            "LastName": "Mundy",
            "Emails": [
                "Ronald@example.com",
                "Ronald@contoso.com"
            ],
            "AddressInfo": [],
            "Gender": "Male",
            "Concurrency": 638413615146651300
        }},
        {{
            "@odata.id": "{base}People('javieralfred')",
            "@odata.etag": "W/\"08DC19CCEE96CE9B\"",
            "@odata.editLink": "{base}People('javieralfred')",
            "UserName": "javieralfred",
            "FirstName": "Javier",
            "LastName": "Alfred",
            "Emails": [
                "Javier@example.com",
                "Javier@contoso.com"
            ],
            "AddressInfo": [
                {{
                    "Address": "89 Jefferson Way Suite 2",
                    "City": {{
                        "CountryRegion": "United States",
                        "Name": "Portland",
                        "Region": "WA"
                    }}
                }}
            ],
            "Gender": "Male",
            "Concurrency": 638413615146651300
        }},
        {{
            "@odata.id": "{base}People('willieashmore')",
            "@odata.etag": "W/\"08DC19CCEE96CE9B\"",
            "@odata.editLink": "{base}People('willieashmore')",
            "UserName": "willieashmore",
            "FirstName": "Willie",
            "LastName": "Ashmore",
            "Emails": [
                "Willie@example.com",
                "Willie@contoso.com"
            ],
            "AddressInfo": [],
            "Gender": "Male",
            "Concurrency": 638413615146651300
        }},
        {{
            "@odata.id": "{base}People('vincentcalabrese')",
            "@odata.etag": "W/\"08DC19CCEE96CE9B\"",
            "@odata.editLink": "{base}People('vincentcalabrese')",
            "UserName": "vincentcalabrese",
            "FirstName": "Vincent",
            "LastName": "Calabrese",
            "Emails": [
                "Vincent@example.com",
                "Vincent@contoso.com"
            ],
            "AddressInfo": [
                {{
                    "Address": "55 Grizzly Peak Rd.",
                    "City": {{
                        "CountryRegion": "United States",
                        "Name": "Butte",
                        "Region": "MT"
                    }}
                }}
            ],
            "Gender": "Male",
            "Concurrency": 638413615146651300
        }},
        {{
            "@odata.id": "{base}People('clydeguess')",
            "@odata.etag": "W/\"08DC19CCEE96CE9B\"",
            "@odata.editLink": "{base}People('clydeguess')",
            "UserName": "clydeguess",
            "FirstName": "Clyde",
            "LastName": "Guess",
            "Emails": [
                "Clyde@example.com"
            ],
            "AddressInfo": [],
            "Gender": "Male",
            "Concurrency": 638413615146651300
        }},
        {{
            "@odata.id": "{base}People('keithpinckney')",
            "@odata.etag": "W/\"08DC19CCEE96CE9B\"",
            "@odata.editLink": "{base}People('keithpinckney')",
            "UserName": "keithpinckney",
            "FirstName": "Keith",
            "LastName": "Pinckney",
            "Emails": [
                "Keith@example.com",
                "Keith@contoso.com"
            ],
            "AddressInfo": [],
            "Gender": "Male",
            "Concurrency": 638413615146651300
        }}
    ]
}}"#
        )
        .as_str(),
    );

    let b = Body::from(r);
    let mut res = Response::new(b);
    let hdrs = res.headers_mut();
    hdrs.append(
        CONTENT_TYPE,
        "application/json;odata.metadata=minimal".parse().unwrap(),
    );
    hdrs.append("odata-version", "4.0".parse().unwrap());
    res
}

async fn handle_dyn_entity(
    Path(entity): Path<String>,
    Query(q): Query<HashMap<String, String>>,
    // req: Request<Body>,
    _e: Extension<Arc<Mutex<State>>>,
) -> Response<Body> {
    // info!("request: {req:#?}");
    info!("Entity: {entity}");
    info!("Query: {q:?}");
    // let base = "http://localhost:3000/";

    // ---------------------
    let model = make_person_model();
    let db = Database::new();
    db.connect("person.sqlite".into());
    db.activate_structure(model);
    // ---------------------

    let b = Body::from(read_record(entity.as_str(), db));
    let mut res = Response::new(b);
    let hdrs = res.headers_mut();
    hdrs.append(
        CONTENT_TYPE,
        "application/json;odata.metadata=minimal".parse().unwrap(),
    );
    hdrs.append("odata-version", "4.0".parse().unwrap());
    res
}

fn read_record(id: &str, db: Database) -> String {
    let condition = WhereCondition::new().and(WhereExpr::Equals("id".into(), id.into()));
    let q = DBXQuery::new("person", vec!["*"], condition);

    let pp = db.select_rows(q);
    let buf = Rc::new(RefCell::new(String::new()));
    {
        let mut w = buf.borrow_mut();
        write!(w, "{{\"value\":").unwrap();
    }
    let fmt = edm::json::Format::new(buf.clone());
    fmt.convert(&pp.unwrap()).unwrap();
    {
        let mut w = buf.borrow_mut();
        write!(w, "}}").unwrap();
    }
    let r = buf.borrow().clone();
    r
}

async fn handler_meta(request: Request<Body>) -> Response<Body> {
    debug!("request: {:#?}", request);
    let mut r = String::new();
    r.push_str(
        "<edmx:Edmx xmlns:edmx=\"http://docs.oasis-open.org/odata/ns/edmx\" Version=\"4.0\">",
    );
    r.push_str("<edmx:DataServices>");
    r.push_str("<Schema xmlns=\"http://docs.oasis-open.org/odata/ns/edm\" Namespace=\"Microsoft.OData.SampleService.Models.TripPin\">");
    r.push_str("</Schema>");
    r.push_str("</edmx:DataServices>");
    r.push_str("</edmx:Edmx>");

    let b = Body::from(r);
    let mut res = Response::new(b);
    res.headers_mut()
        .append(CONTENT_TYPE, "application/xml".parse().unwrap());
    // debug!("response: {:#?}", res);
    res
}

#[allow(dead_code)]
async fn catch_all(request: Request<Body>) -> String {
    debug!("request: {:#?}", request);
    "...".into()
}

#[allow(dead_code)]
fn using_serve_dir(server_dir: &str) -> Router {
    // serve the file in the "assets" directory under `/assets`
    Router::new().nest_service("/", ServeDir::new(server_dir))
}

#[tokio::main]
async fn main() {
    let args = CmdArgs::parse();

    let env_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "simple_rest_server=debug".into());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(env_filter.clone()))
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("log filter is {}", env_filter);
    let extension = Arc::new(Mutex::new(State { last_call: 0 }));
    let svc = Router::new()
        .route("/", get(handler_meta))
        .route("/:id", get(handle_dyn_entity))
        .route("/$metadata", get(metadata))
        .layer(Extension(extension));

    let app = Router::new()
                .nest("/api/invoice", svc)
                .nest_service("/", ServeDir::new(args.root_path));

    tracing::debug!("listening on {}", args.port);
    let listener = tokio::net::TcpListener::bind(args.port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
