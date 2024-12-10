# Wedding Reservation API

This project is a Wedding Reservation API built using Rust.

## Features

- **RESTful API**: Provides endpoints for managing wedding reservations.
- **Database Integration**: Employs SeaORM for seamless database interactions.
- **Authentication**: Authentication and Middleware
- **CORS Handling**: Handling CORS with Tower HTTP CorsLayer
- **API Docs**: OpenAPI Swager Ready
- **Minio Object Storage**: Integrated with Minio Object Storage

## Prerequisites

- **Rust**: Ensure that Rust is installed on your system. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- **Database**: Set up a SurealDB database and note the connection details, You can install it from [surealdb.com](https://surrealdb.com/)
- **Docker**: if you want build this project using docker, you need docker, You can install it from [docker.com](https://www.docker.com/)
- **Nix**: if you want build this project using nix, you need nix, You can install it from [nixos.org](https://nixos.org/)

## Getting Started

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/maulanasdqn/wedding-api.git
   cd wedding-api
   ```

2. **Set Up Environment Variables**:

   Copy a `.env.exanple` file:

   ```env
   cp .env.example .env
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

   The API will be accessible at `http://localhost:3000`.

## Docker

1. **Build the Docker Image**:

   ```bash
   docker build -t wedding-api .
   ```

2. **Run the Docker Container**:

   ```bash
   docker run -p 3000:3000 -e DATABASE_URL=postgres://username:password@localhost/database_name wedding-api
   ```

   The API will be accessible at `http://localhost:3000/api/docs`.


## Using Nix as Builder

1. **Install Nix**:

   ```bash
   curl -L https://nixos.org/nix/install | sh
   ```

2. **Switch to Nix Shell or Nix Flake**:

   ```bash
   nix develop
   ```

3. **Build the Project**:

   ```bash
   nix build
   ```

4. **Run the Server**:

   ```bash
    nix run
   ```

## Endpoints

- **API Docs**
`http://localhost/3000/api/docs` 
- **Get Reservations** : `http://localhost:3000/api/reservations`
- **Create Reservation**: `http://localhost:3000/api/reservations/create`
- **Upload Files** `http://localhost:3000/api/upload`

## Contributing

Contributions are welcome! Please fork the repository and create a pull request with your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Axum](https://github.com/tokio-rs/axum)
- [SeaORM](https://github.com/SeaQL/sea-orm)
- SurealDB(https://surealdb.com)
