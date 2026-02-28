import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

const api = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
});

// Market Data
export const marketApi = {
  getTicker: (base: string, quote: string, exchange: string) =>
    api.post('/api/market/ticker', { base, quote, exchange }),
  
  getCandles: (base: string, quote: string, exchange: string, interval?: string, limit?: number) =>
    api.post('/api/market/candles', { base, quote, exchange, interval, limit }),
};

// Orders
export const orderApi = {
  placeOrder: (order: any) => api.post('/api/orders', order),
  cancelOrder: (orderId: string) => api.delete(`/api/orders/${orderId}`),
  getOrders: () => api.get('/api/orders'),
};

// Portfolio
export const portfolioApi = {
  getPositions: () => api.get('/api/portfolio/positions'),
  getHistory: () => api.get('/api/portfolio/history'),
};

// Strategies
export const strategyApi = {
  list: () => api.get('/api/strategies'),
  enable: (id: string) => api.post(`/api/strategies/${id}/enable`),
  disable: (id: string) => api.post(`/api/strategies/${id}/disable`),
};

export default api;
