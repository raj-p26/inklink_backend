# Inklink

Inklink is a simple backend API service written in Rust for managing articles. It allows users to perform CRUD operations on articles, as well as authentication functionality such as user registration and login.

## Features

- User registration: Allows users to create an account with a unique username and password.
- User login: Provides authentication for users to log in and access protected endpoints.
- Article CRUD operations: Users can create, read, update, and delete articles.
- Secure authentication: Implements secure password hashing for user authentication.

## Installation

1. Ensure you have Rust installed. If not, you can download and install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2. Clone the repository: git clone https://github.com/raj-p26/inklink_backend
3. Navigate to the project directory: cd inklink_backend
4. Build the project: cargo build

## Usage

1. Start the server: cargo run
2. Access the API endpoints using your preferred HTTP client, such as cURL or Postman.

## API Endpoints

- GET /users/: Retrieve all users.
- POST /users/register: Register a new user.
- POST /users/login: Log in with existing credentials.
- PUT /users/update: Update a user's profile.
- GET /users/:id: Retrieve a user by ID.
- GET /users/:id/latest: Retrieve the latest article of a user.
- POST /articles/new: Create a new article.
- GET /articles/:id: Retrieve an article by ID.
- GET /articles/all: Retrieve all articles.
- GET /articles/latest: Retrieve latest articles.
- GET /articles/:user_id/:type: Retrieve articles by user ID and type.
- PUT /articles/update/: Update an existing article.
- DELETE /articles/delete/:id: Delete an article by ID.

## Contributing

Contributions are welcome! If you would like to contribute to Inklink, please open a pull request with your proposed changes.
