pub mod apprepo {
    tonic::include_proto!("apprepo");

    pub use apprepo_service_client::ApprepoServiceClient;
    pub use apprepo_service_server::ApprepoService;
    pub use apprepo_service_server::ApprepoServiceServer;
}
