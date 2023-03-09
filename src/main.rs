use reqwest::Error;
use std::collections::HashMap;
use std::{thread,time};

async fn login(client: &reqwest::Client, data: &HashMap<&str, &str>) -> Result<(), Error> {

    let _ = client.post("http://phc.prontonetworks.com/cgi-bin/authlogin?URI=http://www.msftconnecttest.com/redirect")
    .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36")
    .form(&data)
    .send()
    .await?;

    println!("Connecting to VIT WIFI");
    Ok(())
}



#[tokio::main]
async fn main() {


    let mut data = HashMap::new();

    data.insert("userId", "");
    data.insert("password","");
    data.insert("serviceName", "ProntoAuthentication");
    data.insert("Submit22", "Login");

    let client = reqwest::Client::new();

    loop {
        let check = client.get("https://www.google.com").send().await;

        match check {
            Ok(_) => {
                println!("Active internet");
                thread::sleep(time::Duration::from_millis(5000));
                continue;
            }

            Err(_) => {
                let res = login(&client, &data).await;

                if let Err(_) = res {
                    println!("Some error");
                    continue;
                }
                else{
                    println!("Connected");
                }
            }
        }

   
        
    }
}
