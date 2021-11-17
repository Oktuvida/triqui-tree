import React from 'react';
import './App.css';
import { HeaderComponent } from './components/HeaderComponent';
import { PlayerComponent } from './components/PlayerComponent';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import { BoardComponent } from './components/BoardComponent';
import { ResultComponent } from './components/ResultComponent';
import { SizeComponent } from './components/SizeComponent';

function App() {
  return (
    <Router>
      <HeaderComponent></HeaderComponent>
      <div className="container">
        <Routes>
          <Route path="/" element={<PlayerComponent />} />
          <Route path="board_size" element={<SizeComponent />}/>
          <Route path="/game" element={<BoardComponent />} />
          <Route path="/results" element={<ResultComponent />} />
        </Routes>
      </div>
    </Router>
  );
}

export default App;
