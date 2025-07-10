use google_sheets4::{hyper_util::{client::legacy::Client, rt::TokioExecutor}, yup_oauth2::{self, ServiceAccountAuthenticator}, Sheets};
use hyper_rustls::HttpsConnectorBuilder;
use google_sheets4::hyper_util::client::legacy::connect::HttpConnector;

///
/// Obtains the `credentials.json` of the root project and authenticate it
pub async fn get_credentials() -> Result<Sheets<hyper_rustls::HttpsConnector<HttpConnector>>, Box<dyn std::error::Error>> {
    // `creadentials.json`: Provided by Google
    let credentials = yup_oauth2::read_service_account_key("credentials.json")
        .await
        .expect("Cannot read credential, an error occured.");

    let auth = ServiceAccountAuthenticator::builder(credentials)
        .build()
        .await
        .expect("There was an error, trying to build connection with authenticator");

    let connector = HttpsConnectorBuilder::new()
        .with_native_roots()?
        .https_or_http()
        .enable_http1()
        .build();

    let client = Client::builder(TokioExecutor::new()).build(connector);

    let hub = Sheets::new(client, auth);
    
    Ok(hub)
}