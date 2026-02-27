import { useState } from 'react';

export default function Markets() {
  const [filter, setFilter] = useState<'all' | 'crypto' | 'stock'>('all');

  const markets = [
    { symbol: 'BTC', name: 'Bitcoin', price: 67500, change: 2.34, volume: '28.5B', type: 'crypto' },
    { symbol: 'ETH', name: 'Ethereum', price: 3450, change: 1.89, volume: '15.2B', type: 'crypto' },
    { symbol: 'BNB', name: 'BNB', price: 605, change: -0.45, volume: '1.2B', type: 'crypto' },
    { symbol: 'SOL', name: 'Solana', price: 145, change: 5.67, volume: '2.8B', type: 'crypto' },
    { symbol: 'XRP', name: 'Ripple', price: 0.52, change: -1.23, volume: '1.5B', type: 'crypto' },
    { symbol: 'AAPL', name: 'Apple Inc.', price: 189.45, change: 0.78, volume: '45M', type: 'stock' },
    { symbol: 'GOOGL', name: 'Alphabet Inc.', price: 175.32, change: 1.12, volume: '22M', type: 'stock' },
    { symbol: 'MSFT', name: 'Microsoft', price: 425.67, change: -0.34, volume: '18M', type: 'stock' },
    { symbol: 'AMZN', name: 'Amazon', price: 178.89, change: 2.45, volume: '35M', type: 'stock' },
    { symbol: 'TSLA', name: 'Tesla', price: 245.67, change: -2.12, volume: '95M', type: 'stock' },
  ];

  const filtered = filter === 'all' ? markets : markets.filter(m => m.type === filter);

  return (
    <div className="markets-page">
      <div className="page-header">
        <h1>Markets</h1>
        <div className="filters">
          <button className={filter === 'all' ? 'active' : ''} onClick={() => setFilter('all')}>All</button>
          <button className={filter === 'crypto' ? 'active' : ''} onClick={() => setFilter('crypto')}>Crypto</button>
          <button className={filter === 'stock' ? 'active' : ''} onClick={() => setFilter('stock')}>Stocks</button>
        </div>
      </div>

      <div className="markets-table">
        <div className="table-header">
          <span>Symbol</span>
          <span>Name</span>
          <span>Price</span>
          <span>24h Change</span>
          <span>Volume</span>
        </div>
        {filtered.map(market => (
          <div key={market.symbol} className="table-row">
            <span className="symbol">
              {market.symbol}
              <span className={`type ${market.type}`}>{market.type}</span>
            </span>
            <span className="name">{market.name}</span>
            <span className="price">${market.price.toLocaleString()}</span>
            <span className={`change ${market.change >= 0 ? 'positive' : 'negative'}`}>
              {market.change >= 0 ? '+' : ''}{market.change}%
            </span>
            <span className="volume">{market.volume}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
