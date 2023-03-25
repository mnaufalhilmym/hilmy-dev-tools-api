use actix_web::{get, web, HttpResponse};

use crate::dto::AppData;

#[get("/")]
pub async fn root(data: web::Data<AppData>) -> HttpResponse {
    let site_link_url = data.site_link_url.as_str();

    HttpResponse::TemporaryRedirect()
        .insert_header(("location", site_link_url))
        .finish()
}
