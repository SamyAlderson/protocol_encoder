//! Fichier de tests pour la fonctionnalité

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::Encoder;
    use bincode;

    #[test]
    fn test_encoder() {
        // Création d'un encodeur
        let mut encoder = Encoder::new();

        // Envoi de données pour encoder
        let data = vec![1, 2, 3];
        let encoded_data = encoder.encode(data.clone());

        // Vérification de la longueur de l'encodage
        assert_eq!(encoded_data.len(), 3);

        // Décodage des données
        let decoded_data: Vec<u8> = bincode::deserialize(&encoded_data).unwrap();

        // Vérification de la conformité des données
        assert_eq!(decoded_data, data);
    }

    #[test]
    fn test_encoder_with_error_handling() {
        // Création d'un encodeur avec gestion d'erreurs
        let mut encoder = Encoder::new_with_error_handling();

        // Envoi de données pour encoder
        let data = vec![1, 2, 3];
        let encoded_data = encoder.encode(data.clone());

        // Vérification de la longueur de l'encodage
        assert_eq!(encoded_data.len(), 3);

        // Décodage des données
        let decoded_data: Vec<u8> = bincode::deserialize(&encoded_data).unwrap();

        // Vérification de la conformité des données
        assert_eq!(decoded_data, data);
    }

    #[test]
    fn test_encoder_with_invalid_data() {
        // Création d'un encodeur
        let mut encoder = Encoder::new();

        // Envoi de données invalides pour encoder
        let data = vec![1, 2, 3, 4];
        let encoded_data = encoder.encode(data.clone());

        // Vérification de l'erreur de décodage
        assert!(bincode::deserialize(&encoded_data).is_err());
    }
}