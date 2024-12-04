# Wedding Reservation API

This project is a Wedding Reservation API built using Rust.

## Features

- **RESTful API**: Provides endpoints for managing wedding reservations.
- **Database Integration**: Employs SeaORM for seamless database interactions.
- **Authentication**: Authentication and Middleware
- **CORS Handling**: Handling CORS with Tower HTTP CorsLayer

## Prerequisites

- **Rust**: Ensure that Rust is installed on your system. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- **Database**: Set up a PostgreSQL database and note the connection details.
- **Docker**: if you want build this project using docker, you need docker, You can install it from [docker.com](https://www.docker.com/)

## Getting Started

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/maulanasdqn/wedding-api.git
   cd wedding-api
   ```

2. **Set Up Environment Variables**:

   Create a `.env` file in the root directory with the following content:

   ```env
   DATABASE_URL=postgres://username:password@localhost/database_name
   ```

3. **Install Dependencies**:

   ```bash
   cargo build --release
   ```
    ```bash
   cargo install sea-orm-cli
   ```

4. **Run Database Migrations**:

   ```bash
   sea-orm-cli migrate up
   ```

5. **Start the Server**:

   ```bash
   cargo run -p api --release
   ```

   The API will be accessible at `http://localhost:3000/api/docs`.

## Endpoints

**Get Reservation** : `http://localhost:3000/api/reservations`
**Create Reservation**: `http://localhost:3000/api/reservations/create`

## Contributing

Contributions are welcome! Please fork the repository and create a pull request with your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Axum](https://github.com/tokio-rs/axum)
- [SeaORM](https://github.com/SeaQL/sea-orm)
