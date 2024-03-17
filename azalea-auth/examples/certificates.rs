use std::path::PathBuf;

use azalea_auth::{account::Account, MicrosoftAccount};

#[tokio::main]
async fn main() {
    env_logger::init();

    let cache_file = PathBuf::from("example_cache.json");

    let auth_result = MicrosoftAccount::new(
        "example@example.com",
        azalea_auth::MicrosoftAuthOpts {
            cache_file: Some(cache_file),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let certs = auth_result.fetch_certificates()
        .await
        .unwrap();

    println!("{certs:?}");
}
