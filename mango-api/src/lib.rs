use reqwest::Client;
use serde_json::Value;

pub async fn fetch_price() -> Value {
    let url = "https://all-market-stats-api.onrender.com/markets/".to_string();
    let client = Client::new();
    let response = client.get(url).send().await.unwrap().text().await.unwrap();

    let p: Value = serde_json::from_str(response.as_str()).unwrap();
    p
}

pub async fn fetch_leaderboard() -> u16 {
    let url = "https://mango-transaction-log.herokuapp.com/v3/stats/pnl-leaderboard".to_string();
    let c = Client::new();
    let res = c.get(url).send().await.unwrap();
    println!("{:?}", res);
    res.status().as_u16()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_stats() {
        let status = fetch_leaderboard().await;
        assert_eq!(status, 200);
    }
    #[tokio::test]
    async fn test_prices() {
        let x = fetch_price().await;
        println!("{:#?}", x);
        //assert_eq!(status, 200);
    }
}

