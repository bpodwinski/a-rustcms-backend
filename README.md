# a-rustcms-back

A robust and scalable **Content Management System (CMS) backend** built in **Rust**, leveraging modern frameworks and libraries for high performance and security

## Features

- **Authentication & Authorization**
  - Secure login using **JWT** (JSON Web Tokens)
  - Role-based access control

- **Content Management**
  - Full **CRUD** operations for:
    - **Posts**
    - **Categories**
    - **Tags**
    - **Users**
    - **Roles**
  - Manage relationships like **Post-Categories** and **User-Roles**

- **Database**
  - Uses **PostgreSQL** with async operations via `sqlx`
  - Structured SQL migrations using the `/migrations` directory

- **API Documentation**
  - Auto-generated with **OpenAPI (Swagger)** using the `utoipa` library

- **Security**
  - Secure password hashing with **argon2**
  - Middleware-based JWT validation

- **Error Handling**
  - Centralized error handling middleware

- **Logging**
  - Structured logging with `env_logger`

## Technologies Used

- **Rust**
- **tokio** - Asynchronous runtime for efficient concurrency
- **ntex** - Web framework for asynchronous operations
- **sqlx** - Async database access for PostgreSQL
- **dotenv** - Environment variable management
- **jsonwebtoken** - JWT handling
- **argon2** - Password hashing
- **utoipa** - OpenAPI documentation generation
- **serde** - Data serialization/deserialization

## Project Structure

```bash
├── Cargo.toml               # Rust dependencies and project configuration
├── .env.example             # Example environment configuration
├── migrations/              # SQL migrations for the database
├── src/
│   ├── main.rs              # Entry point of the application
│   ├── config/              # Project configuration
│   ├── controllers/         # HTTP request handlers per entity
│   ├── dtos/                # Data Transfer Objects for structured requests/responses
│   ├── handlers/            # Utility handlers (error handling, slug generation, etc.)
│   ├── middlewares/         # Custom middlewares (auth, error handling)
│   ├── models/              # Database models
│   ├── repositories/        # Data access layer using SQLx
│   ├── services/            # Business logic layer
│   ├── tests/               # Unit and integration tests
│   └── validators/          # Custom validators for data integrity
```

## Setup and Installation

### Prerequisites
- Rust (latest stable version)
- PostgreSQL

### Clone the Repository
```bash
git clone https://github.com/bpodwinski/a-rustcms-backend.git
cd a-rustcms-back
```

### Install Dependencies
```bash
cargo build
```

### Configure Environment Variables
Copy the example environment configuration and set your values:
```bash
cp .env.example .env
```

### Run Database Migrations
```bash
sqlx migrate run
```

### Run the Application
```bash
cargo run
```

### Access API Documentation
- Visit `http://127.0.0.1:8080/swagger/` for the Swagger UI

## Running Tests
```bash
cargo test
```

## API Endpoints

### Authentication
- `POST /api/v1/auth/login` - User login and JWT issuance

### Posts
- `GET /api/v1/posts` - List all posts
- `POST /api/v1/posts` - Create a new post
- `GET /api/v1/posts/{id}` - Retrieve a post by ID
- `PUT /api/v1/posts/{id}` - Update a post by ID
- `DELETE /api/v1/posts/{id}` - Delete a post by ID

### Categories, Tags, Users, Roles
- Similar CRUD endpoints following the same structure.

## Best Practices Implemented
- Layered architecture: **Controller > Service > Repository**
- Middleware-driven approach for clean code
- Type-safe query execution using `sqlx`
- Structured and centralized error handling
- Role-based route protection

## Security
- JWT-based authentication
- Secure password storage using **argon2**
- Data validation

## Future Improvements
- Add rate limiting for enhanced security
- Improve testing coverage, especially for edge cases
- Add multi-language support
- Implement caching for frequently accessed resources

