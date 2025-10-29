use std::collections::HashMap;
use std::error::Error;

/// Renvoie l'horoscope correspondant au signe astrologique
pub fn main_horoscope(sign: &str) -> Result<String, Box<dyn Error>> {
    let mut horoscopes: HashMap<&str, &str> = HashMap::new();

    horoscopes.insert("BELIER", "Vous déciderez vous aussi de braquer le musée le plus proche de chez vous. Mais vous n’arriverez à voler que les audio-guides.");
    horoscopes.insert("TAUREAU", "On vous demandera d’abattre le chanteur Garou. Attention : il ne peut être vaincu qu’avec des balles en argent.");
    horoscopes.insert("GEMEAUX", "Vous apprendrez enfin que vos parents ont dû faire « crac-crac » pour vous avoir. Votre vie prendra une tournure inattendue, et vous déciderez de tout plaquer pour aller vivre à Saint-Etienne.");
    horoscopes.insert("CANCER", "Un collègue quinquagénaire s’amusera à ponctuer vos phrases de « … ou pas ! » et trouvera ça drôle. Renversez-le avec votre véhicule.");
    horoscopes.insert("LION", "À la suite d’un cafouillage administratif, vous devrez remplacer la bouillotte personnelle d’Emmanuel Macron.");
    horoscopes.insert("VIERGE", "Vous devrez malheureusement faire la conversation avec des fans de course (ils vous diront « Cette année je prépare l’Ironman » ce genre de trucs). Tenez bon.");
    horoscopes.insert("BALANCE", "Si c’est la semaine de votre anniversaire, vous recevrez un faux diplôme pour exercer la chirurgie esthétique. C’est une nouvelle vie qui commence.");
    horoscopes.insert("SCORPION", "Vous aurez du Pascal Obispo dans la tête toute la semaine ( « L’Île aux Oiseaux » pour être précis). Vigilance.");
    horoscopes.insert("FURET", "Vous serez invité(e) sur l’île où vit encore Michael Jackson. Ou celle de Jeffrey Epstein, c’est comme vous voulez.");
    horoscopes.insert("SAGITTAIRE", "Deux flics seront chargés de vous retrouver et vous mettre sous les verrous. Il y aura un vétéran à deux mois de la retraite, et un jeune fou aux méthodes expéditives qui se « contrefout du règlement ».");
    horoscopes.insert("CAPRICORNE", "Vous paierez un abonnement à 199,99 € par mois pour regarder Nicolas Sarkozy dans sa cellule 24h/24. On vous comprend.");
    horoscopes.insert("VERSEAU", "Vous ferez une reconversion pour devenir bras droit d’un parrain de la drogue. Vous devrez faire craquer vos phalanges et dire « Compris patron ! » ou « Considérez qu’il est déjà mort ».");
    horoscopes.insert("POISSONS", "Vous devrez vous prononcer sur une question cruciale : êtes-vous pour ou contre le comique de répétition ?");

    let key = sign.to_uppercase();
    if let Some(txt) = horoscopes.get(key.as_str()) {
        Ok(txt.to_string())
    } else {
        Ok("Signe inconnu. Tapez correctement votre signe.".to_string())
    }
}
