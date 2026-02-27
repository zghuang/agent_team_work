import { useState } from 'react';

export default function Trading() {
  const [symbol, setSymbol] = useState('BTC');
  const [side, setSide] = useState<'buy' | 'sell'>('buy');
  const [orderType, setOrderType] = useState<'market' | 'limit'>('market');
  const [quantity, setQuantity] = useState('');
  const [price, setPrice] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    console.log('Order:', { symbol, side, orderType, quantity, price });
  };

  const recentOrders = [
    { id: 1, symbol: 'BTC', side: 'buy', quantity: 0.5, price: 67000, status: 'filled' },
    { id: 2, symbol: 'ETH', side: 'sell', quantity: 2.0, price: 3400, status: 'filled' },
    { id: 3, symbol: 'SOL', side: 'buy', quantity: 10, price: 142, status: 'pending' },
    { id: 4, symbol: 'BTC', side: 'sell', quantity: 0.1, price: 68000, status: 'cancelled' },
  ];

  return (
    <div className="trading-page">
      <div className="trading-grid">
        {/* Order Form */}
        <div className="order-form">
          <h2>Place Order</h2>
          
          <div className="side-toggle">
            <button 
              className={side === 'buy' ? 'buy active' : 'buy'} 
              onClick={() => setSide('buy')}
            >
              Buy
            </button>
            <button 
              className={side === 'sell' ? 'sell active' : 'sell'} 
              onClick={() => setSide('sell')}
            >
              Sell
            </button>

          <form>
          </div onSubmit={handleSubmit}>
            <div className="form-group">
              <label>Symbol</label>
              <select value={symbol} onChange={e => setSymbol(e.target.value)}>
                <option value="BTC">BTC/USD</option>
                <option value="ETH">ETH/USD</option>
                <option value="SOL">SOL/USD</option>
                <option value="AAPL">AAPL</option>
                <option value="GOOGL">GOOGL</option>
              </select>
            </div>

            <div className="form-group">
              <label>Order Type</label>
              <select value={orderType} onChange={e => setOrderType(e.target.value as any)}>
                <option value="market">Market Order</option>
                <option value="limit">Limit Order</option>
              </select>
            </div>

            <div className="form-group">
              <label>Quantity</label>
              <input 
                type="number" 
                value={quantity} 
                onChange={e => setQuantity(e.target.value)}
                placeholder="0.00"
                step="0.01"
              />
            </div>

            {orderType === 'limit' && (
              <div className="form-group">
                <label>Price</label>
                <input 
                  type="number" 
                  value={price} 
                  onChange={e => setPrice(e.target.value)}
                  placeholder="0.00"
                  step="0.01"
                />
              </div>
            )}

            <div className="order-summary">
              <div className="summary-row">
                <span>Estimated Total</span>
                <span>$0.00</span>
              </div>
            </div>

            <button type="submit" className={`submit-btn ${side}`}>
              {side === 'buy' ? 'Buy' : 'Sell'} {symbol}
            </button>
          </form>
        </div>

        {/* Recent Orders */}
        <div className="recent-orders">
          <h2>Recent Orders</h2>
          <div className="orders-list">
            {recentOrders.map(order => (
              <div key={order.id} className="order-item">
                <div className="order-info">
                  <span className={`side ${order.side}`}>{order.side.toUpperCase()}</span>
                  <span className="symbol">{order.symbol}</span>
                </div>
                <div className="order-details">
                  <span className="quantity">{order.quantity}</span>
                  <span className="price">@ ${order.price.toLocaleString()}</span>
                </div>
                <span className={`status ${order.status}`}>{order.status}</span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
