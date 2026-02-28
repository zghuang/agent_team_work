import { useEffect, useState } from 'react';
import TradingChart from '../components/TradingChart';
import useCryptoPrices from '../hooks/useCryptoPrices';

interface Message {
  id: number;
  content: string;
  message_type: string;
  source: string;
  created_at: string;
}

interface PortfolioItem {
  symbol: string;
  amount: number;
  value: number;
  change: number;
}

interface WatchlistItem {
  symbol: string;
  price: number;
  change: number;
}

export default function Dashboard() {
  const { prices } = useCryptoPrices();
  const [messages, setMessages] = useState<Message[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedSymbol, setSelectedSymbol] = useState('BTC');

  const portfolio: PortfolioItem[] = [
    { symbol: 'BTC', amount: 0.5, value: 22500, change: 2.5 },
    { symbol: 'ETH', amount: 5, value: 15000, change: -1.2 },
    { symbol: 'SOL', amount: 100, value: 8000, change: 5.8 },
  ];

  const watchlist = prices.length > 0 ? prices.map(p => ({
    symbol: p.symbol.replace('USDT', ''),
    price: p.price,
    change: p.change_24h
  })) : [
    { symbol: 'BTC', price: 45000, change: 2.5 },
    { symbol: 'ETH', price: 3000, change: -1.2 },
    { symbol: 'BNB', price: 350, change: 0.8 },
    { symbol: 'SOL', price: 80, change: 5.8 },
    { symbol: 'XRP', price: 0.6, change: -0.5 },
  ];

  useEffect(() => {
    const fetchMessages = async () => {
      try {
        const response = await fetch('/api/v1/messages/fetch');
        if (response.ok) {
          const data = await response.json();
          setMessages(data);
        }
      } catch (error) {
        console.error('Failed to fetch messages:', error);
        setMessages([
          { id: 1, content: 'BTC price update: $50,000', message_type: 'crypto_price', source: 'demo', created_at: new Date().toISOString() },
          { id: 2, content: 'ETH price update: $3,000', message_type: 'crypto_price', source: 'demo', created_at: new Date().toISOString() },
        ]);
      } finally {
        setLoading(false);
      }
    };
    fetchMessages();
  }, []);

  const totalValue = portfolio.reduce((sum, item) => sum + item.value, 0);

  return (
    <div className="dashboard">
      <div className="dashboard-header">
        <h1>Trading Dashboard</h1>
        <div className="total-value">
          <span className="label">Total Portfolio Value</span>
          <span className="value">${totalValue.toLocaleString()}</span>
        </div>
      </div>
      <div className="dashboard-grid">
        <div className="chart-section">
          <div className="symbol-selector">
            <button className={selectedSymbol === 'BTC' ? 'active' : ''} onClick={() => setSelectedSymbol('BTC')}>BTC</button>
            <button className={selectedSymbol === 'ETH' ? 'active' : ''} onClick={() => setSelectedSymbol('ETH')}>ETH</button>
            <button className={selectedSymbol === 'SOL' ? 'active' : ''} onClick={() => setSelectedSymbol('SOL')}>SOL</button>
          </div>
          <TradingChart symbol={selectedSymbol} />
        </div>
        <div className="portfolio-section">
          <h2>Portfolio</h2>
          <div className="portfolio-list">
            {portfolio.map((item) => (
              <div key={item.symbol} className="portfolio-item">
                <div className="symbol">{item.symbol}</div>
                <div className="details">
                  <span className="amount">{item.amount}</span>
                  <span className="value">${item.value.toLocaleString()}</span>
                  <span className={`change ${item.change >= 0 ? 'positive' : 'negative'}`}>
                    {item.change >= 0 ? '+' : ''}{item.change}%
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
        <div className="watchlist-section">
          <h2>Live Prices</h2>
          <div className="watchlist">
            {watchlist.map((item) => (
              <div key={item.symbol} className="watchlist-item">
                <span className="symbol">{item.symbol}</span>
                <span className="price">${item.price.toLocaleString()}</span>
                <span className={`change ${item.change >= 0 ? 'positive' : 'negative'}`}>
                  {item.change >= 0 ? '+' : ''}{item.change.toFixed(2)}%
                </span>
              </div>
            ))}
          </div>
        </div>
        <div className="messages-section">
          <h2>Market Messages</h2>
          {loading ? <div className="loading">Loading messages...</div> : (
            <div className="messages-list">
              {messages.length === 0 ? <div className="no-messages">No messages</div> :
                messages.map((msg) => (
                  <div key={msg.id} className="message-item">
                    <div className="message-content">{msg.content}</div>
                    <div className="message-meta">
                      <span className="message-type">{msg.message_type}</span>
                      <span className="message-source">{msg.source}</span>
                      <span className="message-time">{new Date(msg.created_at).toLocaleTimeString()}</span>
                    </div>
                  </div>
                ))}
            </div>
          )}
          <button className="refresh-btn" onClick={() => window.location.reload()}>Refresh Data</button>
        </div>
      </div>
    </div>
  );
}
