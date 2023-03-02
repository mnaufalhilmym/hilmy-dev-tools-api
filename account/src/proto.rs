pub mod account {
    tonic::include_proto!("account");

    pub use account_service_client::AccountServiceClient;
    pub use account_service_server::AccountService;
    pub use account_service_server::AccountServiceServer;
}
