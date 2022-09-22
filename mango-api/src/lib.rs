use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug,Deserialize)]
#[allow(dead_code)]
pub struct User{
    profile_image_url:Option<String>,
    profile_name:Option<String>,
    trader_category:Option<String>,
    wallet_pk:Option<String>,
}


pub fn fetch_price() -> Value {
    let url = "https://all-market-stats-api.onrender.com/markets/".to_string();
    let client = Client::new();
    let response = client.get(url).send().unwrap().text().unwrap();

    let p: Value = serde_json::from_str(response.as_str()).unwrap();
    p
}

//https://mango-transaction-log.herokuapp.com/v3/user-data/profile-details
// 5SKRvHAuiARiMYJtkXYuWy1kVTEJWbEAhnVN3MHxKzan
//pub async fn fetch_userdata(wallet_pk: String) -> User {
//    let url =
//        "https://mango-transaction-log.herokuapp.com/v3/user-data/profile-details".to_string();
//    let params = [("wallet-pk", wallet_pk)];
//    let url = reqwest::Url::parse_with_params(&url, params.iter()).unwrap();
//    let client = Client::new();
//    let response = client
//        .get(url)
//        .send()
//        .await
//        .unwrap()
//        .text()
//        .await
//        .unwrap_or_else(|_| json!({"wallet_pk":"null"}).to_string()); //handling the theoretical possibility of a non existent wallet
//    let p: User = serde_json::from_str(response.as_str()).unwrap();
//    p
//}

//pub async fn fetch_leaderboard() -> u16 {
//    let url = "https://mango-transaction-log.herokuapp.com/v3/stats/pnl-leaderboard".to_string();
//    let c = Client::new();
//    let res = c.get(url).send().await.unwrap();
//    println!("{:?}", res);
//    res.status().as_u16()
//}

#[cfg(test)]
mod tests {
    use super::*;
 //   #[tokio::test]
 //   async fn test_stats() {
 //       let status = fetch_leaderboard().await;
 //       assert_eq!(status, 200);
 //   }

    #[test]
    fn test_prices() {
        let x = fetch_price();
        for i in x.as_array().unwrap(){
            println!("{:#?}", i["baseSymbol"]);
        }
        
        //assert_eq!(status, 200);
    }

 //   #[tokio::test]
 //   async fn test_userdata() {
 //       let wallet_pk = "5SKRvHAuiARiMYJtkXYuWy1kVTEJWbEAhnVN3MHxKzan".to_string();
 //       let x = fetch_userdata(wallet_pk).await;
 //       println!("{:#?}", x);
 //       //assert_eq!(status, 200);
 //   }
}
