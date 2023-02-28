# Notes :)

Just hacking around with some ideas.

## Architecture

> Note: ! = Ongoing, ? = Maybe, ~ = Next

### Clients

- ! Web
- ? Mobile

### Services

- ~ Core
- ! Auth
- ~ Feed

## Flow

A generalized flow of the system.

```mermaid
flowchart TB
    Web --> Auth

    %% Auth flow
    Auth <-.-> A(( ))
    A(( )) <-. State .-> Provider
    A(( )) <-. CSRF .-> Temp[(Temp)]
    Auth -- Session --> Web
    Auth -- User --> Core
    Core --> Users[(Users)]

    %% Web flow
    Core <-. User .-> Web
    Core <-. Posts .-> Web
    Core <-. Users .-> Web

    %% Feed flow
    Core -. User .-> Feed
    Feed --> RabbitMQ
    RabbitMQ --> a[Users Feed]
    RabbitMQ --> a[Users Feed]
    RabbitMQ --> a[Users Feed]
```

## Data

Just some basic data structures, for visualization purposes.

### User

| Field      | Type      | Description        |
| ---------- | --------- | ------------------ |
| id         | int       | User ID            |
| email      | string    | User email         |
| created_at | timestamp | User creation date |

## TODOs

- [x] Better error handling

## Stack

Tech I'm using or planning to use.

### Client

Those are the platforms I'm planning to support.

- Web
  - [Next.js](https://nextjs.org/)
  - [Tailwind CSS](https://tailwindcss.com/)

### Service

And these are the main services I'm planning to use.

- Core

  - [Rust](https://www.rust-lang.org/)
  - [Actix](https://actix.rs/)

- Auth

  - [Rust](https://www.rust-lang.org/)
  - [Actix](https://actix.rs/)
  - [Redis](https://redis.io/)
  - [OAuth 2](https://oauth.net/2/)
    - Providers: GitHub, Discord, Spotify.
  - [MongoDB](https://www.mongodb.com/) OR [Neo4j](https://neo4j.com/) (or other graph DB)

- Feed
  - [Rust](https://www.rust-lang.org/)
  - [Actix](https://actix.rs/)
  - [RabbitMQ](https://www.rabbitmq.com/)
