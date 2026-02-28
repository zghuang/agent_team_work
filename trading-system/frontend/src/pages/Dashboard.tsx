import Layout from '../components/Layout';

export default function Dashboard() {
  return (
    <Layout>
      <div className="space-y-6">
        <h1 className="text-2xl font-bold">Dashboard</h1>
        
        {/* Portfolio Summary */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div className="bg-white p-6 rounded-lg shadow">
            <p className="text-sm text-gray-500">Total Value</p>
            <p className="text-2xl font-bold">$10,000.00</p>
          </div>
          <div className="bg-white p-6 rounded-lg shadow">
            <p className="text-sm text-gray-500">Daily P&L</p>
            <p className="text-2xl font-bold text-green-600">+$250.00</p>
          </div>
          <div className="bg-white p-6 rounded-lg shadow">
            <p className="text-sm text-gray-500">Open Positions</p>
            <p className="text-2xl font-bold">3</p>
          </div>
          <div className="bg-white p-6 rounded-lg shadow">
            <p className="text-sm text-gray-500">Active Strategies</p>
            <p className="text-2xl font-bold">2</p>
          </div>
        </div>

        {/* Price Chart Placeholder */}
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-lg font-semibold mb-4">BTC/USDT</h2>
          <div className="h-64 bg-gray-100 flex items-center justify-center">
            <p className="text-gray-500">Chart will be loaded here</p>
          </div>
        </div>

        {/* Recent Trades */}
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-lg font-semibold mb-4">Recent Trades</h2>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="text-left py-2">Time</th>
                  <th className="text-left py-2">Symbol</th>
                  <th className="text-left py-2">Side</th>
                  <th className="text-right py-2">Price</th>
                  <th className="text-right py-2">Qty</th>
                </tr>
              </thead>
              <tbody>
                <tr className="border-b">
                  <td className="py-2">14:30:25</td>
                  <td>BTC/USDT</td>
                  <td className="text-green-600">BUY</td>
                  <td className="text-right">50,123.50</td>
                  <td className="text-right">0.01</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2">14:28:10</td>
                  <td>ETH/USDT</td>
                  <td className="text-red-600">SELL</td>
                  <td className="text-right">2,756.80</td>
                  <td className="text-right">0.5</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </Layout>
  );
}
