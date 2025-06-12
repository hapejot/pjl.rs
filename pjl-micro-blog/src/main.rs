use axum::extract::{Form, Path};
use axum::response::Redirect;
use axum::{extract::State, response::Html, routing::get, Router};
use clap::Parser;
use handlebars::DirectorySourceOptions;
use serde::{Deserialize, Serialize};
use tower_http::services::{ServeDir, ServeFile};
use std::sync::Arc;
use tokio_postgres::NoTls;
use tower_http::trace::{self, TraceLayer};
use tracing::info;
use uuid::Uuid;
use axum::response::IntoResponse;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Clone)]
struct AppState {
    templates: handlebars::Handlebars<'static>,
    db: Arc<pg::DB>,
}

impl AppState {
    fn new(db: pg::DB) -> Self {
        let mut hb = handlebars::Handlebars::new();
        hb.register_templates_directory("templates", DirectorySourceOptions::default())
            .unwrap();
        hb.set_dev_mode(true);
        Self {
            templates: hb,
            db: Arc::new(db),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Post {
    id: String,
    title: String,
    content: String,
}
impl Post {
    fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: "".into(),
            content: "".into(),
        }
    }
}
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "nuc")]
    host: String,
    #[arg(long, default_value = "postgres")]
    user: String,
    #[arg(long, default_value = "Kennwort01")]
    password: String,
    #[arg(long, default_value = "blog")]
    dbname: String,
}
use std::collections::HashMap;

use async_trait::async_trait;
use axum_login::{login_required, AuthUser, AuthnBackend, UserId};

#[derive(Debug, Clone)]
struct User {
    id: i64,
    pw_hash: Vec<u8>,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.pw_hash
    }
}

#[derive(Clone, Default)]
struct Backend {
    users: HashMap<i64, User>,
}

#[derive(Clone)]
struct Credentials {
    user_id: i64,
}

// impl AuthnBackend for Backend {
//     #[doc = " Authenticating user type."]
// type User = User;

//     #[doc = " Credential type used for authentication."]
// type Credentials = Credentials;

//     #[doc = " An error which can occur during authentication and authorization."]
// type Error = std::fmt::Error;

// //     #[doc = " Authenticates the given credentials with the backend."]
// #[must_use]
// #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
// fn authenticate<'life0,'async_trait>(&'life0 self,creds:Self::Credentials,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Option<Self::User> ,Self::Error> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
//         todo!()
//     }

//     #[doc = " Gets the user by provided ID from the backend."]
// #[must_use]
// #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
// fn get_user<'life0,'life1,'async_trait>(&'life0 self,user_id: &'life1 UserId<Self>) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Option<Self::User> ,Self::Error> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
//         todo!()
//     }
// }

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        Credentials { user_id }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.users.get(&user_id).cloned())
    }

    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.users.get(user_id).cloned())
    }
}
#[tokio::main]
async fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();
    info!("Starting up");
    // Parse command line arguments
    let args = Args::parse();

    // Connect to the database
    let (client, connection) = tokio_postgres::connect(
        &format!(
            "host={} user={} password={} dbname={}",
            args.host, args.user, args.password, args.dbname
        ),
        NoTls,
    )
    .await
    .expect("Failed to connect to database");

    // Spawn the connection in a background task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Create the database interface
    let db = pg::DB::new(client);

    // Create the application state
    let state = AppState::new(db);
    let serve_dir = ServeDir::new("static").not_found_service(ServeFile::new("static/index.html"));

    // Build the application with a route
    let app = Router::new()
        .route("/", get(show_posts).post(create_post))
        .route("/new", get(new_post))
        .route("/edit/{id}", get(edit_post))
        .route_layer(login_required!(Backend, login_url = "/login"))
                .with_state(state)
        .layer(TraceLayer::new_for_http())
        .fallback_service(serve_dir);

    // Run the application
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8017").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Serialize)]
struct ListResult<T>
where
    T: Serialize,
{
    values: Vec<T>,
}

async fn show_posts(State(state): State<AppState>) -> Html<String> {
    let posts = state.db.select_all().await.unwrap();
    let r = ListResult {
        values: posts.clone(),
    };
    let html = state.templates.render("index", &r).unwrap();
    Html(html)
}

async fn new_post(State(state): State<AppState>) -> Html<String> {
    let p = Post::new();
    let html = state.templates.render("edit", &p).unwrap();
    Html(html)
}

async fn edit_post(State(state): State<AppState>, Path(id): Path<String>) -> Html<String> {
    let post = state.db.get(&id).await.unwrap().unwrap();
    let html = state.templates.render("edit", &post).unwrap();
    Html(html)
}

async fn create_post(State(state): State<AppState>, Form(post): Form<Post>) -> Redirect {
    state.db.create(&post).await.unwrap();
    Redirect::to("/")
}


mod pg {
    use super::Post;
    use tokio_postgres::{Client, Error};

    pub struct DB {
        client: Client,
    }

    impl DB {
        pub fn new(client: Client) -> Self {
            Self { client }
        }

        pub async fn select_all(&self) -> Result<Vec<Post>, Error> {
            let client = &self.client;
            let query = "SELECT id, title, content FROM posts";
            let rows = client.query(query, &[]).await?;
            let mut posts = Vec::new();
            for row in rows {
                posts.push(Post {
                    id: row.get(0),
                    title: row.get(1),
                    content: row.get(2),
                });
            }
            Ok(posts)
        }

        pub async fn create(&self, post: &Post) -> Result<(), Error> {
            let client = &self.client;
            let query = "INSERT INTO posts (id, title, content) VALUES ($1, $2, $3)";
            match client
                .execute(query, &[&post.id, &post.title, &post.content])
                .await
            {
                Ok(_) => Ok(()),
                Err(_) => {
                    let query = "UPDATE posts SET title=$2, content=$3 WHERE id=$1";
                    client
                        .execute(query, &[&post.id, &post.title, &post.content])
                        .await?;
                    Ok(())
                }
            }
        }

        pub async fn get(&self, id: &str) -> Result<Option<Post>, Error> {
            let client = &self.client;
            let query = "SELECT id, title, content FROM posts WHERE id = $1";
            if let Some(row) = client.query_opt(query, &[&id]).await? {
                Ok(Some(Post {
                    id: row.get(0),
                    title: row.get(1),
                    content: row.get(2),
                }))
            } else {
                Ok(None)
            }
        }

        pub async fn update(&self, id: &str, title: &str, content: &str) -> Result<(), Error> {
            let client = &self.client;

            let query = "UPDATE posts SET title = $2, content = $3 WHERE id = $1";
            client.execute(query, &[&id, &title, &content]).await?;
            Ok(())
        }

        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            let client = &self.client;
            let query = "DELETE FROM posts WHERE id = $1";
            client.execute(query, &[&id]).await?;
            Ok(())
        }
    }
}
