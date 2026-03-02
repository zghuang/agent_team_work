import PriceChart from '../components/PriceChart';
import Layout from '../components/Layout';

export default function Charts() {
  return (
    <Layout>
      <div className="space-y-6">
        <h1 className="text-2xl font-bold">Trading Charts</h1>
        <PriceChart />
      </div>
    </Layout>
  );
}
