use f1_api as f1;

#[tokio::main]
async fn main() {
    // Create a client to make rewest to openf1.org
    let openf1 = f1::OpenF1Client::new();

    // get all sessions info
    let sessions = match openf1.get_sessions().await {
        Ok(response) => response,
        Err(e) => panic!("Error getting session due to {}", e),
    };
}
