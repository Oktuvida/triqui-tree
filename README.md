# Triqui Tree implemented in C++
[text]: ## "Command Line Interface"
At the moment, it only has __[CLI][text]__; it must be compiled and executed from the command line of the operating system.

*Read this in other languages: [English](README.md), [Español](README.es.md).*
## Requirements
- [`Ninja`](https://github.com/ninja-build/ninja)

- [`Meson`](https://mesonbuild.com/Quick-guide.html)
## Steps
Open the command line and go to the directory where you want Triqui Tree to be. There, type the following commands:
### Building
``` console
git clone --branch cpp-dev https://github.com/Oktuvida/triqui-tree.git
cd triqui-tree
meson builddir
ninja -C builddir
cd builddir
```
### Running
 - #### GNU/Linux or Mac OS
``` console
./triqui-tree
```
 - #### Windows
```console
triqui-tree.exe
```
