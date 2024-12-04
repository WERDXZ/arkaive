use reqwest::Client;

use crate::ARKAIVE;

async fn init_auth(client: &mut Client) -> Result<(), crate::Error> {
    // result: {"buttonText":"Check-in closed","buttonType":"danger","buttonDisabled":true,"buttonLogout":"","status":"closed"}

    client
        .get(ARKAIVE)
        .send()
        .await
        .map_err(crate::Error::ConnectionError)?;
    Ok(())
}

pub async fn auth(
    username: &str,
    password: &str,
    client: &mut Client,
) -> Result<(), crate::Error> {
    init_auth(client).await?;
    let params = [("form[email]", username), ("form[password]", password)];
    let status = client
        .post(format!("{}/login_check", ARKAIVE))
        .form(&params)
        .send()
        .await
        .map_err(crate::Error::ConnectionError)?
        .status();

    if status != reqwest::StatusCode::FOUND {
        return Err(crate::Error::UnexpectedStatusCode(status));
    }
    Ok(())
}
