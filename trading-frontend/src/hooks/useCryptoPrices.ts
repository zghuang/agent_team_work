import { useState, useEffect, useRef, useCallback } from 'react';

interface Ticker {
  symbol: string;
  price: number;
  change_24h: number;
  volume_24h: number;
  high_24h: number;
  low_24h: number;
}

export default function useCryptoPrices() {
  const [prices, setPrices] = useState<Ticker[]>([]);
  const [loading, setLoading] = useState(true);
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  const connectWebSocket = useCallback(() => {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}/ws/market`;
    
    // Fallback to localhost if not available
    const finalUrl = wsUrl.replace(window.location.host, 'localhost:8080');
    
    try {
      const ws = new WebSocket(finalUrl);
      
      ws.onopen = () => {
        console.log('WebSocket connected');
        setLoading(false);
      };
      
      ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          if (Array.isArray(data)) {
            setPrices(data);
          }
        } catch (e) {
          console.error('Failed to parse WebSocket message:', e);
        }
      };
      
      ws.onerror = (error) => {
        console.error('WebSocket error:', error);
      };
      
      ws.onclose = () => {
        console.log('WebSocket disconnected, reconnecting...');
        // Reconnect after 3 seconds
        reconnectTimeoutRef.current = setTimeout(() => {
          connectWebSocket();
        }, 3000);
      };
      
      wsRef.current = ws;
    } catch (error) {
      console.error('Failed to connect WebSocket:', error);
      // Fallback to REST API
      fetchPrices();
    }
  }, []);

  const fetchPrices = async () => {
    try {
      const response = await fetch('http://localhost:8080/api/v1/prices');
      if (response.ok) {
        const data = await response.json();
        setPrices(data);
      } else {
        setPrices(getFallbackPrices());
      }
    } catch {
      console.error('Failed to fetch prices');
      setPrices(getFallbackPrices());
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    // Try WebSocket first, fall back to REST
    connectWebSocket();

    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
      if (reconnectTimeoutRef.current) {
        clearTimeout(reconnectTimeoutRef.current);
      }
    };
  }, [connectWebSocket]);

  return { prices, loading };
}

function getFallbackPrices(): Ticker[] {
  return [
    { symbol: 'BTCUSDT', price: 67500, change_24h: 2.5, volume_24h: 28000000000, high_24h: 68000, low_24h: 66000 },
    { symbol: 'ETHUSDT', price: 3450, change_24h: -1.2, volume_24h: 15000000000, high_24h: 3500, low_24h: 3400 },
    { symbol: 'BNBUSDT', price: 580, change_24h: 0.8, volume_24h: 1200000000, high_24h: 590, low_24h: 570 },
    { symbol: 'SOLUSDT', price: 145, change_24h: 5.8, volume_24h: 2500000000, high_24h: 150, low_24h: 138 },
    { symbol: 'XRPUSDT', price: 0.52, change_24h: -0.5, volume_24h: 1500000000, high_24h: 0.53, low_24h: 0.51 },
  ];
}
