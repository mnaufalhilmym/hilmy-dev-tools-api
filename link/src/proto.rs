pub mod link {
    tonic::include_proto!("link");

    pub use link_service_client::LinkServiceClient;
    pub use link_service_server::LinkService;
    pub use link_service_server::LinkServiceServer;
}
