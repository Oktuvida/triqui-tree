import React from 'react';
import { useNavigate } from 'react-router';

export function PlayerComponent() {

    const navigate = useNavigate();
    const select_player = (player: string) => {
        navigate("/board_size", {
            state: {
                minimizer: player,
                maximizer: (player === 'X') ? 'O' : 'X'
            }
        }); 
    }
    return (
        <div className="box_selection">
            <p className="title">¿Con cuál símbolo deseas jugar?</p>
            <div className="options">
                <button className="player_x" onClick={() => select_player("X")}>Jugador (X)</button>
                <button className="player_y" onClick={() => select_player("O")}>Jugador (O)</button>
            </div>
        </div>
    );
}
