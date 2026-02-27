import { useState, useEffect } from 'react'
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts'

// Mock data for demonstration
const mockData = [
  { time: '00:00', price: 42000 },
  { time: '04:00', price: 42500 },
  { time: '08:00', price: 41800 },
  { time: '12:00', price: 43200 },
  { time: '16:00', price: 43800 },
  { time: '20:00', price: 43500 },
]

export default function Dashboard() {
  const [portfolio, setPortfolio] = useState({
    totalValue: 125000,
    dailyChange: 2.5,
    positions: 5,
  })

  return (
    <div>
      <h1 className="text-3xl font-bold mb-6">Dashboard</h1>
      
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div className="bg-gray-800 rounded-lg p-6">
          <div className="text-gray-400 text-sm">Total Portfolio Value</div>
          <div className="text-3xl font-bold">${portfolio.totalValue.toLocaleString()}</div>
        </div>
        <div className="bg-gray-800 rounded-lg p-6">
          <div className="text-gray-400 text-sm">Daily Change</div>
          <div className={`text-3xl font-bold ${portfolio.dailyChange >= 0 ? 'text-green-400' : 'text-red-400'}`}>
            {portfolio.dailyChange >= 0 ? '+' : ''}{portfolio.dailyChange}%
          </div>
        </div>
        <div className="bg-gray-800 rounded-lg p-6">
          <div className="text-gray-400 text-sm">Open Positions</div>
          <div className="text-3xl font-bold">{portfolio.positions}</div>
        </div>
      </div>

      <div className="bg-gray-800 rounded-lg p-6">
        <h2 className="text-xl font-semibold mb-4">Portfolio Performance</h2>
        <div className="h-80">
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={mockData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
              <XAxis dataKey="time" stroke="#9CA3AF" />
              <YAxis stroke="#9CA3AF" />
              <Tooltip 
                contentStyle={{ backgroundColor: '#1F2937', border: 'none' }}
                labelStyle={{ color: '#9CA3AF' }}
              />
              <Line type="monotone" dataKey="price" stroke="#10B981" strokeWidth={2} />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>
    </div>
  )
}
