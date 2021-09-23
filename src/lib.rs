pub mod servers;

#[tokio::main]
#[test]
async fn test() {
    let data = servers::get_battlelog_keeper_data("ced04fb4-22e6-4547-9b09-14fb5a52d132".to_string()).await.unwrap();
    println!("{:#?}", data)
}

