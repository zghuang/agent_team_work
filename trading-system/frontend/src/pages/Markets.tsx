import Layout from '../components/Layout';

const mockMarkets = [
  { symbol: 'BTC/USDT', price: 50123.50, change: 2.34, volume: '1.2B' },
  { symbol: 'ETH/USDT', price: 2756.80, change: -1.23, volume: '850M' },
  { symbol: 'SOL/USDT', price: 98.45, change: 5.67, volume: '320M' },
  { symbol: 'AAPL', price: 185.20, change: 0.45, volume: '52M' },
  { symbol: 'TSLA', price: 245.30, change: -2.10, volume: '108M' },
];

export default function Markets() {
  return (
    <Layout>
      <div className="space-y-6">
        <div className="flex justify-between items-center">
          <h1 className="text-2xl font-bold">Markets</h1>
          <div className="space-x-2">
            <button className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">
              Crypto
            </button>
            <button className="px-4 py-2 bg-gray-200 text-gray-700 rounded hover:bg-gray-300">
              Stocks
            </button>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow overflow-hidden">
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="text-left p-4">Symbol</th>
                <th className="text-right p-4">Price</th>
                <th className="text-right p-4">24h Change</th>
                <th className="text-right p-4">Volume</th>
                <th className="text-center p-4">Action</th>
              </tr>
            </thead>
            <tbody>
              {mockMarkets.map((market) => (
                <tr key={market.symbol} className="border-t hover:bg-gray-50">
                  <td className="p-4 font-medium">{market.symbol}</td>
                  <td className="p-4 text-right">${market.price.toLocaleString()}</td>
                  <td className={`p-4 text-right ${market.change >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                    {market.change >= 0 ? '+' : ''}{market.change}%
                  </td>
                  <td className="p-4 text-right">{market.volume}</td>
                  <td className="p-4 text-center">
                    <button className="px-3 py-1 text-sm bg-blue-100 text-blue-700 rounded hover:bg-blue-200">
                      Trade
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </Layout>
  );
}
