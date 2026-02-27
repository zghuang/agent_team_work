import { useState, useEffect } from 'react'

interface Order {
  id: number
  symbol: string
  quantity: number
  price: number
  status: string
}

function App() {
  const [orders, setOrders] = useState<Order[]>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    fetch('/api/orders')
      .then(res => res.json())
      .then(data => {
        setOrders(data)
        setLoading(false)
      })
      .catch(err => {
        console.error('Failed to fetch orders:', err)
        setLoading(false)
      })
  }, [])

  return (
    <div style={{ padding: '20px', fontFamily: 'system-ui, sans-serif' }}>
      <h1>Trading Platform</h1>
      <p>Welcome to the trading system</p>
      
      <h2>Orders</h2>
      {loading ? (
        <p>Loading...</p>
      ) : (
        <table style={{ borderCollapse: 'collapse', width: '100%' }}>
          <thead>
            <tr style={{ borderBottom: '2px solid #ccc' }}>
              <th style={{ padding: '10px', textAlign: 'left' }}>ID</th>
              <th style={{ padding: '10px', textAlign: 'left' }}>Symbol</th>
              <th style={{ padding: '10px', textAlign: 'right' }}>Quantity</th>
              <th style={{ padding: '10px', textAlign: 'right' }}>Price</th>
              <th style={{ padding: '10px', textAlign: 'left' }}>Status</th>
            </tr>
          </thead>
          <tbody>
            {orders.map(order => (
              <tr key={order.id} style={{ borderBottom: '1px solid #eee' }}>
                <td style={{ padding: '10px' }}>{order.id}</td>
                <td style={{ padding: '10px' }}>{order.symbol}</td>
                <td style={{ padding: '10px', textAlign: 'right' }}>{order.quantity}</td>
                <td style={{ padding: '10px', textAlign: 'right' }}>${order.price.toFixed(2)}</td>
                <td style={{ padding: '10px' }}>
                  <span style={{ 
                    padding: '4px 8px', 
                    borderRadius: '4px',
                    backgroundColor: order.status === 'filled' ? '#4caf50' : '#ff9800',
                    color: 'white'
                  }}>
                    {order.status}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  )
}

export default App
