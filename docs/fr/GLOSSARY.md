# Glossaire

Définitions des termes métier et techniques utilisés dans le Framework AEI.

### Framework AEI (AEIF)
Le Framework d'Intelligence Autonome et Évolutive, une bibliothèque Rust pour construire des réseaux de neurones dynamiques multi-agents. Voir [README.md](../README.md).

### Conception pilotée par le domaine (DDD)
Approche de développement logiciel qui modélise des domaines complexes au moyen de contextes bornés et d'un langage omniprésent.

### Commande
Intention de modifier l'état du système. Les commandes sont gérées par des gestionnaires dédiés et produisent des événements en cas de succès. Voir [src/application/commands.rs](../../src/application/commands.rs).

### Requête
Demande en lecture seule traitée séparément des commandes. Voir [src/application/queries.rs](../../src/application/queries.rs).

### Événement
Enregistrement immuable décrivant un changement d'état survenu suite au traitement d'une commande. Les événements sont persistés dans le magasin d'événements et peuvent être rejoués pour reconstruire l'état. Voir [src/domain/events.rs](../../src/domain/events.rs).

### Type d'événement
Étiquette catégorisant une entrée de mémoire, utilisée pour interroger des types d'expériences spécifiques.

### Magasin d'événements
Stockage persistant chargé d'ajouter et de charger les événements de domaine. Les implémentations se trouvent dans [src/infrastructure/event_store.rs](../../src/infrastructure/event_store.rs).

### JsonlEventStore
Magasin d'événements générique persistant les événements au format JSON Lines. Voir [src/infrastructure/jsonl_event_store.rs](../../src/infrastructure/jsonl_event_store.rs).

### Event Sourcing
Modèle architectural où l'état est dérivé d'un journal d'événements plutôt que stocké directement. Le Framework AEI reconstruit les agrégats en rejouant les événements du magasin.

### Gestionnaire de commande
Composant qui valide et exécute une commande, émettant un ou plusieurs événements. Exemples : [AddRandomNeuronHandler](../../src/application/add_random_neuron.rs) et [RemoveRandomNeuronHandler](../../src/application/remove_random_neuron.rs).

### NetworkHandlerBase
Structure de base regroupant le magasin d'événements, le réseau hydraté et le générateur de nombres aléatoires pour les gestionnaires de commandes opérant sur les réseaux.

### MemoryHandlerBase
Structure de base regroupant le magasin d'événements mémoire et la [`AdaptiveMemory`](../../src/domain/memory) hydratée, fournissant des fonctions utilitaires pour persister les événements et élaguer les entrées.

### Gestionnaire de requête
Composant qui sert une requête en lisant depuis une projection ou un modèle de lecture. Voir [src/application/query_handler.rs](../../src/application/query_handler.rs).

### Projection
Processus transformant les événements en un modèle de lecture adapté aux requêtes. Les projections se trouvent sous [src/infrastructure/projection](../../src/infrastructure/projection).

### Modèle de lecture
État optimisé pour répondre aux requêtes, maintenu par des projections dérivées du flux d'événements.

### Agrégat
Objet de domaine qui applique des invariants et reconstruit son état en appliquant des événements, comme [Network](../../src/domain/network.rs).

### Mémoire adaptative
Tampon borné stockant des expériences scorées. Géré via l'event sourcing et interrogé par des projections. Voir [src/domain/memory](../../src/domain/memory).

### Neurone
Unité de traitement de base du réseau. Définie dans [src/domain/neuron.rs](../../src/domain/neuron.rs).

### Synapse
Connexion entre neurones transportant des signaux. Définie dans [src/domain/synapse.rs](../../src/domain/synapse.rs).

### AddRandomNeuronCommand
Commande qui introduit un nouveau neurone dans le réseau à un emplacement aléatoire. Implémentée dans [src/application/add_random_neuron.rs](../../src/application/add_random_neuron.rs).

### RemoveRandomNeuronCommand
Commande qui supprime un neurone choisi aléatoirement du réseau. Implémentée dans [src/application/remove_random_neuron.rs](../../src/application/remove_random_neuron.rs).

### AddRandomSynapseCommand
Commande qui crée une synapse entre deux neurones choisis aléatoirement. Implémentée dans [src/application/add_random_synapse.rs](../../src/application/add_random_synapse.rs).

### RemoveRandomSynapseCommand
Commande demandant la suppression d'une synapse aléatoire du réseau. Implémentée dans [src/application/remove_random_synapse.rs](../../src/application/remove_random_synapse.rs).

### CreateNeuron Command
Commande qui insère un neurone avec un identifiant et une activation spécifiques dans le réseau. Gérée par `CommandHandler`.

### RemoveNeuron Command
Commande qui supprime un neurone par son identifiant et élimine les synapses associées. Gérée par `CommandHandler`.

### MutateRandomSynapseWeightCommand
Commande qui applique un bruit gaussien au poids d'une synapse choisie aléatoirement. Implémentée dans [src/application/mutate_random_synapse_weight.rs](../../src/application/mutate_random_synapse_weight.rs).

### SynapseWeightMutated
Événement de domaine enregistrant la modification du poids d'une synapse. Émis par `MutateRandomSynapseWeightHandler`.

### SetSynapseWeightCommand
Commande qui assigne un poids spécifique à une synapse existante. Implémentée dans [src/application/set_synapse_weight.rs](../../src/application/set_synapse_weight.rs).

### SynapseWeightSet
Événement de domaine enregistrant la mise à jour explicite du poids d'une synapse. Émis par `SetSynapseWeightHandler`.

### NeuronAdded
Événement de domaine émis lorsqu'un neurone est ajouté au réseau, suite à `CreateNeuron`.

### NeuronRemoved
Événement de domaine émis lorsqu'un neurone est retiré du réseau, suite à `RemoveNeuron`.

### Activation
Fonction non linéaire appliquée à l'entrée d'un neurone pour produire sa sortie.

### Score de curiosité
Métrique représentant le potentiel exploratoire d'un neurone ou d'une synapse. Recalculé via `RecalculateCuriosityScoreCommand` et persisté par des événements `CuriosityScoreUpdated`.
