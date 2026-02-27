import { useState } from 'react';
import Layout from '../components/Layout';

const mockStrategies = [
  { id: 1, name: 'SMA Crossover', status: 'active', pairs: ['BTC/USDT', 'ETH/USDT'] },
  { id: 2, name: 'RSI Strategy', status: 'active', pairs: ['SOL/USDT'] },
  { id: 3, name: 'MACD Strategy', status: 'paused', pairs: ['BTC/USDT'] },
];

export default function Strategies() {
  const [strategies] = useState(mockStrategies);

  return (
    <Layout>
      <div className="space-y-6">
        <div className="flex justify-between items-center">
          <h1 className="text-2xl font-bold">Strategies</h1>
          <button className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">
            + Add Strategy
          </button>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {strategies.map((strategy) => (
            <div key={strategy.id} className="bg-white p-6 rounded-lg shadow">
              <div className="flex justify-between items-start mb-4">
                <h3 className="font-semibold text-lg">{strategy.name}</h3>
                <span className={`px-2 py-1 text-xs rounded ${
                  strategy.status === 'active' 
                    ? 'bg-green-100 text-green-700' 
                    : 'bg-gray-100 text-gray-700'
                }`}>
                  {strategy.status}
                </span>
              </div>
              <div className="space-y-2">
                <p className="text-sm text-gray-500">Trading Pairs</p>
                <div className="flex flex-wrap gap-2">
                  {strategy.pairs.map((pair) => (
                    <span key={pair} className="px-2 py-1 bg-gray-100 rounded text-sm">
                      {pair}
                    </span>
                  ))}
                </div>
              </div>
              <div className="mt-4 flex space-x-2">
                <button className="flex-1 px-3 py-2 bg-blue-600 text-white rounded text-sm hover:bg-blue-700">
                  Configure
                </button>
                <button className="px-3 py-2 border border-gray-300 rounded text-sm hover:bg-gray-50">
                  {strategy.status === 'active' ? 'Pause' : 'Start'}
                </button>
              </div>
            </div>
          ))}
        </div>
      </div>
    </Layout>
  );
}
