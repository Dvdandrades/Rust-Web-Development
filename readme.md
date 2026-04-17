# Rust Web Development Projects

This repository is a collection of small web development projects built with Rust. Each project explores a different part of the Rust web ecosystem, making this repo a practical space for learning, experimentation, and comparing frameworks.

## Projects

### `rust-actix-web`

A minimal JSON API built with [Actix Web](https://actix.rs/).

What it shows:
- Creating a basic HTTP server
- Defining a route with `GET /`
- Returning JSON responses with `serde`

Run it locally:

```bash
cd rust-actix-web
cargo run
```

Then open:

```text
http://127.0.0.1:8080
```

Expected response:

```json
{"message":"Hello from Rust API!"}
```

### `rust-rocket`

A simple server-rendered web app built with [Rocket](https://rocket.rs/) and dynamic templates.

What it shows:
- Routing with Rocket
- Handling form submissions
- Validating form input
- Rendering HTML with templates
- Serving static assets such as CSS
- Redirecting with flash messages after a successful form submission

Run it locally:

```bash
cd rust-rocket
cargo run
```

By default, Rocket usually starts on:

```text
http://127.0.0.1:8000
```

## Why This Repository Exists

This collection is meant to help explore how Rust can be used for web development across different styles of applications, including:
- JSON APIs
- Server-rendered web apps
- Form handling and validation
- Static asset serving

## Getting Started

Clone the repository and run any project independently:

```bash
git clone https://github.com/Dvdandrades/Rust-Web-Development.git
cd rust_web_development
```

Then choose one of the project folders and run:

```bash
cargo run
```

## Notes

- Each folder is a standalone Cargo project.
- The projects are intentionally small and focused.
- More Rust web examples can be added over time as the collection grows.
