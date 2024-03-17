//! Authenticate with Microsoft and get a Minecraft profile, but don't cache and
//! use our own code to display the link code.
//!
//! If you still want it to cache, look at the code in [`azalea_auth::auth`] and
//! see how that does it.

use std::error::Error;

use azalea_auth::ProfileResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let profile = auth().await?;
    println!("Logged in as {}", profile.name);

    Ok(())
}

async fn auth() -> Result<ProfileResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let res = azalea_auth::get_ms_link_code(&client).await?;
    println!(
        "Go to {} and enter the code {}",
        res.verification_uri, res.user_code
    );
    let msa = azalea_auth::get_ms_auth_token(&client, res).await?;
    let xbl_auth = azalea_auth::auth_with_xbox_live(&client, &msa.data.access_token).await?;

    let xsts_token = azalea_auth::obtain_xsts_for_minecraft(
        &client,
        &xbl_auth
            .get()
            .expect("Xbox Live auth token shouldn't have expired yet")
            .token,
    )
    .await?;

    // Minecraft auth
    let mca = azalea_auth::auth_with_minecraft(&client, &xbl_auth.data.user_hash, &xsts_token).await?;

    let minecraft_access_token: String = mca
        .get()
        .expect("Minecraft auth shouldn't have expired yet")
        .access_token
        .to_string();
    Ok(azalea_auth::get_profile(&client, &minecraft_access_token).await?)
}
