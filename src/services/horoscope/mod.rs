use std::collections::HashMap;
use std::error::Error;

use html_escape::decode_html_entities;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use unidecode::unidecode;

/// Récupère dynamiquement tous les horoscopes depuis Le Gorafi
fn fetch_horoscopes() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; LegorafiScraper/0.1)")
        .build()?;

    let category_url = "https://www.legorafi.fr/category/horoscope-2/";

    let cat_html = client.get(category_url).send()?.text()?;
    let cat_doc = Html::parse_document(&cat_html);

    let mut article_url: Option<String> = None;

    let a_sel = Selector::parse("a").unwrap();
    for a in cat_doc.select(&a_sel) {
        if let Some(href) = a.value().attr("href") {
            if href.contains("/horoscope-du-") {
                article_url = Some(href.to_string());
                break;
            }
        }
    }

    let article_url = match article_url {
        Some(url) => reqwest::Url::parse(&url)
            .or_else(|_| reqwest::Url::parse(category_url)?.join(&url))?
            .to_string(),
        None => return Err("Aucun lien d’horoscope trouvé".into()),
    };

    // Télécharger la page de l’article
    let art_html = client.get(&article_url).send()?.text()?;
    let art_doc = Html::parse_document(&art_html);

    // Extraire les paragraphes du contenu principal
    let p_sel = Selector::parse("div#mvp-content-main p").unwrap();
    let mut horoscopes = HashMap::new();

    for p in art_doc.select(&p_sel) {
        let text = p.text().collect::<Vec<_>>().join(" ");
        let decoded = decode_html_entities(&text);

        // Format attendu : "Signe : message"
        if let Some((sign, msg)) = decoded.split_once(':') {
            let sign_upper = sign.trim().to_uppercase();
            let normalized = unidecode(&sign_upper);
            let message = msg.trim().to_string();
            horoscopes.insert(normalized, message);
        }
    }

    Ok(horoscopes)
}

/// Renvoie l’horoscope correspondant au signe astrologique
pub fn main_horoscope(sign: &str) -> Result<String, Box<dyn Error>> {
    let horoscopes = fetch_horoscopes()?;
    let key = sign.to_uppercase();

    if let Some(txt) = horoscopes.get(&key) {
        Ok(txt.clone())
    } else {
        Ok("Signe inconnu ou non trouvé dans l’horoscope du jour.".to_string())
    }
}
