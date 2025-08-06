# 🚀 Framework AEI – ROADMAP

_Framework d'Intelligence Autonome & Évolutive (AEIF)_

---

## Phase 0 — Fondations & Spécifications du projet

- [x] **Initialiser le dépôt :**
  - Workspace Rust
  - Structure de crates modulaires
  - CI/CD pour la compilation et les tests
- [ ] **Documentation :**
  - [x] `README.md` (vision, objectifs, démarrage rapide)
  - [ ] `GLOSSARY.md` ou section

---

## Phase 1 — Architecture de base & spécifications

- [ ] **Définir les traits et interfaces clés :**
  - `NeuralNetwork` : interface minimale pour tout réseau
  - `KnowledgeModule` : interface standard pour les compétences attachables
  - `MemoryStore` : backend de stockage abstrait
- [ ] **Squelettes de modules :** code minimal pour chaque crate avec responsabilités claires
- [ ] **Commentaires de doc :** chaque interface documentée (style Rust)

---

## Phase 2 — Moteur de réseau neuronal minimal

- [ ] **Module `crates/core/nn` :**
  - Structures `Neuron`, `Layer`, `Network`
  - Propagation avant (sigmoïde/ReLU)
  - Structure de couches extensible
- [ ] **Conception de l'API :**
  - Instanciation de réseaux avec topologie arbitraire
  - Fonction de propagation : entrée → sortie
- [ ] **Exemples :**
  - Utilisation simple dans `examples/` (ex. XOR, régression linéaire)

---

## Phase 3 — Persistance & abstraction de mémoire

- [ ] **`crates/memory` :**
  - Trait `MemoryStore` : modèle put/get/save/load
  - Backend en mémoire (HashMap)
  - Backend fichier (JSON/YAML/TOML)
- [ ] **Sérialisation :**
  - Sauvegarder et restaurer les poids/config du réseau
- [ ] **Démo :**
  - Stocker/charger l'état de l'agent et gérer les checkpoints

---

## Phase 4 — Modules de connaissance & mémoire sémantique

- [ ] **`crates/modules` :**
  - Définir le trait `KnowledgeModule` (versionné, attacher/détacher, identifier)
  - Modules de base : p. ex. "math", "echo", "counter"
- [ ] **Mémoire sémantique :**
  - Structurer la connaissance en graphes ou embeddings
  - Hooks de persistance pour stockage long terme
- [ ] **API & documentation :**
  - Comment construire/étendre des modules et alimenter la mémoire sémantique

---

## Phase 5 — Moteur NLP

- [ ] **Pipeline de traitement de texte :**
  - Tokenisation, stemming, parsing de base
  - Génération d'embeddings pour intégration avec la mémoire sémantique
- [ ] **Modules de compréhension du langage :**
  - Reconnaissance d'intention
  - Extraction d'entités liées à la base de connaissances
- [ ] **Exemples :**
  - Démo chatbot simple montrant les capacités NLP

---

## Phase 6 — Runtime & orchestration des agents

- [ ] **`crates/runtime` :**
  - Planificateur d'agents (mono & multi-agent)
  - Boucle d'événements ou système à ticks
- [ ] **Interactions :**
  - Communication agent-agent (bus de messages/événements)
  - Orchestration dynamique des modules
- [ ] **Exemples :**
  - Scénario multi-agent dans `examples/` (agents se saluant, échangeant des infos)

---

## Phase 7 — Apprentissage, adaptation & raisonnement

- [ ] **Boucle d'apprentissage :**
  - Rétropropagation pour réseaux simples
  - API d'entraînement incrémental
- [ ] **Capacités de raisonnement :**
  - Raisonnement basé sur règles ou heuristiques sur la mémoire sémantique
  - Modules simples de planification/décision
- [ ] **Métriques & évaluation :**
  - Suivre la performance et la progression de l'apprentissage

---

## Phase 8 — Tests, documentation et démos

- [ ] **Tests unitaires & d'intégration :**
  - Couverture pour les crates et modules principaux
- [ ] **Documentation complète :**
  - Guides, docs API, tutoriels
- [ ] **Démos vitrines :**
  - Dans `examples/` : créer, entraîner, persister et interagir avec des agents

---

## Phase 9 — Communauté, extensibilité & release

- [ ] **Onboarding open source :**
  - [ ] Issues & jalons sur GitHub
  - [x] Guide de contribution clair
- [ ] **Extensibilité :**
  - Système de plugins et templates d'extension
  - Hooks FFI pour d'autres langages
- [ ] **Première release (v0.1) :**
  - Stabilisation de l'API
  - Publication sur crates.io et annonce à la communauté

---

## Objectifs étendus
- [ ] Interface web ou CLI pour gérer les agents
- [ ] Support d'architectures neuronales alternatives (RNN, CNN)
- [ ] Persistance sur backends cloud
- [ ] Interface avec des données réelles (APIs, web scraping)
- [ ] Intégration avec d'autres langages (bindings Python, etc.)

---

## Comment contribuer
- Discuter des nouvelles fonctionnalités via les issues GitHub
- Proposer vos propres modules ou compétences
- Soumettre des PR avec tests et documentation

---

*Cette roadmap est itérative et pourra évoluer selon les retours de la communauté et l'avancement du projet.*
