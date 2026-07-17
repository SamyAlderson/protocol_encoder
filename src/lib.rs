//! Module de définition de la fonctionnalité
//!
//! Ce module définit la fonctionnalité d'encodage de protocole générique.

use serde::{Serialize, Deserialize};
use serde_json;
use bincode;

/// Type de données représentant un message à encoder
#[derive(Serialize, Deserialize)]
pub struct Message {
    /// Nom du champ dans le message
    pub name: String,
    /// Valeur du champ dans le message
    pub value: String,
}

/// Encodeur de protocole générique
pub trait Encoder {
    /// Fonction d'encodage d'un message
    fn encode(&self, message: Message) -> Result<Vec<u8>, String>;
}

/// Implémentation de l'encodeur de protocole générique
pub struct ProtocolEncoder;

impl Encoder for ProtocolEncoder {
    /// Fonction d'encodage d'un message
    ///
    /// Cette fonction encode un message en utilisant le format Bincode.
    ///
    /// # Arguments
    ///
    /// * `message`: Le message à encoder
    ///
    /// # Résultats
    ///
    /// * `Ok`: Le message encode en tant que vecteur de bytes
    /// * `Err`: Une chaîne de caractères représentant l'erreur
    fn encode(&self, message: Message) -> Result<Vec<u8>, String> {
        match bincode::serialize(&message) {
            Ok(encoded_message) => Ok(encoded_message),
            Err(err) => Err(format!("Erreur d'encodage : {}", err)),
        }
    }
}

/// Désencodeur de protocole générique
pub trait Decoder {
    /// Fonction de désencodage d'un message
    fn decode(&self, encoded_message: Vec<u8>) -> Result<Message, String>;
}

/// Implémentation du désencodeur de protocole générique
pub struct ProtocolDecoder;

impl Decoder for ProtocolDecoder {
    /// Fonction de désencodage d'un message
    ///
    /// Cette fonction désencode un message en utilisant le format Bincode.
    ///
    /// # Arguments
    ///
    /// * `encoded_message`: Le message encode en tant que vecteur de bytes
    ///
    /// # Résultats
    ///
    /// * `Ok`: Le message désencode
    /// * `Err`: Une chaîne de caractères représentant l'erreur
    fn decode(&self, encoded_message: Vec<u8>) -> Result<Message, String> {
        match bincode::deserialize(&encoded_message) {
            Ok(decoded_message) => Ok(decoded_message),
            Err(err) => Err(format!("Erreur de désencodage : {}", err)),
        }
    }
}

/// Gestionnaire d'erreurs générique
pub struct ErrorHandler;

impl ErrorHandler {
    /// Fonction de gestion des erreurs
    ///
    /// Cette fonction gère les erreurs génériques en fonction de l'erreur.
    ///
    /// # Arguments
    ///
    /// * `error`: L'erreur à gérer
    ///
    /// # Résultats
    ///
    /// * Une chaîne de caractères représentant la cause de l'erreur
    pub fn handle_error(&self, error: String) -> String {
        format!("Erreur générique : {}", error)
    }
}