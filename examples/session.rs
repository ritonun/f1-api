use f1_api as f1;

#[tokio::main]
async fn main() {
    println!("Get session");

    let openf1 = f1::OpenF1Client::new();
    let sessions = match openf1.get_sessions().await {
        Ok(response) => response,
        Err(e) => panic!("Error getting session due to {}", e),
    };

    for s in sessions {
        println!("{:?}", s);
    }
}
