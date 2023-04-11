use learn::run;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let flag = dotenv!("FLAG");
    println!("{:?}", flag);
    run().await

}