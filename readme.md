# Rust Web Development Projects

This repository contains small Rust web projects built to explore different approaches to backend and server-rendered development. Each folder is a standalone Cargo project that focuses on a specific framework and use case.

## Projects

### `rust-actix-web`

A small JSON API built with [Actix Web](https://actix.rs/).

What it demonstrates:
- Configuring an HTTP server with environment variables
- Organizing code with `handlers`, `models`, and `routes`
- Building JSON request and response types with `serde`
- Generating IDs with `uuid`
- Defining REST-style endpoints under `/api`

Available endpoints:
- `GET /api/health` returns a simple health response
- `POST /api/users` creates a user from JSON input
- `GET /api/users/{id}` returns a mock user for the provided UUID

Run it locally:

```bash
cd rust-actix-web
cargo run
```

By default the server starts at:

```text
http://127.0.0.1:8080
```

Optional environment variables:

```bash
HOST=127.0.0.1
PORT=8080
```

Example requests:

```bash
curl http://127.0.0.1:8080/api/health
```

```bash
curl -X POST http://127.0.0.1:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"dvdandrades","email":"dvd@example.com"}'
```

```bash
curl http://127.0.0.1:8080/api/users/550e8400-e29b-41d4-a716-446655440000
```

### `rust-rocket`

A server-rendered web app built with [Rocket](https://rocket.rs/) and dynamic templates.

What it demonstrates:
- Routing with Rocket
- Rendering templates with `rocket_dyn_templates`
- Handling and validating form input
- Re-rendering the form with validation errors
- Redirecting with flash messages after a successful submission
- Serving static assets from `/public`

Application flow:
- `GET /` shows a form asking for first and last name
- `POST /` validates the submitted form
- `GET /hi?name=...` renders a greeting page after a successful submission

Run it locally:

```bash
cd rust-rocket
cargo run
```

Rocket usually starts on:

```text
http://127.0.0.1:8000
```

## Getting Started

Clone the repository and run either project independently:

```bash
git clone https://github.com/Dvdandrades/Rust-Web-Development.git
cd rust_web_development
```

Then enter one of the project folders and start it with:

```bash
cargo run
```

## Why This Repository Exists

This collection is meant to make it easier to compare how Rust web frameworks feel in practice across common tasks such as:
- Building JSON APIs
- Structuring route and handler code
- Handling forms and validation
- Rendering server-side HTML
- Serving static assets

## Notes

- Each folder is intentionally small and focused on one idea.
- The projects can be studied independently.
- Additional Rust web examples can be added as the repository grows.
