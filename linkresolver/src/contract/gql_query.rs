use cynic::QueryBuilder;

mod schema {
    cynic::use_schema!("./schema/gateway.schema.graphql");
}

#[derive(cynic::QueryVariables)]
pub struct VisitLinkReq {
    pub short_url: String,
}

#[derive(cynic::QueryFragment)]
#[cynic(schema_path = "./schema/gateway.schema.graphql")]
pub struct VisitLinkRes {
    pub long_url: String,
}

#[derive(cynic::QueryFragment)]
#[cynic(
    schema_path = "./schema/gateway.schema.graphql",
    graphql_type = "QueryRootV1",
    variables = "VisitLinkReq"
)]
pub struct VisitLinkQuery {
    #[arguments(shortUrl: $short_url)]
    pub visit_link: VisitLinkRes,
}

pub fn query_builder(short_url: String) -> cynic::Operation<VisitLinkQuery, VisitLinkReq> {
    VisitLinkQuery::build(VisitLinkReq { short_url })
}
