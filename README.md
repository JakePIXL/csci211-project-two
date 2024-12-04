# CSCI211 Project 2 - Command Line Trivia Game

A command-line trivia game built with Rust, SQLx, and MariaDB that supports multiple game modes and user roles.

## Overview

This project implements a text-based trivia game system with admin and player roles. Admins can create and manage trivia games and questions, while players can participate in various trivia games through a command-line interface.

## Features

> No pashword hashing not implemented, just raw storage (din't have time to implement it)

### Admin Features
- Manage Questions:
  - Create True/False questions
  - Edit existing questions
  - Remove questions from games

- Manage Games:
  - Create new trivia games
  - Add questions to games
  - Remove questions from games
  - Reorder questions within games

### Player Features
- Select and play different trivia games
- Answer True/False questions sequentially
- View immediate feedback on answers
- See final score and performance summary

## Database Schema

The application uses a MariaDB database with the following structure:

### Admins Table
- `admin_id` (Primary Key)
- `username`
- `password_hash`
- `created_at`

### Players Table
- `player_id` (Primary Key)
- `username`
- `password_hash`
- `created_at`

### Games Table
- `game_id` (Primary Key)
- `title`
- `description`
- `created_by` (Foreign Key to Admins)
- `created_at`

### Questions Table
- `question_id` (Primary Key)
- `question_text`
- `correct_answer`
- `created_by` (Foreign Key to Admins)
- `created_at`

### Game Questions Table
- `game_id` (Foreign Key)
- `question_id` (Foreign Key)
- `question_order`

## ER Diagram

```mermaid
erDiagram
    ADMINS ||--o{ GAMES : creates
    ADMINS ||--o{ QUESTIONS : creates
    GAMES ||--o{ GAME_QUESTIONS : contains
    QUESTIONS ||--o{ GAME_QUESTIONS : includes

    ADMINS {
        int admin_id PK
        string username
        string password_hash
        datetime created_at
    }

    PLAYERS {
        int player_id PK
        string username
        string password_hash
        datetime created_at
    }

    GAMES {
        int game_id PK
        string title
        string description
        int created_by FK
        datetime created_at
    }

    QUESTIONS {
        int question_id PK
        string question_text
        bool correct_answer
        int created_by FK
        datetime created_at
    }

    GAME_QUESTIONS {
        int game_id FK
        int question_id FK
        int question_order
    }
```

## Technologies Used

- Rust
- SQLx (Database Library)
- MariaDB (Database)
- Tokio (Async Runtime)
- Serde (Serialization)
- TOML (Configuration)

## Installation

1. Clone the repository
2. Configure your database connection in `config.toml`:
```toml
[database]
url = "mysql://username:password@localhost/trivia"
max_connections = 5
```

3. Run the SQL schema creation script:
```bash
mysql -u username -p database_name < GameTables.sql
```

4. Build and run the project:
```bash
cargo build
cargo run
```

## Usage

1. Start the application:
```bash
./projecttwo
```

2. Choose between Admin and Player mode

3. Admin Mode:
   - Create and manage questions
   - Create and configure games
   - Edit game content and order

4. Player Mode:
   - Select a game to play
   - Answer questions sequentially
   - View results and feedback
