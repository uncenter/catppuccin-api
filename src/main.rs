use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use catppuccin::{Palette, PALETTE};
use catppuccin_api::models::{
    self,
    ports::{Category, Port, Showcase},
    shared::Collaborator,
};

use std::collections::HashMap;

use indoc::indoc;
use lazy_static::lazy_static;

pub const PORTS_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/ports.json"));
pub const USERSTYLES_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/userstyles.json"));

lazy_static! {
    pub static ref PORTS_DATA: models::ports::Root = serde_json::from_str(PORTS_JSON).unwrap();
    pub static ref USERSTYLES_DATA: models::userstyles::Root =
        serde_json::from_str(USERSTYLES_JSON).unwrap();
    pub static ref PORTS: HashMap<String, Vec<Port>> = PORTS_DATA
        .ports
        .iter()
        .map(|(identifier, port)| (identifier.clone(), port.clone()))
        .chain(
            USERSTYLES_DATA
                .userstyles
                .iter()
                .map(|u| (u.0.clone(), Into::<Port>::into(u.1.clone())))
        )
        .fold(HashMap::new(), |mut map, (identifier, port)| {
            map.entry(identifier.to_string())
                .or_insert_with(Vec::new)
                .push(port.clone());
            map
        });
    pub static ref COLLABORATORS: HashMap<String, Collaborator> = PORTS_DATA
        .collaborators
        .iter()
        .chain(USERSTYLES_DATA.collaborators.iter())
        .map(|collaborator| (
            collaborator
                .url
                .strip_prefix("https://github.com/")
                .expect("collaborator url should start with github href")
                .to_string(),
            collaborator.clone()
        ))
        .collect();
    pub static ref CATEGORIES: HashMap<String, Category> = PORTS_DATA
        .categories
        .iter()
        .map(|category| (category.key.clone(), category.clone()))
        .collect();
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/ports", get(list_ports))
        .route("/ports/:identifier", get(get_port))
        .route("/collaborators", get(list_collaborators))
        .route("/collaborators/:username", get(get_collaborator))
        .route("/categories", get(list_categories))
        .route("/categories/:key", get(get_category))
        .route("/showcases", get(list_showcases))
        .route("/palette", get(get_palette));

    println!("http://localhost:8080");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    indoc! {"
    Catppuccin API
    ==============

        ／l、
      （ﾟ､ ｡ ７
        l  ~ヽ
        じしf_,)ノ
    
    Unofficial, experimental API for interacting with Catppuccin port metadata.

    Routes
    ------

    # Includes ports and userstyles combined. Use the is-userstyle field to differentiate.
    # Returns an object, where the keys are the port identifiers and the values are arrays of ports matching the identifier (userstyles and ports might have duplicate identifiers, e.g. `mdbook`).
    /ports
        /ports/:identifier

    # Returns an object, where the keys are the usernames and the values are objects containing collaborator information.
    # Duplicate usernames between ports/userstyles are resolved by picking one to use, simply assuming they are identical.
    /collaborators
        /collaborators/:username

    # Returns an object of category keys and values.
    /categories
        /categories/:key

    # Returns an array of showcases.
    /showcases

    # Returns the color palette.
    /palette

    Source
    ------

    https://github.com/uncenter/catppuccin-api
    "}
    .to_string()
}

async fn list_ports() -> Json<HashMap<String, Vec<Port>>> {
    Json(PORTS.clone())
}

async fn get_port(Path(identifier): Path<String>) -> Result<Json<Vec<Port>>, impl IntoResponse> {
    match PORTS.get(&identifier) {
        Some(p) => Ok(Json(p.clone())),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("No port with identifier '{identifier}'"),
        )),
    }
}

async fn list_collaborators() -> Json<HashMap<String, Collaborator>> {
    Json(COLLABORATORS.clone())
}

async fn get_collaborator(
    Path(username): Path<String>,
) -> Result<Json<Collaborator>, impl IntoResponse> {
    match COLLABORATORS.get(&username) {
        Some(c) => Ok(Json(c.clone())),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("No collaborator with username '{username}'"),
        )),
    }
}

async fn list_categories() -> Json<HashMap<String, Category>> {
    Json(CATEGORIES.clone())
}

async fn get_category(Path(key): Path<String>) -> Result<Json<Category>, impl IntoResponse> {
    match CATEGORIES.get(&key) {
        Some(c) => Ok(Json(c.clone())),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("No category with key '{key}'"),
        )),
    }
}

async fn list_showcases() -> Json<Vec<Showcase>> {
    Json(PORTS_DATA.showcases.clone())
}

async fn get_palette() -> Json<Palette> {
    Json(PALETTE)
}
