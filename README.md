# protocol_encoder
[![Rust][1]][2]
[![MIT Licence][3]][4]
[![CI][5]][6]

## Description

Le projet `protocol_encoder` est une implémentation d'un encodeur de protocole générique en Rust. Son objectif est de fournir une bibliothèque flexible et facile à utiliser pour encoder et déncoder des données selon un protocole spécifique.

## Fonctionnalités

* Encodeur et décodeur de protocole générique
* Support pour différents types de données (strings, nombres, structures, etc.)
* Possibilité d'ajouter et de supprimer des champs dynamiquement
* Fonctionnalités de validation et d'erreur pour garantir la sécurité et la cohérence des données

## Installation

Pour installer le projet, exécutez la commande suivante dans votre terminal :
```bash
cargo build
```
Pour utiliser le projet, vous pouvez simplement ajouter le dépendance suivante à votre fichier de configuration `Cargo.toml` :
```toml
[dependencies]
protocol_encoder = "0.1.0"
```
## Usage avec exemples

Voici un exemple de code pour utiliser le projet :
```rust
use protocol_encoder::{Encoder, Decoder};

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let mut encoder = Encoder::new();
    encoder.add_field("data", &data);

    let encoded_data = encoder.encode();
    println!("Encoded data: {:?}", encoded_data);

    let mut decoder = Decoder::new();
    decoder.add_field("data", &encoded_data);
    let decoded_data = decoder.decode();
    println!("Decoded data: {:?}", decoded_data);
}
```
## Architecture du projet

Le projet est composé de plusieurs modules :

* `src/lib.rs` : Fichier de définition de la fonctionnalité
* `src/main.rs` : Fichier principal pour la démonstration
* `tests/lib.rs` : Fichier de tests pour la fonctionnalité
* `Cargo.toml` : Fichier de configuration de Cargo

## Contribuer

Si vous souhaitez contribuer au projet, vous pouvez suivre les étapes suivantes :

1. Cloner le projet à partir de GitHub
2. Créer une nouvelle branche pour votre feature
3. Commiter et pousser vos modifications
4. Créer une pull request pour soumettre vos modifications

## Licence

Le projet `protocol_encoder` est sous licence MIT. Vous pouvez trouver la licence dans le fichier `LICENSE`.

[1]: https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust
[2]: https://www.rust-lang.org/
[3]: https://img.shields.io/badge/Licence-MIT-000000?style=flat-square
[4]: https://opensource.org/licenses/MIT
[5]: https://img.shields.io/badge/CI-CircleCI-000000?style=flat-square&logo=circleci
[6]: https://circleci.com/