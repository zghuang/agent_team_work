import { useState } from 'react';

export default function Strategies() {
  const [strategies] = useState([
    { id: 1, name: 'MA Crossover', description: 'Moving Average Crossover Strategy', type: 'crypto', enabled: true, pnl: 1250.50, winRate: 68 },
    { id: 2, name: 'RSI Reversal', description: 'RSI Overbought/Oversold Strategy', type: 'crypto', enabled: true, pnl: -230.00, winRate: 45 },
    { id: 3, name: 'Bollinger Bands', description: 'Bollinger Bands Breakout', type: 'stock', enabled: false, pnl: 890.25, winRate: 72 },
    { id: 4, name: 'MACD Divergence', description: 'MACD Histogram Divergence', type: 'crypto', enabled: true, pnl: 567.80, winRate: 55 },
    { id: 5, name: 'Volume Spike', description: 'High Volume Breakout', type: 'stock', enabled: true, pnl: -120.50, winRate: 38 },
  ]);

  const toggleStrategy = (id: number) => {
    console.log('Toggle strategy:', id);
  };

  return (
    <div className="strategies-page">
      <div className="page-header">
        <h1>Trading Strategies</h1>
        <button className="add-btn">+ New Strategy</button>
      </div>

      <div className="strategies-grid">
        {strategies.map(strategy => (
          <div key={strategy.id} className={`strategy-card ${strategy.enabled ? 'active' : 'inactive'}`}>
            <div className="strategy-header">
              <h3>{strategy.name}</h3>
              <label className="switch">
                <input 
                  type="checkbox" 
                  checked={strategy.enabled}
                  onChange={() => toggleStrategy(strategy.id)}
                />
                <span className="slider"></span>
              </label>
            </div>
            <p className="description">{strategy.description}</p>
            <div className="strategy-meta">
              <span className={`type ${strategy.type}`}>{strategy.type}</span>
            </div>
            <div className="strategy-stats">
              <div className="stat">
                <span className="label">Total P&L</span>
                <span className={`value ${strategy.pnl >= 0 ? 'positive' : 'negative'}`}>
                  {strategy.pnl >= 0 ? '+' : ''}${strategy.pnl.toFixed(2)}
                </span>
              </div>
              <div className="stat">
                <span className="label">Win Rate</span>
                <span className="value">{strategy.winRate}%</span>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
