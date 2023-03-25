use actix_web::{web, HttpResponse, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::{
    dto::token::Token,
    env::{AppMode, AppName, ServiceName},
    gql_schema::schema::GqlSchema,
};

pub mod graphql;

async fn root(
    app_name: web::Data<AppName>,
    app_mode: web::Data<AppMode>,
    service_name: web::Data<ServiceName>,
) -> Result<HttpResponse> {
    let app_name = app_name.as_str();
    let app_mode = app_mode.as_str();
    let service_name = service_name.as_str();

    Ok(HttpResponse::Ok().body(format!(
        "{app_name} {service_name} is running in {app_mode}."
    )))
}

async fn graphql_v1(
    schema: web::Data<GqlSchema>,
    auth: Option<BearerAuth>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    if let Some(auth) = auth {
        req = req.data(Token(auth.token().to_owned()));
    }

    schema.execute(req).await.into()
}

async fn graphiql_v1(app_mode: web::Data<AppMode>) -> Result<HttpResponse> {
    let app_mode = app_mode.as_str();
    if app_mode == "DEBUG" {
        Ok(HttpResponse::Ok().body(GraphiQLSource::build().endpoint("/graphql/v1").finish()))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub struct CtxData {
    pub app_name: AppName,
    pub app_mode: AppMode,
    pub service_name: ServiceName,
    pub gql_schema: GqlSchema,
}

pub fn register(cfg: &mut web::ServiceConfig, data: CtxData) {
    // register / path
    cfg.service(
        web::scope("/")
            .app_data(web::Data::new(data.app_name))
            .app_data(web::Data::new(data.app_mode.to_owned()))
            .app_data(web::Data::new(data.service_name))
            .route("", web::get().to(root)),
    );

    // register /graphql path
    cfg.service(
        web::scope("/graphql").service(
            web::scope("/v1")
                .app_data(web::Data::new(data.gql_schema))
                .route("", web::post().to(graphql_v1)),
        ),
    );

    // register /graphiql
    cfg.service(
        web::scope("/graphiql")
            .app_data(web::Data::new(data.app_mode))
            .route("/v1", web::get().to(graphiql_v1)),
    );
}
