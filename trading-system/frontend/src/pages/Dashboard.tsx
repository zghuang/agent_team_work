import { useState, useEffect } from 'react';
import Layout from '../components/Layout';
import { portfolioApi, orderApi, marketApi } from '../services/api';

interface Position {
  symbol: string;
  quantity: number;
  avg_price: number;
  current_price: number;
  pnl: number;
  pnl_percentage: number;
}

interface Portfolio {
  total_value: number;
  total_pnl: number;
  total_pnl_percentage: number;
  positions: Position[];
}

interface Order {
  id: string;
  symbol: string;
  side: string;
  price: number | null;
  quantity: number;
  status: string;
  created_at: string;
}

interface Ticker {
  symbol: string;
  price: number;
}

export default function Dashboard() {
  const [portfolio, setPortfolio] = useState<Portfolio | null>(null);
  const [orders, setOrders] = useState<Order[]>([]);
  const [tickers, setTickers] = useState<Ticker[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);
        
        // Fetch portfolio data
        const portfolioRes = await portfolioApi.get();
        setPortfolio(portfolioRes.data);
        
        // Fetch orders
        const ordersRes = await orderApi.list();
        setOrders(ordersRes.data);
        
        // Fetch market tickers
        const tickerRes = await marketApi.getTicker();
        setTickers(tickerRes.data.data || []);
        
        setError(null);
      } catch (err: any) {
        console.error('Error fetching dashboard data:', err);
        setError(err.message || 'Failed to load data');
      } finally {
        setLoading(false);
      }
    };

    fetchData();
    
    // Refresh every 30 seconds
    const interval = setInterval(fetchData, 30000);
    return () => clearInterval(interval);
  }, []);

  const formatPrice = (price: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USDT',
      minimumFractionDigits: 2,
    }).format(price);
  };

  const formatPercent = (value: number) => {
    const sign = value >= 0 ? '+' : '';
    return `${sign}${value.toFixed(2)}%`;
  };

  const formatTime = (dateStr: string) => {
    const date = new Date(dateStr);
    return date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
  };

  if (loading && !portfolio) {
    return (
      <Layout>
        <div className="flex items-center justify-center h-64">
          <p className="text-gray-500">Loading...</p>
        </div>
      </Layout>
    );
  }

  return (
    <Layout>
      <div className="space-y-6">
        <h1 className="text-2xl font-bold">Dashboard</h1>
        
        {error && (
          <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
            {error}
          </div>
        )}
        
        {/* Portfolio Summary */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div className="bg-white p-6 rounded-lg shadow">
            <p className="text-sm text-gray-500">Total Value</p>
            <p className="text-2xl font-bold">
              {portfolio ? formatPrice(portfolio.total_value) : '$0.00'}
            </p>
          </div>
          <div className="bg-white p-6 rounded-lg shadow">
            <p className="text-sm text-gray-500">Total P&L</p>
            <p className={`text-2xl font-bold ${portfolio && portfolio.total_pnl >= 0 ? 'text-green-600' : 'text-red-600'}`}>
              {portfolio ? formatPrice(portfolio.total_pnl) : '$0.00'}
            </p>
          </div>
          <div className="bg-white p-6 rounded-lg shadow">
            <p className="text-sm text-gray-500">P&L %</p>
            <p className={`text-2xl font-bold ${portfolio && portfolio.total_pnl_percentage >= 0 ? 'text-green-600' : 'text-red-600'}`}>
              {portfolio ? formatPercent(portfolio.total_pnl_percentage) : '0%'}
            </p>
          </div>
          <div className="bg-white p-6 rounded-lg shadow">
            <p className="text-sm text-gray-500">Open Positions</p>
            <p className="text-2xl font-bold">
              {portfolio?.positions.length || 0}
            </p>
          </div>
        </div>

        {/* Market Tickers */}
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-lg font-semibold mb-4">Market Prices</h2>
          <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
            {tickers.map((ticker) => (
              <div key={ticker.symbol} className="p-3 bg-gray-50 rounded">
                <p className="font-semibold">{ticker.symbol}</p>
                <p className="text-lg">{formatPrice(ticker.price)}</p>
              </div>
            ))}
          </div>
        </div>

        {/* Positions */}
        {portfolio && portfolio.positions.length > 0 && (
          <div className="bg-white p-6 rounded-lg shadow">
            <h2 className="text-lg font-semibold mb-4">Positions</h2>
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead>
                  <tr className="border-b">
                    <th className="text-left py-2">Symbol</th>
                    <th className="text-right py-2">Quantity</th>
                    <th className="text-right py-2">Avg Price</th>
                    <th className="text-right py-2">Current</th>
                    <th className="text-right py-2">P&L</th>
                    <th className="text-right py-2">P&L %</th>
                  </tr>
                </thead>
                <tbody>
                  {portfolio.positions.map((pos, idx) => (
                    <tr key={idx} className="border-b">
                      <td className="py-2 font-medium">{pos.symbol}</td>
                      <td className="text-right">{pos.quantity}</td>
                      <td className="text-right">{formatPrice(pos.avg_price)}</td>
                      <td className="text-right">{formatPrice(pos.current_price)}</td>
                      <td className={`text-right ${pos.pnl >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                        {formatPrice(pos.pnl)}
                      </td>
                      <td className={`text-right ${pos.pnl_percentage >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                        {formatPercent(pos.pnl_percentage)}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        )}

        {/* Recent Orders */}
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-lg font-semibold mb-4">Recent Orders</h2>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="text-left py-2">Time</th>
                  <th className="text-left py-2">Symbol</th>
                  <th className="text-left py-2">Side</th>
                  <th className="text-right py-2">Price</th>
                  <th className="text-right py-2">Qty</th>
                  <th className="text-left py-2">Status</th>
                </tr>
              </thead>
              <tbody>
                {orders.length > 0 ? (
                  orders.slice(0, 10).map((order) => (
                    <tr key={order.id} className="border-b">
                      <td className="py-2">{formatTime(order.created_at)}</td>
                      <td className="py-2">{order.symbol}</td>
                      <td className={`py-2 ${order.side === 'buy' ? 'text-green-600' : 'text-red-600'}`}>
                        {order.side.toUpperCase()}
                      </td>
                      <td className="text-right">{order.price ? formatPrice(order.price) : 'Market'}</td>
                      <td className="text-right">{order.quantity}</td>
                      <td className="py-2">
                        <span className={`px-2 py-1 rounded text-xs ${
                          order.status === 'filled' ? 'bg-green-100 text-green-800' :
                          order.status === 'open' ? 'bg-yellow-100 text-yellow-800' :
                          'bg-gray-100 text-gray-800'
                        }`}>
                          {order.status}
                        </span>
                      </td>
                    </tr>
                  ))
                ) : (
                  <tr>
                    <td colSpan={6} className="py-4 text-center text-gray-500">
                      No orders yet
                    </td>
                  </tr>
                )}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </Layout>
  );
}
