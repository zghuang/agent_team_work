import { useState } from 'react'
import { Brain, Play, Pause, Trash2 } from 'lucide-react'

export default function Strategies() {
  const [strategies, setStrategies] = useState([
    { id: '1', name: 'Moving Average Crossover', description: 'Buy when short MA crosses above long MA', assetType: 'Crypto', enabled: true, pnl: 1250 },
    { id: '2', name: 'RSI Overbought/Oversold', description: 'Buy when RSI < 30, sell when RSI > 70', assetType: 'Stock', enabled: true, pnl: 890 },
    { id: '3', name: 'MACD Divergence', description: 'Trade MACD histogram divergences', assetType: 'Crypto', enabled: false, pnl: -120 },
  ])

  const toggleStrategy = (id: string) => {
    setStrategies(strategies.map(s => s.id === id ? { ...s, enabled: !s.enabled } : s))
  }

  return (
    <div>
      <h1 className="text-3xl font-bold mb-6">Trading Strategies</h1>
      
      <div className="grid gap-4">
        {strategies.map((strategy) => (
          <div key={strategy.id} className="bg-gray-800 rounded-lg p-6 flex items-center justify-between">
            <div className="flex items-center space-x-4">
              <div className={`p-3 rounded-lg ${strategy.enabled ? 'bg-green-900' : 'bg-gray-700'}`}>
                <Brain className={`w-6 h-6 ${strategy.enabled ? 'text-green-400' : 'text-gray-400'}`} />
              </div>
              <div>
                <h3 className="font-semibold text-lg">{strategy.name}</h3>
                <p className="text-gray-400 text-sm">{strategy.description}</p>
                <span className="text-xs text-gray-500">{strategy.assetType}</span>
              </div>
            </div>
            
            <div className="flex items-center space-x-6">
              <div className="text-right">
                <div className={`font-bold ${strategy.pnl >= 0 ? 'text-green-400' : 'text-red-400'}`}>
                  {strategy.pnl >= 0 ? '+' : ''}${strategy.pnl}
                </div>
                <div className="text-xs text-gray-500">Total P&L</div>
              </div>
              
              <button
                onClick={() => toggleStrategy(strategy.id)}
                className={`p-2 rounded-lg ${strategy.enabled ? 'bg-yellow-900 hover:bg-yellow-800' : 'bg-green-900 hover:bg-green-800'}`}
              >
                {strategy.enabled ? <Pause className="w-5 h-5" /> : <Play className="w-5 h-5" />}
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}
