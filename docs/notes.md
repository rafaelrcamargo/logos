## Clients

- Web

## Services

- Core
- Auth

## Flow

```mermaid
flowchart TB
    Web --> Auth
    Auth <--> Provider
    Auth -- Token --> Web

    Web --> Core
    Core --> NEW
    NEW --> Queue
    Queue --> DB
    DB -- Status --> Core
```

When a writter publish an article, it will be added to the `Feed` table that each user has, whe a user views a post it will automatically be removed from the `Feed` table and will increase a view counter in the original post

## Data

### User

| Field      | Type      | Description        |
| ---------- | --------- | ------------------ |
| id         | int       | User ID            |
| email      | string    | User email         |
| created_at | timestamp | User creation date |
