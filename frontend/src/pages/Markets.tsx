export default function Markets() {
  const assets = [
    { symbol: 'BTC', name: 'Bitcoin', price: 43500, change: 2.5, type: 'Crypto' },
    { symbol: 'ETH', name: 'Ethereum', price: 2350, change: 1.8, type: 'Crypto' },
    { symbol: 'AAPL', name: 'Apple Inc.', price: 185, change: -0.5, type: 'Stock' },
    { symbol: 'GOOGL', name: 'Alphabet Inc.', price: 140, change: 1.2, type: 'Stock' },
  ]

  return (
    <div>
      <h1 className="text-3xl font-bold mb-6">Markets</h1>
      
      <div className="bg-gray-800 rounded-lg overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-700">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Symbol</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Name</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Type</th>
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-300 uppercase tracking-wider">Price</th>
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-300 uppercase tracking-wider">24h Change</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-700">
            {assets.map((asset) => (
              <tr key={asset.symbol} className="hover:bg-gray-750">
                <td className="px-6 py-4 whitespace-nowrap font-medium">{asset.symbol}</td>
                <td className="px-6 py-4 whitespace-nowrap text-gray-300">{asset.name}</td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span className={`px-2 py-1 text-xs rounded ${asset.type === 'Crypto' ? 'bg-purple-900 text-purple-300' : 'bg-blue-900 text-blue-300'}`}>
                    {asset.type}
                  </span>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-right">${asset.price.toLocaleString()}</td>
                <td className={`px-6 py-4 whitespace-nowrap text-right ${asset.change >= 0 ? 'text-green-400' : 'text-red-400'}`}>
                  {asset.change >= 0 ? '+' : ''}{asset.change}%
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}
