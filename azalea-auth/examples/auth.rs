use std::path::PathBuf;

use azalea_auth::MicrosoftAccount;

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
    println!("{auth_result:?}");
}
