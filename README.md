# AOS Dispatcher

AOS Dispatcher is a Rust-based server application that handles TEE (Trusted Execution Environment) and OPML (OPtimistic Machine Learning) requests and responses.

## Features

- TEE and OPML question handling
- Worker registration and heartbeat monitoring
- Database integration with PostgreSQL
- Asynchronous communication using Tokio
- RESTful API endpoints

## Getting Started

### Prerequisites

- Rust (latest stable version)
- PostgreSQL

### Installation

1. Clone the repository:

```
git clone https://github.com/your-username/aos-dispatcher.git
cd aos-dispatcher
```

2. Set up the database:
   - Create a PostgreSQL database
   - Set the `DATABASE_URL` environment variable in a `.env` file:

```
DATABASE_URL=postgres://username:password@localhost/database_name
```

3. Run database migrations:

```
diesel migration run
```

4. Build and run the project:

```
cargo run
```

## Project Structure

The main components of the project are:

- `src/main.rs`: Entry point of the application
- `src/server/`: Server-related code
- `src/tee/`: TEE-related handlers and models
- `src/opml/`: OPML-related handlers and models
- `src/config.rs`: Configuration settings
- `src/db/`: Database-related code
- `src/schema.rs`: Database schema definitions

## API Endpoints

- `/ping`: Health check endpoint
- `/sign`: Sign a hash
- `/register_worker`: Register a new worker
- `/receive_heart_beat`: Receive worker heartbeats
- `/api/question`: Handle TEE questions
- `/api/tee_callback`: Handle TEE callbacks
- `/api/opml_question`: Handle OPML questions
- `/api/opml_callback`: Handle OPML callbacks
- `/api/list_models`: List available models
- `/admin/list_workers`: List registered workers
- `/admin/list_questions`: List all questions
- `/admin/list_answers`: List all answers

## Configuration

The application configuration is defined in the `Config` struct in `src/config.rs`. You can modify the default values or use environment variables to override them.

## Database

The project uses Diesel ORM with PostgreSQL. The database schema is defined in `src/schema.rs`, and migrations are located in the `migrations/` directory.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the [MIT License](LICENSE).
