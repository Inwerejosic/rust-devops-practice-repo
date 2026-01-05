Member Management System

A full-stack application featuring a high-performance Rust backend and a modern, reactive Vue.js frontend. This system allows for full CRUD (Create, Read, Update, Delete) operations on a member database using SQLite.
🚀 Tech Stack

    Backend: Rust with Actix-web

    Database: SQLite with Diesel ORM

    Frontend: Vue.js 3 (Vite, Composition API)

    Containerization: Docker & Docker Compose

📂 Project Structure
Plaintext

.
├── backend/             # Rust Actix-web API
│   ├── src/             # Application source code
│   ├── db_data/         # Local SQLite database storage (Docker Volume)
│   ├── Dockerfile       # Multi-stage build for Rust
│   └── .dockerignore
├── frontend/            # Vue.js SPA
│   ├── src/             # Vue components and logic
│   ├── Dockerfile       # Nginx-based production build
│   └── .dockerignore
├── docker-compose.yml   # Orchestrates both services
└── .gitignore           # Global git ignore rules

🛠️ Getting Started
Prerequisites

    Docker Desktop installed.

    (Optional for local dev) Rust and Node.js.

Quick Start (Docker)

The easiest way to run the entire stack is using Docker Compose:

    Clone the repository:
    Bash

git clone <your-repo-url>
cd my-app

Launch the Application:
Bash

    docker-compose up --build

    Access the App:

        Frontend: http://localhost (Port 80)

        Backend API: http://localhost:7070

📡 API Endpoints

The backend exposes the following RESTful endpoints:
Method	Endpoint	Description
GET	/members	List all members
GET	/member/{id}	Get specific member details
POST	/member	Create a new member
PUT	/member/{id}	Full update of a member
PATCH	/member/{id}	Partial update (e.g., change age only)
DELETE	/member/delete/{id}	Remove a member
🔧 Manual Development Setup

If you prefer to run the services outside of Docker:
Backend (Rust)

    Navigate to /backend.

    Ensure you have a SQLite database file or run migrations: diesel migration run.

    Start the server:
    Bash

    cargo run

Frontend (Vue)

    Navigate to /frontend.

    Install dependencies:
    Bash

npm install

Start the development server:
Bash

    npm run dev

💾 Database Persistence

When running via Docker, the SQLite database is stored in ./backend/db_data/. This is mapped to a Docker volume, meaning your data will persist even if you stop or remove your containers.

📝 License

This project is open-source and available under the MIT License.
