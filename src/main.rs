use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use catppuccin_api::models::{
    self,
    ports::{Category, Port, Showcase},
    shared::{Collaborator, SingleOrMultiple},
};
use indoc::indoc;
use lazy_static::lazy_static;
use serde::Serialize;

pub const PORTS_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/ports.json"));
pub const USERSTYLES_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/userstyles.json"));

lazy_static! {
    pub static ref PORTS_DATA: models::ports::Root = serde_json::from_str(PORTS_JSON).unwrap();
    pub static ref USERSTYLES_DATA: models::userstyles::Root =
        serde_json::from_str(USERSTYLES_JSON).unwrap();
    pub static ref PORTS: Vec<ModifiedPort> = PORTS_DATA
        .ports
        .iter()
        .map(|(identifier, port)| (identifier.clone(), port.clone()))
        .chain(
            USERSTYLES_DATA
                .userstyles
                .iter()
                .map(|(identifier, userstyle)| {
                    let port: Port = userstyle.clone().into();
                    (identifier.clone(), port)
                })
        )
        .map(|(identifier, port)| {
            let categories = port
                .categories
                .iter()
                .map(|c| {
                    PORTS_DATA
                        .categories
                        .iter()
                        .find(|category| category.key == *c)
                        .unwrap()
                })
                .cloned()
                .collect();

            ModifiedPort {
                identifier,
                port,
                categories,
            }
        })
        .collect::<Vec<_>>();
    pub static ref COLLABORATORS: Vec<Collaborator> = PORTS_DATA
        .collaborators
        .iter()
        .chain(USERSTYLES_DATA.collaborators.iter())
        .cloned()
        .collect::<Vec<_>>();
}

#[derive(Serialize, Clone, Debug)]
pub struct ModifiedPort {
    #[serde(flatten)]
    f1: T1,
    #[serde(flatten)]
    port: Port,
    categories: Vec<Category>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Identifier {
    identifier: String,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/ports", get(list_ports))
        .route("/port/:identifier", get(get_port))
        .route("/collaborators", get(list_collaborators))
        .route("/collaborator/:username", get(get_collaborator))
        .route("/categories", get(list_categories))
        .route("/category/:key", get(get_category))
        .route("/showcases", get(list_showcases));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    indoc! {"
      --------------
    < Catppuccin API >
      --------------
      \\
        \\
         ／l、
       （ﾟ､ ｡ ７
         l  ~ヽ
         じしf_,)ノ

    === Routes ===
    - `/ports`
        - `/port/:identifier`
    - `/collaborators`
        - `/collaborator/:username`
    - `/categories`
        - `/category/:key`
    - `/showcases`
    "}
    .to_string()
}

async fn list_ports() -> Json<Vec<ModifiedPort>> {
    Json(PORTS.iter().cloned().collect())
}

async fn get_port(
    Path(identifier): Path<String>,
) -> Result<Json<SingleOrMultiple<ModifiedPort>>, impl IntoResponse> {
    let matches: Vec<_> = PORTS
        .iter()
        .filter(|port| port.identifier.to_lowercase() == identifier)
        .cloned()
        .collect();
    match matches.len() {
        0 => Err((
            StatusCode::NOT_FOUND,
            format!("No port with identifier {}", identifier),
        )),
        1 => Ok(Json(SingleOrMultiple::<ModifiedPort>::Single(
            matches.get(0).unwrap().clone(),
        ))),
        _ => Ok(Json(SingleOrMultiple::<ModifiedPort>::Multiple(
            matches,
        ))),
    }
}

async fn list_collaborators() -> Json<Vec<Collaborator>> {
    Json(COLLABORATORS.iter().cloned().collect())
}

async fn get_collaborator(
    Path(username): Path<String>,
) -> Result<Json<Collaborator>, impl IntoResponse> {
    match COLLABORATORS.iter().find(|c| {
        c.url
            .strip_prefix("https://github.com/")
            .expect("collaborator url should start with github href")
            == username
    }) {
        Some(p) => Ok(Json(p.clone())),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("No collaborator with username {}", username),
        )),
    }
}

async fn list_categories() -> Json<Vec<Category>> {
    Json(PORTS_DATA.categories.iter().cloned().collect())
}

async fn get_category(Path(key): Path<String>) -> Result<Json<Category>, impl IntoResponse> {
    match PORTS_DATA
        .categories
        .iter()
        .find(|category| category.key == key)
    {
        Some(c) => Ok(Json(c.clone())),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("No category with key {}", key),
        )),
    }
}

async fn list_showcases() -> Json<Vec<Showcase>> {
    Json(PORTS_DATA.showcases.iter().cloned().collect())
}
