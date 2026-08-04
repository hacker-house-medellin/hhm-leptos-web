# hhm-leptos-web

Leptos SSR + Axum + WebSocket comparison server for Hacker House Medellín.

**Product:** Hacker House Medellín — Operations software for an entrepreneur coliving and coworking community.

Run rooms, desks, member stays, community events, access workflows, and day-to-day operations for a hacker house in Medellín, Colombia.

## Safety and production boundary

The bootstrap does not implement payments, identity verification, door-control hardware, or Colombian lodging compliance. Add those only after security and local regulatory review.

This repository is an executable bootstrap, not a production deployment. Before live
use, add authentication, tenant authorization, rate limits, durable migrations,
observability, backups, incident response, dependency review, and secret management.


This repository is intentionally independent from the MASH production-oriented surface so framework tradeoffs can be measured with the same health and WebSocket contracts.
