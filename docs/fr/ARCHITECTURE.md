# Architecture

Ce document décrit la conception de haut niveau du cadre Autonomous & Evolutive
Intelligence Framework.

## Domain-Driven Design

Le framework modélise les neurones, synapses et réseaux comme entités métiers.
L'agrégat `Network` applique les événements pour maintenir la cohérence de son
état interne.

## Event Sourcing

Chaque modification d'état est capturée comme un événement immuable stocké dans
un journal append-only. La relecture du journal reconstruit exactement l'état du
réseau, assurant traçabilité et reproductibilité.

## CQRS

Les commandes modifient le système et les requêtes le consultent. Les handlers
traitent les commandes en persistant de nouveaux événements et en mettant à jour
l'agrégat, tandis que les query handlers répondent aux lectures à partir des
projections en mémoire situées dans `infrastructure/projection`.

## Extensibilité

Les adaptateurs de persistance ou de transport se trouvent dans
`infrastructure` et peuvent être remplacés pour cibler différentes bases de
données ou files de messages.
