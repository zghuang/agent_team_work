import TradingChart from '../components/TradingChart';

export default function Dashboard() {
  const portfolio = [
    { symbol: 'BTC', amount: 0.5, value: 22500, change: 2.5 },
    { symbol: 'ETH', amount: 5, value: 15000, change: -1.2 },
    { symbol: 'SOL', amount: 100, value: 8000, change: 5.8 },
  ];

  const watchlist = [
    { symbol: 'BTC', price: 45000, change: 2.5 },
    { symbol: 'ETH', price: 3000, change: -1.2 },
    { symbol: 'BNB', price: 350, change: 0.8 },
    { symbol: 'SOL', price: 80, change: 5.8 },
    { symbol: 'XRP', price: 0.6, change: -0.5 },
  ];

  return (
    <div className="dashboard">
      <div className="dashboard-header">
        <h1>Trading Dashboard</h1>
        <div className="total-value">
          <span className="label">Total Portfolio Value</span>
          <span className="value">$45,500.00</span>
        </div>
      </div>

      <div className="dashboard-grid">
        <div className="chart-section">
          <TradingChart symbol="BTC" />
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
          <h2>Watchlist</h2>
          <div className="watchlist">
            {watchlist.map((item) => (
              <div key={item.symbol} className="watchlist-item">
                <span className="symbol">{item.symbol}</span>
                <span className="price">${item.price.toLocaleString()}</span>
                <span className={`change ${item.change >= 0 ? 'positive' : 'negative'}`}>
                  {item.change >= 0 ? '+' : ''}{item.change}%
                </span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
