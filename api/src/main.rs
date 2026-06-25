use poem::{ Route, Server, get, handler, listener::TcpListener, post, web::{ Json, Path } };

use crate::{ request_inputs::{ CreateWebsiteInput }, request_outputs::CreateWebsiteOutput };
use store::Store;

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("health: {}", website_id)
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let s = Store {};
    let id = s.create_website();
    let response = CreateWebsiteOutput {
        id: id,
    };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3001")).run(app).await
}
