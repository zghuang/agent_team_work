import { useState, useEffect } from 'react';
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  AreaChart,
  Area,
} from 'recharts';
import { marketApi } from '../services/api';

interface Candle {
  symbol: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
  timestamp: string;
}

interface ChartProps {
  symbol?: string;
  interval?: string;
}

export default function PriceChart({ symbol = 'BTC/USDT', interval = '1h' }: ChartProps) {
  const [candles, setCandles] = useState<Candle[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedSymbol, setSelectedSymbol] = useState(symbol);
  const [timeframe, setTimeframe] = useState(interval);

  const symbols = ['BTC/USDT', 'ETH/USDT', 'SOL/USDT', 'BNB/USDT', 'XRP/USDT'];
  const timeframes = ['1m', '5m', '15m', '1h', '4h', '1d'];

  useEffect(() => {
    const fetchCandles = async () => {
      try {
        setLoading(true);
        const [base, quote] = selectedSymbol.split('/');
        const res = await marketApi.getCandles(base, quote, 'binance', timeframe, 100);
        
        if (res.data.success && res.data.data) {
          setCandles(res.data.data);
          setError(null);
        }
      } catch (err: any) {
        console.error('Error fetching candles:', err);
        setError(err.message);
        // Generate mock data for demo
        const mockCandles = generateMockCandles(selectedSymbol, 100);
        setCandles(mockCandles);
      } finally {
        setLoading(false);
      }
    };

    fetchCandles();
  }, [selectedSymbol, timeframe]);

  const generateMockCandles = (sym: string, count: number): Candle[] => {
    const basePrice = sym.startsWith('BTC') ? 50000 : 
                      sym.startsWith('ETH') ? 3000 : 
                      sym.startsWith('SOL') ? 100 : 
                      sym.startsWith('BNB') ? 350 : 0.6;
    
    return Array.from({ length: count }, (_, i) => {
      const change = (Math.random() - 0.5) * basePrice * 0.02;
      const close = basePrice + change;
      const open = close - (Math.random() - 0.5) * basePrice * 0.01;
      const high = Math.max(open, close) + Math.random() * basePrice * 0.005;
      const low = Math.min(open, close) - Math.random() * basePrice * 0.005;
      
      return {
        symbol: sym,
        open,
        high,
        low,
        close,
        volume: Math.random() * 1000,
        timestamp: new Date(Date.now() - (count - i) * 3600000).toISOString(),
      };
    });
  };

  const chartData = candles.map(c => ({
    time: new Date(c.timestamp).toLocaleTimeString('en-US', { 
      hour: '2-digit', 
      minute: '2-digit' 
    }),
    price: c.close,
    open: c.open,
    high: c.high,
    low: c.low,
    volume: c.volume,
  }));

  const formatPrice = (value: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USDT',
      minimumFractionDigits: 2,
    }).format(value);
  };

  const CustomTooltip = ({ active, payload }: any) => {
    if (active && payload && payload.length) {
      const data = payload[0].payload;
      return (
        <div className="bg-white p-3 border shadow-lg rounded">
          <p className="text-sm text-gray-500">{data.time}</p>
          <p className="font-semibold">Close: {formatPrice(data.price)}</p>
          <p className="text-sm">Open: {formatPrice(data.open)}</p>
          <p className="text-sm">High: {formatPrice(data.high)}</p>
          <p className="text-sm">Low: {formatPrice(data.low)}</p>
          <p className="text-sm text-gray-500">Vol: {data.volume.toFixed(2)}</p>
        </div>
      );
    }
    return null;
  };

  return (
    <div className="space-y-4">
      {/* Controls */}
      <div className="flex flex-wrap gap-4 items-center">
        <div>
          <label className="block text-sm text-gray-500 mb-1">Symbol</label>
          <select
            value={selectedSymbol}
            onChange={(e) => setSelectedSymbol(e.target.value)}
            className="border rounded px-3 py-2"
          >
            {symbols.map(s => (
              <option key={s} value={s}>{s}</option>
            ))}
          </select>
        </div>
        <div>
          <label className="block text-sm text-gray-500 mb-1">Timeframe</label>
          <select
            value={timeframe}
            onChange={(e) => setTimeframe(e.target.value)}
            className="border rounded px-3 py-2"
          >
            {timeframes.map(t => (
              <option key={t} value={t}>{t}</option>
            ))}
          </select>
        </div>
      </div>

      {/* Chart */}
      <div className="bg-white p-4 rounded-lg shadow">
        <h3 className="text-lg font-semibold mb-4">{selectedSymbol} Price Chart</h3>
        
        {loading ? (
          <div className="h-80 flex items-center justify-center">
            <p className="text-gray-500">Loading chart...</p>
          </div>
        ) : error ? (
          <div className="h-80 flex items-center justify-center">
            <p className="text-red-500">{error}</p>
          </div>
        ) : (
          <ResponsiveContainer width="100%" height={400}>
            <AreaChart data={chartData}>
              <defs>
                <linearGradient id="colorPrice" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#3B82F6" stopOpacity={0.3} />
                  <stop offset="95%" stopColor="#3B82F6" stopOpacity={0} />
                </linearGradient>
              </defs>
              <CartesianGrid strokeDasharray="3 3" stroke="#E5E7EB" />
              <XAxis 
                dataKey="time" 
                tick={{ fontSize: 12 }}
                interval="preserveStartEnd"
              />
              <YAxis 
                domain={['auto', 'auto']}
                tick={{ fontSize: 12 }}
                tickFormatter={(value) => formatPrice(value)}
              />
              <Tooltip content={<CustomTooltip />} />
              <Area
                type="monotone"
                dataKey="price"
                stroke="#3B82F6"
                strokeWidth={2}
                fillOpacity={1}
                fill="url(#colorPrice)"
              />
            </AreaChart>
          </ResponsiveContainer>
        )}
      </div>

      {/* Stats */}
      {candles.length > 0 && (
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div className="bg-white p-4 rounded-lg shadow">
            <p className="text-sm text-gray-500">Current</p>
            <p className="text-xl font-bold">{formatPrice(candles[candles.length - 1]?.close || 0)}</p>
          </div>
          <div className="bg-white p-4 rounded-lg shadow">
            <p className="text-sm text-gray-500">High</p>
            <p className="text-xl font-bold text-green-600">
              {formatPrice(Math.max(...candles.map(c => c.high)))}
            </p>
          </div>
          <div className="bg-white p-4 rounded-lg shadow">
            <p className="text-sm text-gray-500">Low</p>
            <p className="text-xl font-bold text-red-600">
              {formatPrice(Math.min(...candles.map(c => c.low)))}
            </p>
          </div>
          <div className="bg-white p-4 rounded-lg shadow">
            <p className="text-sm text-gray-500">Volume</p>
            <p className="text-xl font-bold">
              {candles.reduce((sum, c) => sum + c.volume, 0).toFixed(2)}
            </p>
          </div>
        </div>
      )}
    </div>
  );
}
