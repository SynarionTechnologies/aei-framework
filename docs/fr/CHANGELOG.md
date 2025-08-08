# Changelog

Toutes les modifications notables de ce projet sont documentées dans ce fichier.

Le format est basé sur [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
et ce projet adhère à [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Non publié]
### Ajouté
- Mutation aléatoire de l’activation des neurones via `MutateRandomNeuronActivationCommand` et `MutateRandomNeuronActivationHandler`.
- Mutation aléatoire du poids des synapses via `MutateRandomSynapseWeightCommand` et `MutateRandomSynapseWeightHandler`.
- Exemple orienté événements et tests de commande de synapse.
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
- Création aléatoire de synapse via `AddRandomSynapseCommand` et `AddRandomSynapseHandler`.
- Ajout aléatoire de neurone orienté événements via `AddRandomNeuronCommand` et
  `AddRandomNeuronHandler`.
- Suppression aléatoire de neurone orientée événements via `RemoveRandomNeuronCommand` et
  `RemoveRandomNeuronHandler`.
- Suppression aléatoire de synapse orientée événements via `RemoveRandomSynapseCommand` et
  `RemoveRandomSynapseHandler`.
### Modifié
- Les commandes et requêtes résident désormais dans le module `application`.
- Les événements de domaine ont été déplacés sous `domain` et exposés via `domain::events`.
- Les modèles de lecture sont implémentés en tant que projections dans `infrastructure/projection`.
- La logique de propagation applique désormais les activations après les sommes pondérées et réinitialise toutes les valeurs des neurones entre les exécutions.
- Rustdoc complet pour les modules et les API publiques.
- Les identifiants des neurones et des synapses utilisent désormais `Uuid` au lieu d'index numériques.
- Suppression des modules hérités `api` et `core` ; les primitives sont déplacées dans `domain`.
- La documentation est disponible en anglais et en français sous `docs/`.
### Supprimé
- Alias de type inutilisés `NodeList` et `TopoOrder` dans l'API réseau.
- Crate `modules` vide retirée de l'espace de travail.
- Méthodes directes `Network::add_random_neuron` et `Network::remove_random_neuron`
  remplacées par des handlers orientés événements.
