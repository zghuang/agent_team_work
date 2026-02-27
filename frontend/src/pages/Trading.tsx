import { useState } from 'react'
import { ArrowUp, ArrowDown } from 'lucide-react'

export default function Trading() {
  const [orderSide, setOrderSide] = useState<'buy' | 'sell'>('buy')
  const [symbol, setSymbol] = useState('BTC')
  const [quantity, setQuantity] = useState('')
  const [price, setPrice] = useState('')

  const recentOrders = [
    { id: '1', symbol: 'BTC', side: 'buy', quantity: 0.5, price: 43000, status: 'filled' },
    { id: '2', symbol: 'ETH', side: 'sell', quantity: 2.0, price: 2300, status: 'filled' },
    { id: '3', symbol: 'AAPL', side: 'buy', quantity: 10, price: 184, status: 'pending' },
  ]

  return (
    <div>
      <h1 className="text-3xl font-bold mb-6">Trading</h1>
      
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Order Form */}
        <div className="bg-gray-800 rounded-lg p-6">
          <h2 className="text-xl font-semibold mb-4">Place Order</h2>
          
          <div className="flex space-x-2 mb-4">
            <button
              onClick={() => setOrderSide('buy')}
              className={`flex-1 py-2 rounded-lg flex items-center justify-center ${
                orderSide === 'buy' ? 'bg-green-600' : 'bg-gray-700'
              }`}
            >
              <ArrowUp className="w-4 h-4 mr-2" /> Buy
            </button>
            <button
              onClick={() => setOrderSide('sell')}
              className={`flex-1 py-2 rounded-lg flex items-center justify-center ${
                orderSide === 'sell' ? 'bg-red-600' : 'bg-gray-700'
              }`}
            >
              <ArrowDown className="w-4 h-4 mr-2" /> Sell
            </button>
          </div>
          
          <div className="space-y-4">
            <div>
              <label className="block text-sm text-gray-400 mb-1">Symbol</label>
              <select
                value={symbol}
                onChange={(e) => setSymbol(e.target.value)}
                className="w-full bg-gray-700 rounded-lg px-4 py-2"
              >
                <option value="BTC">BTC - Bitcoin</option>
                <option value="ETH">ETH - Ethereum</option>
                <option value="AAPL">AAPL - Apple</option>
                <option value="GOOGL">GOOGL - Google</option>
              </select>
            </div>
            
            <div>
              <label className="block text-sm text-gray-400 mb-1">Quantity</label>
              <input
                type="number"
                value={quantity}
                onChange={(e) => setQuantity(e.target.value)}
                placeholder="0.00"
                className="w-full bg-gray-700 rounded-lg px-4 py-2"
              />
            </div>
            
            <div>
              <label className="block text-sm text-gray-400 mb-1">Price (optional for market order)</label>
              <input
                type="number"
                value={price}
                onChange={(e) => setPrice(e.target.value)}
                placeholder="0.00"
                className="w-full bg-gray-700 rounded-lg px-4 py-2"
              />
            </div>
            
            <button
              className={`w-full py-3 rounded-lg font-semibold ${
                orderSide === 'buy' ? 'bg-green-600 hover:bg-green-500' : 'bg-red-600 hover:bg-red-500'
              }`}
            >
              {orderSide === 'buy' ? 'Buy' : 'Sell'} {symbol}
            </button>
          </div>
        </div>
        
        {/* Recent Orders */}
        <div className="bg-gray-800 rounded-lg p-6">
          <h2 className="text-xl font-semibold mb-4">Recent Orders</h2>
          
          <div className="space-y-3">
            {recentOrders.map((order) => (
              <div key={order.id} className="flex items-center justify-between bg-gray-700 rounded-lg p-4">
                <div className="flex items-center space-x-3">
                  <span className={`font-bold ${order.side === 'buy' ? 'text-green-400' : 'text-red-400'}`}>
                    {order.side.toUpperCase()}
                  </span>
                  <span className="font-medium">{order.symbol}</span>
                </div>
                <div className="text-right">
                  <div>{order.quantity} @ ${order.price.toLocaleString()}</div>
                  <span className={`text-xs px-2 py-1 rounded ${
                    order.status === 'filled' ? 'bg-green-900 text-green-300' : 'bg-yellow-900 text-yellow-300'
                  }`}>
                    {order.status}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  )
}
