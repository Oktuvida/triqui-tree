import React, { useEffect, useState, useCallback } from 'react';
import { v4 as uuid } from 'uuid';
import { useLocation, useNavigate } from 'react-router-dom';

export function BoardComponent() {
    const location = useLocation();
    const navigate = useNavigate();
    const size: number = location.state.size ? location.state.size : 3;
    const minimizer: string = location.state.minimizer ? location.state.minimizer : 'X';
    const maximizer: string = location.state.maximizer ? location.state.maximizer : 'O';
    let box_played = 0;
    const get_algorithm_depth = useCallback(
        (): BigInt => {
            const var_name: string = "algorithm_depth";
            let algorithm_depth: number | null = Number(window.sessionStorage.getItem(var_name));
            if (algorithm_depth === null || algorithm_depth <= 7) {
                algorithm_depth = size === 3 ? 7 : size === 4 ? 4 : 3;
                window.sessionStorage.setItem(var_name, `${algorithm_depth}`);
            }
            return BigInt(algorithm_depth);
        },
        [size]
    );
    const [state, setState]: any = useState({
        controller: null
    });
    useEffect(() => {
        const init = async () => {
            try {
                const { Controller } = await import("triqui-tree");
                const controller = Controller.new(get_algorithm_depth(), size, maximizer, minimizer);
                setState((state: any) => ({ ...state, controller: controller }));
            } catch (err: any) {
                console.log(`Unexpected error in init. [Message: ${err.message}]`)
            }
        }
        init();
    }, [size, minimizer, maximizer, get_algorithm_depth]);
    const boxes: JSX.Element[] = Array.from({ length: size }, (_, i) =>
        <section key={uuid()}>
            {
                Array.from({ length: size }, (_, j) => 
                <span id={`box_${size * i + j}`} key={uuid()}></span>
                )
            }
        </section>
    );
    const get_winner = (game_state: number): string => {
        if (game_state <= state.controller.get_min_value()) {
            return "¡El ganador es la jugador!";
        }
        if (game_state >= state.controller.get_max_value()) {
            return "¡La ganadora es la máquina!";
        }
        return "¡Es empate!";
    }

    const play_move = (e: any) => {
        const id: string = e.target.id;
        if (id && !e.target.classList.contains("disabled") && state.controller != null) {
            const ind: number = Number(id.split('_')[1]);
            const board: any = state.controller.play_turn(ind);
            const game_state: number = board.get_board_state();
            const ai_id: number = board.get_movement_found();
            const ai_box: HTMLSpanElement | null = document.getElementById(`box_${ai_id}`);
            e.target.innerHTML = minimizer;
            e.target.classList.add("disabled");
            box_played += 2;
            if (ai_box && !ai_box.classList.contains("disabled")) {
                ai_box.innerHTML = maximizer;
                ai_box.classList.add("disabled");
            }
            if (ai_id === state.controller.get_invalid_mv()
                || box_played >= size * size
                || game_state <= state.controller.get_min_value()
                || game_state >= state.controller.get_max_value()) {
                navigate('/results', {
                    state: {
                        winner: get_winner(game_state)
                    }
                })
            }
        }
    }
    return (
        <div className="board">
            <div className="details">
                <div className="players">
                    <span className="x_turn">Turno {minimizer}</span>
                    <span className="o_turn">Turno {maximizer}</span>
                    <div className="slider"></div>
                </div>
            </div>
            <div className="play_area" onClick={play_move}>
                {boxes}
            </div>
        </div>
    );
}
