import * as triqui_tree from "./functions";
triqui_tree.set_message_difficulty();
triqui_tree.modify_input_attributes("checked");
document
    .getElementById("board")
    .addEventListener("click", triqui_tree.process_move);
document
    .getElementById("board")
    .addEventListener("mouseover", triqui_tree.show_symbol_hover);
document
    .getElementById("board")
    .addEventListener("mouseout", triqui_tree.hide_symbol_hover);
