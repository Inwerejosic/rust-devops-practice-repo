
---

# 🚀 Member Management System

![Rust](https://img.shields.io/badge/backend-Rust-orange)
![Actix](https://img.shields.io/badge/framework-Actix--Web-blue)
![Vue](https://img.shields.io/badge/frontend-Vue%203-42b883)
![Docker](https://img.shields.io/badge/containerized-Docker-2496ED)
![License](https://img.shields.io/badge/license-MIT-green)
![Status](https://img.shields.io/badge/status-Production%20Ready-success)

A **modern, full-stack Member Management System** designed with performance, clarity, and scalability in mind.

The backend is built with **Rust + Actix-web** for speed and safety, while the frontend delivers a smooth, reactive experience using **Vue 3**. Everything is fully containerized with Docker, making local development and deployment effortless.

---

## ✨ Features

* ⚡ **High-performance Rust backend** (Actix-web)
* 🔐 Type-safe database access using **Diesel ORM**
* 🧩 Clean RESTful API design
* 🖥️ Modern Vue 3 frontend (Vite-powered)
* 🐳 Fully Dockerized (multi-stage builds)
* 💾 Persistent SQLite storage
* 🚀 Ready for local development or production use

---

## 🧠 Tech Stack

### Backend

* Rust
* Actix-web
* Diesel ORM
* SQLite

### Frontend

* Vue.js 3
* Vite
* Composition API

### Infrastructure

* Docker
* Docker Compose

---

## 📁 Project Structure

```
.
├── backend/                 # Rust Actix-web API
│   ├── src/                 # Application source code
│   ├── db_data/             # SQLite database storage (Docker volume)
│   ├── Dockerfile           # Multi-stage Rust build
│   └── .dockerignore
│
├── frontend/                # Vue.js Single Page Application
│   ├── src/                 # Vue components and logic
│   ├── Dockerfile           # Nginx-based production build
│   └── .dockerignore
│
├── docker-compose.yml   
├──  README.md
└── .gitignore               # Global Git ignore rules
```

---

## 🚀 Quick Start (Recommended)

The fastest way to get everything running is via **Docker Compose**.

### Prerequisites

* Docker Desktop installed

### Start the application

```bash
docker-compose up --build
```

### Access the services

* 🌐 Frontend: **[http://localhost](http://localhost)**
* 🔌 Backend API: **[http://localhost:7070](http://localhost:7070)**

---

## 📡 API Overview

The backend exposes a clean REST API for managing members:

| Method | Endpoint              | Description         |
| ------ | --------------------- | ------------------- |
| GET    | `/members`            | List all members    |
| GET    | `/member/{id}`        | Retrieve a member   |
| POST   | `/member`             | Create a new member |
| PUT    | `/member/{id}`        | Replace a member    |
| PATCH  | `/member/{id}`        | Update a member     |
| DELETE | `/member/delete/{id}` | Delete a member     |

---

## 🛠️ Manual Development (Without Docker)

### Backend

```bash
cd backend
diesel migration run
cargo run
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

---

## 💾 Data Persistence

When running with Docker, the SQLite database is persisted in:

```
backend/db_data/
```

This ensures data survives container restarts and rebuilds.

---

## 📦 Production Ready

* Multi-stage Docker builds for minimal image size
* Stateless frontend served via Nginx
* Persistent database volumes
* Clean separation of concerns

Perfect for:

* Portfolio projects
* Internal tools
* Small-to-medium production workloads
* Learning modern Rust + Vue architectures

---

## 📄 License

Licensed under the **MIT License** — free to use, modify, and distribute.

---

### ⭐ Like this project?

If this helped or inspired you, consider starring the repository — it helps more than you think.

---