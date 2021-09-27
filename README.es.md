# Triqui Tree implementado en C++
[text]: ## "Interfaz de línea de comandos"
Por el momento, sólo tiene __[CLI][text]__; debe ser compilado y ejecutado desde la línea de comandos del sistema operativo.

*Lee esto en otros lenguajes: [English](README.md), [Español](README.es.md).*
## Requisitos
- [`Ninja`](https://github.com/ninja-build/ninja)

- [`Meson`](https://mesonbuild.com/Quick-guide.html)
## Pasos
Abre la línea de comandos y dirígete al directorio donde quieres que esté Triqui Tree. Allí, escribe los siguientes comandos:
### Compilación
```console
git clone --branch cpp-dev https://github.com/Oktuvida/triqui-tree.git
cd triqui-tree
meson builddir
ninja -C builddir
cd builddir
```
### Ejecución
 - #### GNU/Linux o Mac OS
```console
./triqui-tree
```
 - #### Windows
```console
triqui-tree.exe
```
