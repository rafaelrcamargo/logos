# Notes

Just hacking around with some ideas.

## Flow

A generalized flow of the system.

```mermaid
flowchart TB
    Client --> Auth

    %% Auth flow
    Auth --> A(( ))
    A(( )) -. State .-> Provider
    A(( )) -. CSRF .-> Temp[(Temp)]
    Auth -- User --> Users[(Users)]

    %% User flow

    %% Posts flow
    Client -- Post --> Posts_queue[(Posts queue)]
    Posts_queue[(Posts queue)] -- Post --> Posts[(Posts)]
    Posts_queue[(Posts queue)] -.- Posted .-> Users[(Users)]

    %% Feed flow
    Client -- Feed --> Feeds_queue[(Feeds queue)]
```

## Stack

Tech I'm using or planning to use.

### Client

Those are the platforms I'm planning to support.

- Web
  - [Next.js](https://nextjs.org/)
  - [Tailwind CSS](https://tailwindcss.com/)

### Service

And these are the main services I'm planning to use.

- Auth
  - [OAuth 2](https://oauth.net/2/)
    - Providers: GitHub, Discord, Spotify.
  - [Redis](https://redis.io/)
    - For storing temporary data & sessions.
- User
  - [Neo4j](https://neo4j.com/)
    - For storing user data and provide user relations.
- Posts
  - [MeiliSearch](https://www.meilisearch.com/)
    - For full-text search + storing posts data.
- Feed
  - [RabbitMQ](https://www.rabbitmq.com/)
    - For queueing posts on user feeds.
