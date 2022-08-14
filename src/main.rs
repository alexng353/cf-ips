use tokio;

#[tokio::main]
async fn main() {
    let res = reqwest::get("https://www.cloudflare.com/ips-v4").await;
    let ips = res.unwrap().text().await.unwrap();
    for ip in ips.split('\n') {
        if ip.len() > 0 {
            println!("set_real_ip_from {};", ip);
        }
    }

    let res = reqwest::get("https://www.cloudflare.com/ips-v6").await;
    let ips = res.unwrap().text().await.unwrap();
    for ip in ips.split('\n') {
        if ip.len() > 0 {
            println!("set_real_ip_from {};", ip);
        }
    }
}
