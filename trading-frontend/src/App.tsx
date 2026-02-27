import { useState } from 'react';
import Dashboard from './pages/Dashboard';
import Markets from './pages/Markets';
import Strategies from './pages/Strategies';
import Trading from './pages/Trading';

function App() {
  const [currentPage, setCurrentPage] = useState('dashboard');

  const renderPage = () => {
    switch(currentPage) {
      case 'dashboard':
        return <Dashboard />;
      case 'markets':
        return <Markets />;
      case 'strategies':
        return <Strategies />;
      case 'trading':
        return <Trading />;
      default:
        return <Dashboard />;
    }
  };

  return (
    <div className="app">
      <nav className="sidebar">
        <div className="logo">
          <h2>Trading Pro</h2>
        </div>
        <ul className="nav-links">
          <li 
            className={currentPage === 'dashboard' ? 'active' : ''}
            onClick={() => setCurrentPage('dashboard')}
          >
            Dashboard
          </li>
          <li 
            className={currentPage === 'markets' ? 'active' : ''}
            onClick={() => setCurrentPage('markets')}
          >
            Markets
          </li>
          <li 
            className={currentPage === 'trading' ? 'active' : ''}
            onClick={() => setCurrentPage('trading')}
          >
            Trading
          </li>
          <li 
            className={currentPage === 'strategies' ? 'active' : ''}
            onClick={() => setCurrentPage('strategies')}
          >
            Strategies
          </li>
        </ul>
      </nav>
      <main className="main-content">
        {renderPage()}
      </main>
    </div>
  );
}

export default App;
