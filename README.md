# Rust REST API with Local Storage

This is a simple REST API built using the Rust programming language. The API allows users to manage tasks with the ability to retrieve and create tasks. Data is stored in memory using a thread-safe shared state.

---

## Features
- **GET /tasks**: Retrieve a list of all tasks.
- **POST /tasks**: Add a new task to the local storage.
- Local storage implemented using `Arc` and `Mutex` for thread-safe access.

---

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.70+ recommended)
- Cargo (comes with Rust installation)
- A tool to test APIs (e.g., [Postman](https://www.postman.com/) or `curl`)

---

## Setup and Usage

1. **Clone the Repository**
   ```bash
   git clone https://github.com/Usamahafiz8/rust-simple-api 
   cd rust-simple-api
   ```

2. **Install Dependencies**
   Edit the `Cargo.toml` file to include the required dependencies:
   ```toml
   [dependencies]
   axum = "0.6"
   tokio = { version = "1", features = ["full"] }
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   ```
   Then, run:
   ```bash
   cargo build
   ```

3. **Run the Server**
   Start the server using:
   ```bash
   cargo run
   ```
   The server will be available at `http://127.0.0.1:8080`.

4. **Test the Endpoints**
   Use your preferred API client to interact with the endpoints.

---

## API Endpoints

### **1. GET /tasks**
Retrieve a list of all tasks.

- **URL**: `http://127.0.0.1:8080/tasks`
- **Method**: `GET`
- **Response**:
  ```json
  [
      {
          "id": 1,
          "title": "Learn Rust",
          "completed": false
      }
  ]
  ```

---

### **2. POST /tasks**
Create a new task.

- **URL**: `http://127.0.0.1:8080/tasks`
- **Method**: `POST`
- **Body (JSON)**:
  ```json
  {
      "id": 1,
      "title": "Learn Rust",
      "completed": false
  }
  ```
- **Response**:
  ```json
  {
      "id": 1,
      "title": "Learn Rust",
      "completed": false
  }
  ```

---

## Code Overview

### **Key Components**
1. **`AppState`**: Holds the in-memory `Vec<Task>` for local storage.
2. **`Arc` and `Mutex`**: Ensure thread-safe access to shared state across multiple requests.
3. **Axum Framework**: Used to build the REST API.

### **File Structure**
```
├── Cargo.toml   # Project dependencies and metadata
└── src
    └── main.rs  # Main application logic
```

---

## Future Improvements
1. **Persistent Storage**: Add file-based storage or integrate with a database like SQLite or PostgreSQL.
2. **Error Handling**: Implement robust error handling for invalid inputs or server errors.
3. **Additional Features**:
   - Update tasks (`PUT /tasks/:id`)
   - Delete tasks (`DELETE /tasks/:id`)

---

## Contributing
Feel free to fork this repository and submit pull requests. Contributions are welcome!

---

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

