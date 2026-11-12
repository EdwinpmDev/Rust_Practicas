fn main() {
    println!("Hola mundo!");
}

// Esta es la manera de escribir "Hola mundo!" en Rust

/* 
1.- fn main()
fn: Es la palabra clave para declarar una función (viene de function)

main: Es el nombre de la función. Al igual que en C++, main es especial
porque marca PUNTO DE ENTRADA de todo programa ejecutable en Rust; es siempre
lo primero que se ejecuta al correr el codigo.

(): Los parentesis indican los parametros que recibe la función. Al estar vaciós,
significa que main no recibe ningún dato de entrada al arrancar.

2.- { y }
{}: Las llaves delimitan el cuerpo o bloque de la función. Todo el codigo que este
dentro de ella pertenece a main y se ejecutara el orden de arriba hacia abajo.

3.- println!("Hola mundo!");

println!: Imprime texto en la consola, seguido de un salto de linea (print line), el
signo de exlamación en Rust indica que no se esta llamando a una función normal,
si no a una macro. Las macros son fragmentos de codigo que escriben más codigo
antes de que el programa termine de compilarse.
(metaprogramación). Se usa una macro aquí porque println! puede recibir una 
cantidad variable de argumentos y verificar tipos de datos en tiempo de compilación,
algo que las funciones de Rust estandar no hacen.

"Hola mundo!": Es una cadena de texto literal (string slice) que se pasa como
argumento dentro del parentesis

; El punto y coma marca el final de una sentencia (statement), indicando que esa
instrucción ha terminado y puede pasar a la siguiente.

CARGO es el gestor de paquetes de y herramienta de compilación, siendo más facil
configurar compiladores y librerias externas, a diferencia de C++ y su gestor de 
paquetes, acá  se usan comandos simples como cargo new y cargo run

*/