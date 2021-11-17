import React from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import {v4 as uuid} from 'uuid';

export function SizeComponent() {
    const location = useLocation();
    const navigate = useNavigate();

    const init_game = (size: number) => {
        navigate("/game", {
            state: {
                minimizer: location.state.minimizer,
                maximizer: location.state.maximizer,
                size: size
            }
        });
    }
    const buttons: JSX.Element[] = [3, 4, 5].map(number => 
        <div className="btn" key={uuid()}>
            <button 
                onClick={() => init_game(number)}
            >
                <p>{number}</p> casillas
            </button>
        </div>
    );
    return (
        <div className="box_selection size_selection">
            <p className="title">¿Con cuál tamaño de tablero deseas jugar?</p>
            <div className="options">
                {
                    buttons
                }
            </div>
        </div>
    );
}
