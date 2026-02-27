# Trading System

A full-stack trading system for stocks and cryptocurrency, built with Rust (backend) and React (frontend).

## Architecture

```
trading-system/
├── backend/           # Rust API server
│   ├── src/
│   │   ├── api/      # HTTP handlers
│   │   ├── models/   # Data structures
│   │   ├── services/ # Business logic
│   │   ├── utils/    # Utilities
│   │   └── main.rs   # Entry point
│   └── Cargo.toml
│
└── frontend/         # React SPA
    ├── src/
    │   ├── components/
    │   ├── pages/
    │   ├── App.tsx
    │   └── main.tsx
    └── package.json
```

## Tech Stack

### Backend (Rust)
- **Web Framework**: Actix-web or Axum
- **Async Runtime**: Tokio
- **Database**: SQLite (sqlx)
- **HTTP Client**: Reqwest
- **Serialization**: Serde

### Frontend (React)
- **Framework**: React 18 + TypeScript
- **Build Tool**: Vite
- **State Management**: Zustand
- **Data Fetching**: TanStack Query
- **Charts**: Recharts
- **UI**: Tailwind CSS
- **Icons**: Lucide React

## Getting Started

### Prerequisites
- Rust 1.70+
- Node.js 18+
- npm or yarn

### Backend Setup

```bash
cd backend
cargo build
cargo run
```

Server runs at `http://localhost:8080`

### Frontend Setup

```bash
cd frontend
npm install
npm run dev
```

App runs at `http://localhost:3000`

## Features

- 📊 **Dashboard**: Portfolio overview with performance charts
- 📈 **Markets**: Real-time price data for stocks and crypto
- 🧠 **Strategies**: Automated trading strategy management
- 💱 **Trading**: Place buy/sell orders

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /health | Health check |
| GET | /api/market/:symbol | Get market data |
| GET | /api/strategies | List strategies |
| POST | /api/orders | Create order |

## Development

### Adding a new API endpoint

1. Add handler in `backend/src/api/mod.rs`
2. Register route in `backend/src/main.rs`

### Adding a new page

1. Create component in `frontend/src/pages/`
2. Add route in `frontend/src/App.tsx`

## License

MIT
