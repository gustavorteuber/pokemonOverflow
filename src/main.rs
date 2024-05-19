use reqwest::Client;
use serde::Deserialize;
use std::io::{self, Write};
use tokio::time::{sleep, Duration};
use rand::Rng;
use thirtyfour::prelude::*;
use thirtyfour::ChromeCapabilities;
use urlencoding::encode;

#[derive(Deserialize)]
struct Pokemon {
    id: u32,
    name: String,
    sprites: Sprites,
    stats: Vec<Stat>,
}

#[derive(Deserialize)]
struct Sprites {
    front_default: String,
}

#[derive(Deserialize)]
struct Stat {
    base_stat: u32,
    stat: StatDetail,
}

#[derive(Deserialize)]
struct StatDetail {
    name: String,
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    print!("Digite o número de telefone (com o código do país e DDD): ");
    io::stdout().flush().unwrap();
    let mut whatsapp_number = String::new();
    io::stdin().read_line(&mut whatsapp_number).unwrap();
    let whatsapp_number = whatsapp_number.trim().to_string();

    let client = Client::new();

    let mut caps = ChromeCapabilities::new();
    caps.add_chrome_arg("--disable-gpu")?;
    caps.add_chrome_arg("--no-sandbox")?;
    caps.add_chrome_arg("--disable-dev-shm-usage")?;
    let driver = WebDriver::new("http://localhost:9515", &caps).await?;

    loop {
        let pokemon_id = rand::thread_rng().gen_range(1..=151); 
        let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_id);
        match client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Pokemon>().await {
                        Ok(pokemon) => {
                            let message = format!(
                                "🌟 Pokémon: {} 🌟\nID: {}\nStats:\n{}\n\n🚀 Saiba mais sobre o overflow do @gustavorteuber🚀\n\n{}",
                                pokemon.name,
                                pokemon.id,
                                pokemon.stats.iter().map(|s| format!("{}: {}", s.stat.name, s.base_stat)).collect::<Vec<_>>().join("\n"),
                                pokemon.sprites.front_default,
                            );
                            let whatsapp_url = format!("https://wa.me/{}?text={}", whatsapp_number, encode(&message));
                            
                            driver.get(whatsapp_url).await?;
                            sleep(Duration::from_secs(5)).await;
                            
                            let message_box = driver.find_element(By::Css("div[contenteditable='true']")).await?;
                            message_box.send_keys("\n").await?; 
                            message_box.send_keys(&message).await?;
                            message_box.send_keys("\n").await?; 
                        }
                        Err(err) => eprintln!("Failed to parse Pokémon data: {}", err),
                    }
                } else {
                    eprintln!("Failed to fetch Pokémon data: HTTP {}", response.status());
                }
            }
            Err(err) => eprintln!("Failed to fetch Pokémon data: {}", err),
        }

        sleep(Duration::from_secs(10)).await; 
    }
}
