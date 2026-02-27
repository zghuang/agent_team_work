# Trading System

A full-stack trading platform with Rust/Axum backend and React/Vite frontend.

## Quick Start

### Using Docker Compose (Recommended)

```bash
# Start all services
docker-compose up --build

# Services will be available at:
# - Frontend: http://localhost:3000
# - Backend API: http://localhost:8080
# - PostgreSQL: localhost:5432
# - Redis: localhost:6379
```

### Development Mode

#### Backend
```bash
cd trading-backend
cargo run
```

#### Frontend
```bash
cd trading-frontend
npm install
npm run dev
```

## Project Structure

```
trading-backend/     # Rust + Axum API server
trading-frontend/    # React + Vite frontend
docker-compose.yml   # Orchestration
```

## API Endpoints

- `GET /` - Root endpoint
- `GET /api/health` - Health check
- `GET /api/orders` - List orders
- `POST /api/orders` - Create order
