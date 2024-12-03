# Wedding Reservation API

This project is a Wedding Reservation API built using Rust, leveraging the Axum web framework and SeaORM for database interactions.

## Features

- **RESTful API**: Provides endpoints for managing wedding reservations.
- **Asynchronous Operations**: Utilizes Rust's async capabilities for efficient performance.
- **Database Integration**: Employs SeaORM for seamless database interactions.

## Prerequisites

- **Rust**: Ensure that Rust is installed on your system. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- **Database**: Set up a PostgreSQL database and note the connection details.

## Getting Started

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/maulanasdqn/rust-wedding.git
   cd rust-wedding
   ```

2. **Set Up Environment Variables**:

   Create a `.env` file in the root directory with the following content:

   ```env
   DATABASE_URL=postgres://username:password@localhost/database_name
   ```

3. **Install Dependencies**:

   ```bash
   cargo build
   ```

4. **Run Database Migrations**:

   ```bash
   sea-orm-cli migrate up
   ```

5. **Start the Server**:

   ```bash
   cargo run
   ```

   The API will be accessible at `http://localhost:3000`.

## Usage

- **Create a Reservation**: Send a `POST` request to `/api/reservations/create` with the reservation details.
- **Get Reservations**: Send a `GET` request to `/api/reservations` to retrieve all reservations.
- **Update a Reservation**: Send a `PUT` request to `/api/reservations/{id}/update` with the updated details.
- **Delete a Reservation**: Send a `DELETE` request to `/api/reservations/{id}/delete` to remove a reservation.

## Contributing

Contributions are welcome! Please fork the repository and create a pull request with your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Axum](https://github.com/tokio-rs/axum)
- [SeaORM](https://github.com/SeaQL/sea-orm)
