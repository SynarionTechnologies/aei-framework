# Changelog

Toutes les modifications notables de ce projet sont documentées dans ce fichier.

Le format est basé sur [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
et ce projet adhère à [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Non publié]
### Ajouté
- Entraînement par rétropropagation avec `Network::train` et `Network::predict`
- Dérivées des fonctions d'activation permettant la descente de gradient
- Exemple XOR, tests et tutoriel README
- Makefile avec des tâches de développement courantes
- Workflow GitHub Actions exécutant `make ci`
- Changelog initial
- README.md
- Structures `Neuron`, `Synapse` et `Network`
- Tests unitaires de propagation
- Documentation et guides en anglais
- Tests couvrant toutes les fonctions d'activation et la propagation chaînée
- Neurones d'entrée/sortie nommés avec les API de haut niveau `set_inputs`, `get_outputs` et `propagate_inputs`
- Sérialisation et chargement JSON des réseaux via `save_json` et `load_json`
- Journalisation structurée de la progression de l'entraînement via la crate `log`.
- Création aléatoire de neurones avec `Network::add_random_neuron` générant des
  connexions automatiques.
- Suppression aléatoire de neurone avec `Network::remove_random_neuron` supprimant un neurone interne et ses synapses.
### Modifié
- La logique de propagation applique désormais les activations après les sommes pondérées et réinitialise toutes les valeurs des neurones entre les exécutions.
- Rustdoc complet pour les modules et les API publiques.
- Les identifiants des neurones et des synapses utilisent désormais `Uuid` au lieu d'index numériques.
- La structure du projet sépare désormais les primitives `core` et le module réseau `api`.
- La documentation est disponible en anglais et en français sous `docs/`.
