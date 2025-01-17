use serde::Deserialize;
use std::env;

// Structure pour désérialiser la réponse JSON
#[derive(Deserialize, Debug)]

struct FMPResponse {
    symbol: String,
    #[serde(rename = "companyName")]
    company_name: String,
    price: f32,
    // description: Option<String>, // Description peut être absente
}

// Fonction de commande Tauri
#[tauri::command]
fn greet(_name: &str) -> Result<String, String> {
    let api_key = get_api_key();
    let symbol = "AAPL"; // Symbole pour le CAC 40
    let api_url = format!(
        "https://financialmodelingprep.com/api/v3/profile/{}?apikey={}",
        symbol, api_key
    );

    // On appelle une fonction asynchrone dans un contexte synchrone
    let response = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(fetch_symbol_data(&api_url));
    println!("Réponse brute : {:?}", response);

    match response {
        Ok(resp) => {
            // let description = resp
            //     .description
            //     .unwrap_or("Description non disponible".to_string());
            println!(
                "Données récupérées pour {}:\nCompany Name: {}, \nActual Price: {}",
                resp.symbol, resp.company_name, resp.price
            );
            Ok(format!(
                "Hello, {}! \nDonnées récupérées pour {}:\nCompany Name: {}, \nActual Price: {}",
                _name, resp.symbol, resp.company_name, resp.price
            ))
        }
        Err(e) => {
            println!("Erreur lors de l'appel API : {}", e);
            Err(format!("Une erreur est survenue : {}", e))
        }
    }
}

// Fonction pour appeler l'API et désérialiser la réponse
async fn fetch_symbol_data(api_url: &str) -> Result<FMPResponse, Box<dyn std::error::Error>> {
    println!("URL envoyée : {}", api_url);

    // Désérialiser en Vec<FMPResponse> car la réponse est un tableau JSON
    let response = reqwest::get(api_url).await?.json::<Vec<FMPResponse>>().await?;

    // Vérifier si le tableau n'est pas vide et retourner le premier élément
    Ok(response.into_iter().next().ok_or_else(|| {
        // Retourner une erreur générique avec un message d'erreur personnalisé
        Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Aucune donnée trouvée pour ce symbole"))
    })?)
}

// Charger la clé API depuis .env
pub fn get_api_key() -> String {
    dotenv::dotenv().ok(); // Charge le fichier .env
    env::var("FMP_API_KEY").expect("La clé API n'est pas définie")
}

// Point d'entrée de l'application Tauri
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
