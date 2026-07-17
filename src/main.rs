//! Fichier principal pour la démonstration du encodeur de protocole générique

use std::env;
use std::fs;
use std::path::Path;

use serde_json::json;

use crate::lib::{Encoder, Error};

/// Fonction principale pour la démonstration
///
/// Cette fonction utilise la fonctionnalité d'encodage définie dans le fichier lib.rs
fn main() {
    // Récupération des arguments
    let args: Vec<String> = env::args().collect();

    // Vérification du nombre d'arguments
    if args.len() != 2 {
        eprintln!("Usage: {} <fichier_donnees>", args[0]);
        std::process::exit(1);
    }

    // Récupération du chemin du fichier des données
    let donnees_file = &args[1];

    // Vérification de l'existence du fichier
    if !Path::new(donnees_file).exists() {
        eprintln!("Le fichier {} n'existe pas", donnees_file);
        std::process::exit(1);
    }

    // Lecture du contenu du fichier
    let contenu = fs::read_to_string(donnees_file).unwrap_or_else(|_| {
        eprintln!("Impossible de lire le fichier {}", donnees_file);
        std::process::exit(1);
    });

    // Sérialisation en JSON
    let donnees: serde_json::Value = serde_json::from_str(&contenu).unwrap_or_else(|err| {
        eprintln!("Erreur de sérialisation : {}", err);
        std::process::exit(1);
    });

    // Encodage des données
    let donnees_encoded = Encoder::new().encode(donnees);

    // Affichage du résultat
    println!("Données encodées : {}", donnees_encoded);
}

fn main_run() -> Result<(), Error> {
    // Appel de la fonction principale
    main();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        // Test de la fonction principale
        main_run();
    }
}