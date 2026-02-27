import { useEffect, useRef } from 'react';
import { createChart, ColorType, CandlestickSeries, Time } from 'lightweight-charts';

interface ChartProps {
  symbol: string;
}

export default function TradingChart({ symbol }: ChartProps) {
  const chartContainerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!chartContainerRef.current) return;

    const chart = createChart(chartContainerRef.current, {
      layout: {
        background: { type: ColorType.Solid, color: '#1a1a2e' },
        textColor: '#d1d4dc',
      },
      grid: {
        vertLines: { color: '#2b2b43' },
        horzLines: { color: '#2b2b43' },
      },
      width: chartContainerRef.current.clientWidth,
      height: 400,
    });

    const candlestickSeries = chart.addSeries(CandlestickSeries, {
      upColor: '#26a69a',
      downColor: '#ef5350',
      borderVisible: false,
      wickUpColor: '#26a69a',
      wickDownColor: '#ef5350',
    });

    // Generate sample data
    const sampleData = generateSampleData();
    candlestickSeries.setData(sampleData);

    chart.timeScale().fitContent();

    const handleResize = () => {
      if (chartContainerRef.current) {
        chart.applyOptions({ width: chartContainerRef.current.clientWidth });
      }
    };

    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
      chart.remove();
    };
  }, [symbol]);

  return (
    <div className="trading-chart">
      <div className="chart-header">
        <h3>{symbol}/USD</h3>
        <span className="price">${generateRandomPrice().toFixed(2)}</span>
      </div>
      <div ref={chartContainerRef} className="chart-container" />
    </div>
  );
}

function generateSampleData() {
  const data: { time: Time; open: number; high: number; low: number; close: number }[] = [];
  const startTime = new Date('2024-01-01').getTime() / 1000;
  let price = 45000;

  for (let i = 0; i < 100; i++) {
    const change = (Math.random() - 0.5) * 1000;
    const open = price;
    const close = price + change;
    const high = Math.max(open, close) + Math.random() * 200;
    const low = Math.min(open, close) - Math.random() * 200;

    data.push({
      time: (startTime + i * 86400) as Time,
      open,
      high,
      low,
      close,
    });

    price = close;
  }

  return data;
}

function generateRandomPrice() {
  return 45000 + Math.random() * 5000;
}
