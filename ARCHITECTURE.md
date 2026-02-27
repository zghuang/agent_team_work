# Trading System Technical Architecture

**Version:** 1.0 | **Date:** 2026-02-27 | **Issue:** #11

---

## 1. Overview

Trading system for stocks + cryptocurrency with extensibility, maintainability.

### Technology Stack

| Layer | Technology |
|-------|------------|
| Backend | Rust (Axum) |
| Frontend | React (Vite + shadcn/ui) |
| Database | PostgreSQL + Redis |
| Real-time | WebSocket |

---

## 2. Backend (Rust + Axum)

### Key Crates
```toml
axum = "0.7"
sql = "0.8"
tokio = "1"
serde = "1.0"
tokio-tungstenite = "0.21"
```

### Project Structure
```
trading-backend/
├── src/
│   ├── main.rs
│   ├── api/          # Controllers
│   ├── application/  # Services
│   ├── domain/       # Entities
│   ├── infrastructure/ # DB, WS
│   └── config/
├── docker/
└── Cargo.toml
```

### API Endpoints
- `POST /api/v1/auth/register`
- `POST /api/v1/auth/login`
- `GET/POST /api/v1/messages`
- `GET/POST /api/v1/strategies`
- `WS /ws/market`

---

## 3. Frontend (React)

### Stack
- **Framework:** Vite + React 18
- **State:** Zustand
- **UI:** shadcn/ui + Tailwind CSS
- **API:** TanStack Query

### Structure
```
trading-frontend/
├── src/
│   ├── api/
│   ├── components/ui/
│   ├── features/
│   │   ├── auth/
│   │   ├── messages/
│   │   └── strategies/
│   ├── stores/
│   └── hooks/
└── package.json
```

---

## 4. Infrastructure

### Docker Compose
- backend (Rust)
- frontend (React)
- PostgreSQL
- Redis
- Nginx

---

## 5. Summary

| Requirement | Solution |
|-------------|----------|
| Rust Backend | Axum + Clean Architecture |
| React Frontend | Vite + shadcn/ui + Zustand |
| Database | PostgreSQL + Redis |
| Real-time | WebSocket |

---

*Issue #11 - Trading System Framework*
