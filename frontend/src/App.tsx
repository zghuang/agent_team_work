import { BrowserRouter, Routes, Route } from 'react-router-dom'
import Dashboard from './pages/Dashboard'
import Markets from './pages/Markets'
import Strategies from './pages/Strategies'
import Trading from './pages/Trading'
import Layout from './components/Layout'

function App() {
  return (
    <BrowserRouter>
      <Layout>
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/markets" element={<Markets />} />
          <Route path="/strategies" element={<Strategies />} />
          <Route path="/trading" element={<Trading />} />
        </Routes>
      </Layout>
    </BrowserRouter>
  )
}

export default App
