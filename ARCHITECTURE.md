# Trading System Technical Architecture

**Version:** 1.0  
**Date:** 2026-02-27  
**Status:** Design Proposal  
**Issue:** #11 - Build Trading System Technical Framework

---

## 1. Overview

This document outlines the technical architecture for a trading system supporting stocks and cryptocurrency. The system is designed with extensibility, maintainability, and readability as core principles.

### 1.1 Technology Stack

| Layer | Technology | Rationale |
|-------|------------|-----------|
| Backend | Rust | High performance, memory safety, mature ecosystem |
| Frontend | React | Most popular, large ecosystem, strong community |
| Database | PostgreSQL + Redis | Relational data + caching/pub-sub |
| Real-time | WebSocket | Low-latency market data updates |

### 1.2 Initial Scope

- **Phase 1:** Message collection, strategy development foundation
- **Phase 2:** Trading execution (stocks + crypto)
- **Phase 3:** Portfolio management, analytics

---

## 2. Backend Architecture (Rust)

### 2.1 Web Framework Recommendation

**Recommendation: Axum** (primary), with Actix-web as alternative

| Aspect | Axum | Actix-web |
|--------|------|-----------|
| Learning Curve | Lower | Steeper |
| Async Story | Built on tokio | Built on tokio |
| Extensibility | Middleware via tower | Powerful but complex |
| Maintenance | Active, well-maintained | Very active |
| Performance | Excellent | Excellent |

**Why Axum:**
- Ergonomic API design
- Strong integration with the broader Rust async ecosystem
- Better documentation and developer experience
- Seamless tower middleware integration
- Type-safe request handling with compile-time checks

### 2.2 Database Design

#### PostgreSQL (Primary Data Store)

```
┌─────────────────────────────────────────────────────────────┐
│                      PostgreSQL                             │
├─────────────────────────────────────────────────────────────┤
│  users              │ User accounts & authentication        │
│  strategies         │ Trading strategy definitions         │
│  messages           │ Collected messages/analysis          │
│  market_data        │ Historical market data                │
│  trades             │ Executed trade records                │
│  portfolios         │ Portfolio positions                  │
└─────────────────────────────────────────────────────────────┘
```

#### Redis (Cache + Pub/Sub)

- **Session caching:** Fast user session retrieval
- **Rate limiting:** API request throttling
- **Pub/Sub:** Real-time market data distribution
- **Cache layer:** Frequently accessed market data

### 2.3 Key Crates (Production-Ready)

```toml
# Cargo.toml dependencies
[dependencies]
# Web Framework
axum = "0.7"
tower = "0.4"          # Middleware
tower-http = "0.5"    # CORS, compression, serve static

# Database
sqlx = "0.8"          # Async SQL with compile-time checks
tokio-postgres = "0.7" # Direct PostgreSQL
redis = "0.25"        # Redis client

# Serialization
serde = "1.0"         # Serialization framework
serde_json = "1.0"   # JSON support

# WebSocket
tokio-tungstenite = "0.21" # WebSocket client/server
futures-util = "0.3"       # Async utilities

# Async Runtime
tokio = "1"           # Runtime

# Utilities
thiserror = "1.0"    # Error handling
uuid = "1.0"         # Unique IDs
chrono = "0.4"       # Date/time
tracing = "0.1"      # Structured logging
tracing-subscriber = "0.3"

# Validation
validator = "0.16"   # Request validation

# Testing
mockall = "0.12"
tokio-test = "0.4"
```

### 2.4 Project Structure (Clean Architecture)

```
trading-backend/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library root
│   │
│   ├── api/                 # API Layer (Controllers)
│   │   ├── mod.rs
│   │   ├── routes/
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   ├── messages.rs
│   │   │   ├── strategies.rs
│   │   │   └── websocket.rs
│   │   ├── middleware/
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   └── logging.rs
│   │   └── mod.rs
│   │
│   ├── application/         # Application Layer (Use Cases)
│   │   ├── mod.rs
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── message_service.rs
│   │   │   ├── strategy_service.rs
│   │   │   └── trading_service.rs
│   │   ├── dto/
│   │   │   ├── mod.rs
│   │   │   └── ...
│   │   └── errors.rs
│   │
│   ├── domain/              # Domain Layer (Entities)
│   │   ├── mod.rs
│   │   ├── entities/
│   │   │   ├── mod.rs
│   │   │   ├── user.rs
│   │   │   ├── message.rs
│   │   │   ├── strategy.rs
│   │   │   └── trade.rs
│   │   ├── repositories/
│   │   │   ├── mod.rs
│   │   │   ├── message_repo.rs
│   │   │   └── strategy_repo.rs
│   │   └── value_objects/
│   │       ├── mod.rs
│   │       └── ...
│   │
│   ├── infrastructure/     # Infrastructure Layer
│   │   ├── mod.rs
│   │   ├── database/
│   │   │   ├── mod.rs
│   │   │   ├── postgres.rs
│   │   │   ├── redis.rs
│   │   │   └── migrations/
│   │   ├── websocket/
│   │   │   ├── mod.rs
│   │   │   └── handler.rs
│   │   └── external/
│   │       ├── mod.rs
│   │       └── market_api.rs
│   │
│   └── config/
│       ├── mod.rs
│       └── settings.rs
│
├── tests/
│   ├── integration/
│   └── unit/
│
├── docker/
│   └── Dockerfile
│
├── docker-compose.yml
│
├── .env.example
│
├── Cargo.toml
│
└── README.md
```

### 2.5 API Design

#### REST Endpoints

```
Authentication:
POST   /api/v1/auth/register     # Register new user
POST   /api/v1/auth/login        # Login
POST   /api/v1/auth/logout      # Logout
GET    /api/v1/auth/me          # Current user

Messages:
GET    /api/v1/messages         # List messages (paginated)
POST   /api/v1/messages         # Create message
GET    /api/v1/messages/:id     # Get message by ID
DELETE /api/v1/messages/:id    # Delete message

Strategies:
GET    /api/v1/strategies       # List strategies
POST   /api/v1/strategies       # Create strategy
GET    /api/v1/strategies/:id   # Get strategy
PUT    /api/v1/strategies/:id   # Update strategy
DELETE /api/v1/strategies/:id   # Delete strategy
POST   /api/v1/strategies/:id/backtest  # Run backtest

WebSocket:
WS     /ws/market               # Real-time market data
WS     /ws/strategies/:id       # Strategy signals
```

#### WebSocket Protocol

```json
// Client -> Server (Subscribe)
{
  "type": "subscribe",
  "channel": "market_data",
  "symbols": ["BTC/USD", "ETH/USD"]
}

// Server -> Client (Market Update)
{
  "type": "market_update",
  "symbol": "BTC/USD",
  "price": 45000.00,
  "volume": 125.5,
  "timestamp": "2026-02-27T15:00:00Z"
}

// Server -> Client (Strategy Signal)
{
  "type": "signal",
  "strategy_id": "uuid",
  "action": "BUY",
  "symbol": "BTC/USD",
  "price": 45000.00,
  "confidence": 0.85
}
```

---

## 3. Frontend Architecture (React)

### 3.1 State Management Recommendation

**Recommendation: Zustand** (primary)

| Aspect | Zustand | Redux Toolkit |
|--------|---------|---------------|
| Boilerplate | Minimal | Moderate |
| Learning Curve | Low | Medium |
| Performance | Excellent | Excellent |
| DevTools | Good | Excellent |
| Bundle Size | ~1KB | ~12KB |
| TypeScript | First-class | First-class |

**Why Zustand:**
- Minimal boilerplate - write less code
- Simple API - no actions/reducers complexity
- Built-in persistence middleware
- Excellent TypeScript support
- Suitable for both small and medium apps

### 3.2 UI Component Library

**Recommendation: shadcn/ui + Tailwind CSS**

| Library | Pros | Cons |
|---------|------|------|
| shadcn/ui | Beautiful, accessible, customizable | Requires Tailwind |
| Mantine | All-in-one, great components | Larger bundle |
| MUI | Mature, lots of examples | Heavy, dated default look |
| Chakra UI | Easy to use, accessible | Maintenance concerns |

**Why shadcn/ui:**
- Copy-paste components (not a dependency)
- Built on Radix UI primitives (accessible)
- Tailwind CSS for styling (flexible)
- Beautiful default design
- Actively maintained by Vercel team

### 3.3 API Client Recommendation

**Recommendation: TanStack Query (React Query)**

- Built-in caching and synchronization
- Optimistic updates
- Request deduplication
- Background refetching
- Pagination and infinite scroll support
- TypeScript support

### 3.4 Project Structure

```
trading-frontend/
├── public/
│   └── favicon.ico
│
├── src/
│   ├── main.tsx
│   ├── App.tsx
│   ├── index.css
│   │
│   ├── api/                    # API layer
│   │   ├── client.ts           # Axios/fetch setup
│   │   ├── auth.ts
│   │   ├── messages.ts
│   │   └── strategies.ts
│   │
│   ├── components/             # Shared components
│   │   ├── ui/                 # shadcn/ui components
│   │   │   ├── button.tsx
│   │   │   ├── card.tsx
│   │   │   ├── input.tsx
│   │   │   └── ...
│   │   ├── layout/
│   │   │   ├── Header.tsx
│   │   │   ├── Sidebar.tsx
│   │   │   └── Layout.tsx
│   │   └── common/
│   │       ├── Loading.tsx
│   │       └── ErrorBoundary.tsx
│   │
│   ├── features/               # Feature-based modules
│   │   ├── auth/
│   │   │   ├── components/
│   │   │   ├── LoginPage.tsx
│   │   │   ├── RegisterPage.tsx
│   │   │   └── useAuth.ts
│   │   │
│   │   ├── messages/
│   │   │   ├── components/
│   │   │   ├── MessageList.tsx
│   │   │   ├── MessageForm.tsx
│   │   │   └── hooks/
│   │   │       └── useMessages.ts
│   │   │
│   │   ├── strategies/
│   │   │   ├── components/
│   │   │   ├── StrategyList.tsx
│   │   │   ├── StrategyEditor.tsx
│   │   │   └── hooks/
│   │   │       └── useStrategies.ts
│   │   │
│   │   └── trading/
│   │       ├── components/
│   │       ├── MarketChart.tsx
│   │       └── hooks/
│   │           └── useMarketData.ts
│   │
│   ├── hooks/                  # Shared hooks
│   │   ├── useWebSocket.ts
│   │   └── useLocalStorage.ts
│   │
│   ├── stores/                 # Zustand stores
│   │   ├── authStore.ts
│   │   └── uiStore.ts
│   │
│   ├── lib/                    # Utilities
│   │   ├── utils.ts
│   │   ├── formatters.ts
│   │   └── validators.ts
│   │
│   ├── types/                  # TypeScript types
│   │   ├── api.ts
│   │   ├── auth.ts
│   │   └── ...
│   │
│   └── config/                 # Configuration
│       └── constants.ts
│
├── components.json             # shadcn/ui config
├── tailwind.config.js
├── tsconfig.json
├── vite.config.ts
├── package.json
│
└── README.md
```

### 3.5 Frontend Dependencies

```json
{
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-router-dom": "^6.20.0",
    "@tanstack/react-query": "^5.0.0",
    "zustand": "^4.4.0",
    "axios": "^1.6.0",
    "clsx": "^2.0.0",
    "tailwind-merge": "^2.0.0",
    "class-variance-authority": "^0.7.0",
    "@radix-ui/react-slot": "^1.0.2",
    "@radix-ui/react-dialog": "^1.0.5",
    "@radix-ui/react-dropdown-menu": "^2.0.6",
    "lucide-react": "^0.294.0",
    "recharts": "^2.10.0",
    "date-fns": "^2.30.0",
    "zod": "^3.22.0",
    "react-hook-form": "^7.48.0",
    "@hookform/resolvers": "^3.3.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@vitejs/plugin-react": "^4.2.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0",
    "tailwindcss": "^3.3.0",
    "postcss": "^8.4.0",
    "eslint": "^8.55.0",
    "@typescript-eslint/eslint-plugin": "^6.0.0",
    "prettier": "^3.1.0"
  }
}
```

---

## 4. Infrastructure

### 4.1 Docker Compose Setup

```yaml
# docker-compose.yml
version: '3.8'

services:
  # Backend API
  backend:
    build:
      context: ./trading-backend
      dockerfile: docker/Dockerfile
    ports:
      - "${BACKEND_PORT:-8080}:8080"
    environment:
      - DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@db:5432/${DB_NAME}
      - REDIS_URL=redis://redis:6379
      - RUST_LOG=${RUST_LOG:-info}
    depends_on:
      db:
        condition: service_healthy
      redis:
        condition: service_started
    volumes:
      - ./trading-backend:/app
      - cargo-cache:/app/target
    restart: unless-stopped

  # Frontend
  frontend:
    build:
      context: ./trading-frontend
      dockerfile: Dockerfile
    ports:
      - "${FRONTEND_PORT:-3000}:80"
    depends_on:
      - backend
    volumes:
      - ./trading-frontend:/app
      - /app/node_modules
    environment:
      - VITE_API_URL=http://localhost:8080
    restart: unless-stopped

  # PostgreSQL Database
  db:
    image: postgres:15-alpine
    environment:
      - POSTGRES_USER=${DB_USER}
      - POSTGRES_PASSWORD=${DB_PASSWORD}
      - POSTGRES_DB=${DB_NAME}
    ports:
      - "${DB_PORT:-5432}:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./docker/postgres/init.sql:/docker-entrypoint-initdb.d/init.sql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${DB_USER}"]
      interval: 10s
      timeout: 5s
      retries: 5
    restart: unless-stopped

  # Redis Cache
  redis:
    image: redis:7-alpine
    ports:
      - "${REDIS_PORT:-6379}:6379"
    volumes:
      - redis_data:/data
    command: redis-server --appendonly yes
    restart: unless-stopped

  # Nginx Reverse Proxy (optional)
  nginx:
    image: nginx:alpine
    ports:
      - "${NGINX_PORT:-80}:80"
      - "${NGINX_SSL_PORT:-443}:443"
    volumes:
      - ./docker/nginx/nginx.conf:/etc/nginx/nginx.conf:ro
      - ./docker/nginx/ssl:/etc/nginx/ssl:ro
    depends_on:
      - backend
      - frontend
    restart: unless-stopped

volumes:
  postgres_data:
  redis_data:
  cargo-cache:

networks:
  default:
    name: trading-network
```

### 4.2 Environment Configuration

```bash
# .env.example

# Backend
BACKEND_PORT=8080
DATABASE_URL=postgres://trading:trading123@localhost:5432/trading_db
REDIS_URL=redis://localhost:6379
RUST_LOG=info
JWT_SECRET=your-secret-key-here

# Frontend
VITE_API_URL=http://localhost:8080
VITE_WS_URL=ws://localhost:8080

# Database
DB_USER=trading
DB_PASSWORD=trading123
DB_NAME=trading_db
DB_PORT=5432

# Redis
REDIS_PORT=6379

# Ports
FRONTEND_PORT=3000
NGINX_PORT=80
NGINX_SSL_PORT=443
```

### 4.3 CI/CD Pipeline

```yaml
# .github/workflows/ci.yml
name: CI/CD

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

env:
  CARGO_TERM_COLOR: always

jobs:
  # Backend CI
  backend-ci:
    name: Backend CI
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      
      - name: Cache cargo
        uses: Swatinem/rust-cache@v2
        with:
          workspaces: './trading-backend -> target'
      
      - name: Check formatting
        working-directory: trading-backend
        run: cargo fmt --check
      
      - name: Run clippy
        working-directory: trading-backend
        run: cargo clippy -- -D warnings
      
      - name: Run tests
        working-directory: trading-backend
        run: cargo test --all-features
      
      - name: Build
        working-directory: trading-backend
        run: cargo build --release

  # Frontend CI
  frontend-ci:
    name: Frontend CI
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
          cache-dependency-path: trading-frontend/package-lock.json
      
      - name: Install dependencies
        working-directory: trading-frontend
        run: npm ci
      
      - name: Check formatting
        working-directory: trading-frontend
        run: npm run format:check
      
      - name: Run linter
        working-directory: trading-frontend
        run: npm run lint
      
      - name: Run tests
        working-directory: trading-frontend
        run: npm run test

  # E2E Tests (optional)
  e2e-tests:
    name: E2E Tests
    runs-on: ubuntu-latest
    needs: [backend-ci, frontend-ci]
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Start services
        run: docker-compose up -d
      
      - name: Run Playwright tests
        run: npm run test:e2e
        working-directory: trading-frontend
      
      - name: Stop services
        if: always()
        run: docker-compose down

  # Deploy
  deploy:
    name: Deploy
    runs-on: ubuntu-latest
    needs: e2e-tests
    if: github.ref == 'refs/heads/main'
    
    steps:
      - name: Deploy to server
        run: |
          echo "Deploying to production..."
          # Add deployment commands here
```

---

## 5. Project Structure

### 5.1 Repository Strategy

**Recommendation: Monorepo** (with workspace support)

```
agent_team_work/
├── trading-backend/           # Rust backend
│   ├── Cargo.toml
│   └── src/
│
├── trading-frontend/           # React frontend
│   ├── package.json
│   └── src/
│
├── docker/                     # Docker configurations
│   ├── nginx/
│   └── postgres/
│
├── .github/
│   └── workflows/
│
├── docker-compose.yml          # Full stack
├── docker-compose.override.yml # Local dev
├── .env.example
├── README.md
└── ARCHITECTURE.md
```

**Benefits of Monorepo:**
- Single source of truth
- Easier cross-module refactoring
- Shared tooling and configurations
- Simplified CI/CD (one pipeline)
- Atomic commits across frontend/backend

**Alternative: Separate Repos** (if team prefers)
- `trading-backend` (Rust)
- `trading-frontend` (React)

---

## 6. Development Workflow

### 6.1 Local Development

```bash
# 1. Clone the repository
git clone https://github.com/zghuang/agent_team_work.git
cd agent_team_work

# 2. Copy environment file
cp .env.example .env

# 3. Start development environment
docker-compose up -d

# 4. Run backend (separate terminal)
cd trading-backend
cargo run

# 5. Run frontend (separate terminal)
cd trading-frontend
npm run dev

# 6. Access the application
# Frontend: http://localhost:3000
# Backend API: http://localhost:8080
```

### 6.2 Database Migrations

```bash
# Create migration
cd trading-backend
cargo sqlx migrate add create_messages

# Run migrations
cargo sqlx migrate run

# Revert migration
cargo sqlx migrate revert
```

---

## 7. Security Considerations

### 7.1 Authentication

- JWT-based authentication with refresh tokens
- Secure HTTP-only cookies for token storage
- Rate limiting on auth endpoints
- Password hashing with bcrypt/argon2

### 7.2 API Security

- CORS configuration for allowed origins
- Input validation on all endpoints
- SQL injection prevention via parameterized queries
- XSS protection headers
- CSRF tokens for state-changing operations

### 7.3 Data Protection

- TLS/SSL in production
- Environment variables for secrets
- Database encryption at rest (production)
- Regular security audits

---

## 8. Future Extensibility

### Phase 2: Trading Execution

- Broker API integrations
- Order management system
- Position tracking
- P&L calculations

### Phase 3: Advanced Features

- Real-time charts with WebSocket
- Backtesting engine
- Portfolio analytics
- Mobile applications (React Native)

---

## 9. Summary

This architecture provides:

| Requirement | Solution |
|-------------|----------|
| **Rust Backend** | Axum framework with clean architecture |
| **React Frontend** | Vite + shadcn/ui + Zustand + TanStack Query |
| **Database** | PostgreSQL + Redis |
| **Real-time** | WebSocket with tokio-tungstenite |
| **Extensibility** | Clean architecture with clear layer separation |
| **Maintainability** | Monorepo structure, TypeScript, comprehensive tests |
| **Deployment** | Docker Compose ready, CI/CD configured |

---

*Document created for Issue #11 - Build Trading System Technical Framework*
