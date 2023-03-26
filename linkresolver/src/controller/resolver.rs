use actix_web::{get, web, HttpResponse};
use cynic::http::ReqwestExt;

use crate::{contract, dto::AppData, static_file};

#[get("/{short_url}")]
pub async fn resolve_link(data: web::Data<AppData>, path: web::Path<String>) -> HttpResponse {
    let app_mode = data.app_mode.as_str();
    let gateway_service = data.gql_addrs.as_str();

    let short_url = path.into_inner();

    if short_url == "favicon.ico" {
        return HttpResponse::NotFound().finish();
    }

    let query = contract::gql_query::query_builder(short_url);

    let res = match reqwest::Client::new()
        .post(gateway_service)
        .run_graphql(query)
        .await
    {
        Ok(res) => res,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    if let Some(data) = res.data {
        return HttpResponse::TemporaryRedirect()
            .insert_header(("location", data.visit_link.long_url))
            .finish();
    }

    let mut http_res = &mut HttpResponse::NotFound();
    if let Some(errors) = res.errors {
        if app_mode == "DEBUG" {
            let err = errors
                .iter()
                .map(|error| {
                    format!(
                        "message: {}, locations: {:#?}, path: {:#?}, extensions: {:#?}",
                        error.message, error.locations, error.path, error.extensions
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");
            http_res = http_res.insert_header(("LINK-ERRORS", err));
        }
    }
    http_res.body(static_file::NOT_FOUND)
}
