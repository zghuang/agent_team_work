import { useState, useEffect } from 'react';
import Dashboard from './pages/Dashboard';
import LoginPage from './pages/Login';

function App() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [username, setUsername] = useState('');
  const [currentPage, setCurrentPage] = useState('dashboard');

  useEffect(() => {
    const storedToken = localStorage.getItem('token');
    const storedUsername = localStorage.getItem('username');
    if (storedToken && storedUsername) {
      setIsAuthenticated(true);
      setUsername(storedUsername);
    }
  }, []);

  const handleLogin = (_token: string, user: string) => {
    setIsAuthenticated(true);
    setUsername(user);
  };

  const handleLogout = () => {
    localStorage.removeItem('token');
    localStorage.removeItem('username');
    setIsAuthenticated(false);
    setUsername('');
  };

  if (!isAuthenticated) {
    return <LoginPage onLogin={handleLogin} />;
  }

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
          <li onClick={handleLogout} className="logout">
            Logout ({username})
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
