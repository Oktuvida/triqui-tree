import React from 'react';
import {useLocation, useNavigate} from 'react-router-dom';

export function ResultComponent () {
  const location = useLocation();
  const navigate = useNavigate();
  const winner = location.state.winner;
  return (
    <div className="result_box">
        <div className="winner_text">
          {winner ? winner : ''}
        </div>
        <div className="btn">
            <button 
              onClick={() => {
                navigate("/");
              }}
            >
              Jugar de nuevo
            </button>
        </div>
    </div>
  );
}
