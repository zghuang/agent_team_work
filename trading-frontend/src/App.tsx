import { useState } from 'react';
import Dashboard from './pages/Dashboard';

function App() {
  const [currentPage, setCurrentPage] = useState('dashboard');

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
          <li 
            className={currentPage === 'messages' ? 'active' : ''}
            onClick={() => setCurrentPage('messages')}
          >
            Messages
          </li>
          <li 
            className={currentPage === 'settings' ? 'active' : ''}
            onClick={() => setCurrentPage('settings')}
          >
            Settings
          </li>
        </ul>
      </nav>
      <main className="main-content">
        {currentPage === 'dashboard' && <Dashboard />}
        {currentPage !== 'dashboard' && (
          <div className="coming-soon">
            <h2>{currentPage.charAt(0).toUpperCase() + currentPage.slice(1)}</h2>
            <p>Coming soon...</p>
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
