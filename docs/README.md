# 📰 Logos (WIP)

> A news feed, a really good one!

## 🌟 Services

### 🔑 Gatekeeper

The Gatekeeper service is a REST API that validates and authorizes users. It handles user authentication and permissions.

### 🔒 Locksmith

The Locksmith service authenticates users via OAuth2.

### 🧑‍💼 User Registration Service

The User Registration service is responsible for registering users. It uses a Graph database to store user data.

### 📝 Post Service

The Post service is responsible for registering posts and notifying followers. It consumes a RabbitMQ queue to register posts in order and sends notifications to followers.

## 🤝 Contributing

If you would like to contribute to this project, please follow the following steps:

1. Fork the repository
2. Create a new branch for your changes: `git checkout -b feature/my-feature`
3. Make your changes
4. Test your changes: `npm test`
5. Commit your changes: `git commit -am 'Add some feature'`
6. Push to the branch: `git push origin feature/my-feature`
7. Create a new Pull Request

## 📜 License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0)>
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT)>

at your option.
