# Triqui Tree, tres en raya escrito en Rust usando WebAssembly
¡Puedes jugarlo en [`https://oktuvida.github.io/triqui-tree/`](https://oktuvida.github.io/triqui-tree/) !

*Lee esto en otros lenguajes: [English](README.md), [Español](README.es.md)*.
## ¿Por qué tres en raya?
tres en raya es un juego en el cual dos jugadores toman turnos entre sí para dibujar, usualmente, "O" o "X" en un tablero de 9 cuadrados. El ganador es aquel jugador que posea 3 símbolos iguales seguidos, ya sea mediante filas, columnas o diagonales. Su simplicidad lo hace ideal como objetivo de pedagogía; es un buen comienzo para adentrarse a la teoría de juegos como también a la inteligencia artificial.
## ¿Por qué Triqui Tree?
En Colombia, nuestro país de origen, tres en raya es llamado "Triqui". "Tree" es simplemente una alusión a "Árbol de juego", un concepto un poco teórico y que vendría perfecto en este contexto. 
## ¿Por qué Rust?
Porque Rust es vida, Rust es amor.
## ¿Por qué WebAssembly?
Es divertido escribir en Rust, como también lo es en HTML - CSS - Javascript, ¿por qué no unirlos?
## Resumen de desarrollo
### Juego
Para su implementación, partimos de la estructura "triqui", la cual contiene los símbolos en juego (siendo carácteres) y el tablero (siendo un vector de carácteres), como también su respectivo concepto de constructor. inicializando el tablero en el símbolo predeterminado. A esta estructura le implementamos las interfaces "algorithm", "strategy", y "board".
- La interfaz "board¨ implementa la interacción con el vector de carácteres, haciendo funciones como buscar y obtener un ganador.
- La interfaz "strategy" implementa el esquema general que debería tener el juego, como obtener los movimientos disponibles de juego, jugar y rehacer esta jugada, entre otros.
- La interfaz "algorithm" implementa el método de decisión que tendrá la IA. En este caso se usó el algoritmo recursivo Minimax con la poda alpha-beta, el cual consiste  en escoger un movimiento disponible que beneficie a la IA, mediante la diferenciación de movimientos: maximizar (conveniente para la IA) y minimizar (conveniente para el jugador), seleccionando obviamente este primero.
### Interfaz gráfica
Partimos de la estructura "controller", el cual le otorga a Javascript la información de cuál será la siguiente casilla jugada por la IA, como también de verificar si ya ha ganado un jugador. Javascript se encargará de asignar contenidos y estilos al HTML, como también de cambiar la dificultad del juego mediante el uso del almacenamiento de la sesión del navegador.
## Ejecución
### Requerimientos
- [`Rust`](https://www.rust-lang.org/tools/install)
- [`Node js`](https://nodejs.org/es/download/)
### Compilación
Supondremos que habrás descargado Rust mediante [rustup](https://rustup.rs/), de no ser así, podrías guiarte de [aquí](https://rustwasm.github.io/wasm-pack/book/prerequisites/non-rustup-setups.html). Primero deberás tener *wasm-pack*, para ello, abre una línea de comandos y escribe el siguiente comando:
```console
cargo install wasm-pack
```

¡Y listo, ya tienes todo lo necesario! Vuelve a la línea de comandos y escribe lo siguiente:

``` console
git clone --branch dev https://github.com/Oktuvida/triqui-tree.git
cd triqui-tree
wasm-pack build
cd www/
npm install
npm run start
```
Una vez hecho esto, se te informará en cual puerto podrás visualizar el contenido.
## Licencias
Este repositorio está bajo la licencia [MIT](LICENSE)
