import {
    Controller
} from "triqui-tree";
const controller = Controller.new(get_algorithm_depth());
function get_algorithm_depth() {
    let algorithm_depth = window.sessionStorage.getItem("algorithm_depth");
    if (algorithm_depth === null || algorithm_depth > 7) {
        algorithm_depth = 7;
        window.sessionStorage.setItem("algorithm_depth", algorithm_depth);
    }
    return BigInt(algorithm_depth);
}

function set_algorithm_depth(inner_msg) {
    let algorithm_depth = get_algorithm_depth();
    let depth_values = {
        "Es empate": algorithm_depth,
        "La ganadora es la máquina": algorithm_depth - BigInt(1),
        "El ganador es el jugador": algorithm_depth + BigInt(1),
    };
    let new_algorithm_depth = depth_values[inner_msg];
    window.sessionStorage.setItem(
        "algorithm_depth",
        new_algorithm_depth > 0 ? new_algorithm_depth : 7
    );
}

function create_h4_element_with_animation(msg, width_element, typing_type, order) {
    let h4_element = document.createElement("h4");
    h4_element.innerHTML = msg;
    let time_duration = 1;
    let steps = msg.trim().length;
    Object.assign(h4_element.style, {
        width: width_element,
        animation: `
            ${typing_type} ${time_duration}s steps(${steps}) forwards ${order*time_duration}s,
            blink ${time_duration/steps}s step-end infinite alternate
        `,
        opacity: (order>0) ? '0' : '1'
    });
    return h4_element;

}

function end_of_game_screen() {
    modify_input_attributes("disabled");
    let inner_msg = controller.get_winner();
    set_algorithm_depth(inner_msg);
    let message_square = document.getElementById("final_message");
    switch(inner_msg) {
        case "Es empate": 
            message_square.appendChild(
                create_h4_element_with_animation(inner_msg, "17rem", "single_typing", 0)
            );
            break;
        default:
            let size = inner_msg.length / 2 - 1;
            let msg = inner_msg.substring(0, size);
            message_square.appendChild(
                create_h4_element_with_animation(msg, (msg==="El ganador ")?"18.8rem":"20.2rem", "first_typing", 0)
            );
            msg = inner_msg.substring(size) ;
            message_square.appendChild(
                create_h4_element_with_animation(msg, "24.1rem", "last_typing", 1)
            );
    }
    message_square.classList.add("typing_text");
    Object.assign(document.getElementById("repeat_button").style, {
        opacity: '1',
        visibility: "visible",
    });
}

function play_movement(square, is_ai) {
    square.classList.add(
        is_ai ? controller.get_maximizer() : controller.get_minimizer(),
        "used",
        "disabled"
    );
    square.style.cursor = "default";
    if (is_ai) 
        square.style.color = "#111";
    Object.assign(square.parentNode.querySelector("INPUT"), {
        checked: true,
        disabled: true
    })
}
export function modify_input_attributes(action) {
    switch (action) {
        case "checked":
            document.querySelectorAll("INPUT").forEach((el) => {
                el.checked = false;
            });
            break;
        default:
            document.querySelectorAll("I").forEach((el) => {
                el.classList.add("disabled");
                el.style.cursor = "default";
            });
    }
}
export function process_move(ev) {
    let player_square = ev.target;
    let square_classes = player_square.classList;
    if (
        square_classes.contains("icon-box")
        && !square_classes.contains("used")
        && !square_classes.contains("disabled")
    ) {
        play_movement(player_square, false);
        let ai_mv = controller.play_turn(player_square.id.slice(-1));
        if (ai_mv != controller.get_invalid_mv())
            play_movement(document.getElementById(`square_${ai_mv}`), true);
        if (controller.completed_game())
            end_of_game_screen();
    }
}
export function show_symbol_hover(ev) {
    let square = ev.target;
    let square_classes = square.classList;
    if (
        square_classes.contains("icon-box")
        && !square_classes.contains("used")
        && !square_classes.contains("disabled")
    ) 
        square_classes.add(controller.get_minimizer());
}
export function hide_symbol_hover(ev) {
    let square = ev.target;
    let square_classes = square.classList;
    if (
        square_classes.contains("icon-box")
        && !square_classes.contains("used") 
        && !square_classes.contains("disabled")
    ) 
        square_classes.remove(controller.get_minimizer());
}
export function set_message_difficulty() {
    let box_message = document.querySelector("#show_difficulty h4");
    switch(window.sessionStorage.getItem("algorithm_depth")) {
        case '7':
            break;
        case '6':
        case '5':
            box_message.innerHTML = "Dificultad: Normal";
            break;
        default:
            box_message.innerHTML = "Dificultad: Fácil";
    }
}
