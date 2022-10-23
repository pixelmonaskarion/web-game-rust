use tokio::runtime::Runtime;
use std::collections::HashMap;

fn main() {
    let rt  = Runtime::new().unwrap();
    let connection = Connection::new("https://httpbin.org".to_string());
    let result = rt.block_on(connection.get_json("/ip".to_string()));
    let mut json = HashMap::new();
    match result {
        Ok(e) => json = e,
        Err(e) => println!("{}", e),
    }
    for (key, value) in json.iter() {
        println!("{}: {}", key, value);
    }
}
