use actix_web::{web, HttpResponse, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use tools_lib_db::pg::connection::DbPool;

use crate::{
    dto::token::Token,
    env::{self, AppMode},
};

use self::graphql::v1::{MutationRootV1, QueryRootV1};

mod graphql;

async fn root() -> Result<HttpResponse> {
    let app_name = &env::Env::app_name();
    let app_mode = &env::Env::app_mode();
    let service_name = &env::Env::service_name();

    Ok(HttpResponse::Ok().body(format!(
        "{app_name} {service_name} is running in {app_mode}."
    )))
}

async fn graphql_v1(
    schema: web::Data<Schema<QueryRootV1, MutationRootV1, EmptySubscription>>,
    auth: Option<BearerAuth>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    if let Some(auth) = auth {
        req = req.data(Token(auth.token().to_owned()));
    }

    schema.execute(req).await.into()
}

async fn graphiql_v1() -> Result<HttpResponse> {
    let app_mode = &env::Env::app_mode();
    if app_mode == "DEBUG" {
        Ok(HttpResponse::Ok().body(GraphiQLSource::build().endpoint("/graphql/v1").finish()))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub struct CtxData {
    pub app_mode: AppMode,
    pub db_pool: DbPool,
}

pub fn register(cfg: &mut web::ServiceConfig, data: CtxData) {
    // register / path
    cfg.route("/", web::get().to(root));

    // register /graphql path
    cfg.service(
        web::scope("/graphql").service(
            web::scope("/v1")
                .app_data(web::Data::new(
                    Schema::build(
                        QueryRootV1::default(),
                        MutationRootV1::default(),
                        EmptySubscription,
                    )
                    .data(data.app_mode)
                    .data(data.db_pool)
                    .finish(),
                ))
                .route("", web::post().to(graphql_v1)),
        ),
    );

    // register /graphiql
    cfg.service(web::scope("/graphiql").route("/v1", web::get().to(graphiql_v1)));
}
