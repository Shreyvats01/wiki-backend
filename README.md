# Wiki Backend 🚀

**Wiki** is a community-driven learning platform designed to connect personal learning journeys with public accountability. Developed by [@Shreyvats01](https://github.com/Shreyvats01), Wiki allows learners to join dedicated communities, track their daily progress, and collaborate in real-time.

---

## 🌟 Key Features

### 👤 User Management & Auth
- **Secure Authentication:** Robust signup, signin, and logout flows using JWT and password hashing.
- **Public/Private Profiles:** Customizable user visibility settings to control how your learning journey is shared.
- **User Discovery:** Search and view profiles by username.

### 🏘️ Communities (Rooms)
- **Focused Learning:** Join specific rooms (e.g., "Learn TypeScript", "Rust 101") to learn alongside others.
- **Real-time Chat:** Interactive group communication powered by **WebSockets** for instant feedback and support.
- **Membership Management:** Seamless join/leave functionality with membership state tracking.

### 📈 Progress Tracking & Missions
- **Daily Missions:** Automated or community-driven daily learning tasks (Todos).
- **Progress Monitoring:** Track daily completion rates and historical progress to maintain learning streaks.
- **Task Management:** Flexible task handling with support for tags and categories.

### 🛠️ Core Tech Stack
- **Language:** [Rust](https://www.rust-lang.org/)
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum) (Tokio ecosystem)
- **Database:** [PostgreSQL](https://www.postgresql.org/) with [SQLx](https://github.com/launchbadge/sqlx) for type-safe async queries.
- **Real-time:** WebSockets for live community interaction.
- **Security:** JWT (JSON Web Tokens) and secure password hashing.

---

## 🚀 Future Roadmap
- 🔗 **External Integrations:** Connect with GitHub and LeetCode for automated progress tracking.
- 📝 **Assignments:** Submit community-specific assignments for peer review.
- 🎖️ **Gamification:** Rewards and badges for consistent learning and contributions.

---

## 🛠️ Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (Edition 2024)
- [PostgreSQL](https://www.postgresql.org/download/)
- [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (for migrations)

### Setup
1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shreyvats01/wiki-backend.git
   cd wiki-backend
   ```

2. **Configure Environment Variables:**
   Create a `.env` file in the root directory:
   ```env
   DATABASE_URL=postgres://username:password@localhost:5432/wiki_db
   JWT_SECRET=your_super_secret_key
   ```

3. **Database Migration:**
   ```bash
   sqlx database create
   sqlx migrate run
   ```

4. **Run the Application:**
   ```bash
   cargo run
   ```
   The server will start at `http://0.0.0.0:3000`.

---

## 📂 Project Structure
- `src/modules/`: Domain-driven logic (User, Todo, Progress, Rooms).
- `src/routes/`: API endpoint definitions and routing.
- `src/middleware/`: Authentication and request processing logic.
- `src/utils/`: Shared utilities for DB, JWT, and configuration.
- `migrations/`: SQL migration files for database schema.

---

## 🤝 Contributing
Wiki is an evolving project. If you're interested in connecting learning with the public, feel free to open issues or submit pull requests.

Created with ❤️ by [Shreyvats01](https://github.com/Shreyvats01)
