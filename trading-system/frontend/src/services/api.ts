import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

const api = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
});

// Market Data
export const marketApi = {
  getTicker: (symbol?: string) => 
    api.get('/api/market/ticker', { params: { symbol } }),
  getTickerPost: (base: string, quote: string, exchange: string) =>
    api.post('/api/market/ticker', { base, quote, exchange }),
  getCandles: (base: string, quote: string, exchange: string, interval?: string, limit?: number) =>
    api.post('/api/market/candles', { base, quote, exchange, interval, limit }),
};

// Orders
export const orderApi = {
  list: (filter?: { symbol?: string; status?: string; side?: string }) =>
    api.get('/api/orders', { params: filter }),
  get: (id: string) => api.get(`/api/orders/${id}`),
  create: (order: any) => api.post('/api/orders', order),
};

// Portfolio
export const portfolioApi = {
  get: () => api.get('/api/portfolio'),
};

// Strategies
export const strategyApi = {
  list: () => api.get('/api/strategies'),
  enable: (id: string) => api.post(`/api/strategies/${id}/enable`),
  disable: (id: string) => api.post(`/api/strategies/${id}/disable`),
};

export default api;
