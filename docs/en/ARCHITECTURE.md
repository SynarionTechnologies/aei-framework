# Architecture

This document describes the high-level design of the Autonomous & Evolutive
Intelligence Framework.

## Domain-Driven Design

The framework models neurons, synapses and networks as domain entities. The
`Network` aggregate applies events to maintain consistency across its internal
state.

## Event Sourcing

Every state change is captured as an immutable event stored in an append-only
log. Replaying the log reconstructs the exact state of the network, providing
traceability and reproducibility.

## CQRS

Commands modify the system and queries read from it. Command handlers persist
new events and update the aggregate, while query handlers serve read requests
from the in-memory projection.

## Extensibility

Adapters for persistence or transport live under `infrastructure` and can be
replaced to target different backends such as databases or message queues.
