# QuoteYourLife Backend

QuoteYourLife Backend is a Rust-based web service for managing and serving quotes. It uses Diesel ORM for database interactions and is designed to be fast, reliable, and easy to deploy.

## Features
- RESTful API for managing quotes
- PostgreSQL database integration via Diesel
- Easy configuration via environment variables
- Migration support

## Getting Started

### Prerequisites
- Rust (https://www.rust-lang.org/tools/install)
- PostgreSQL
- Diesel CLI (`cargo install diesel_cli`)

### Setup
1. **Clone the repository:**
   ```sh
   git clone https://github.com/LyanDoo/quoteyourlife-be.git
   cd quoteyourlife-be
   ```
2. **Copy and configure environment variables:**
   ```sh
   copy .example.env .env
   # Edit .env to match your database settings
   ```
3. **Run database migrations:**
   ```sh
   diesel migration run
   ```
4. **Build and run the server:**
   ```sh
   cargo run
   ```

## Project Structure
- `src/` - Main source code
- `migrations/` - Diesel migration files
- `binaries/` - Compiled binaries
- `Cargo.toml` - Rust dependencies and project metadata
- `.env` - Environment configuration

## API Endpoints
'/quotes' - GET
Get all the endpoint

'/quotes' - POST with Json data
{
    "text",
    "author"
}

create new quote with 'text' and 'author' provided via json

## License
MIT
