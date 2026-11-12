fn main() {
    let nombre = "Edwin";
    let mut edad = 22;

    // Se puede hacer:
    println!("Hola, soy {} y tengo {} años", nombre, edad);
    // O hacer esto:
    println!("Hola, soy {nombre} y tengo {edad} años");

    edad = 23;
    println!("En mi proximo cumpleaños tendre {} años", edad);
}

/*
let: Es la palabra clave para crear una variable. Pero en Rust, todas las variables
son inmutables por defecto, eso significa que una vez asignado el valor "Edwin" a 
nombre, no se puede cambiar accidentalmente más adelante. Si se intenta escribir
nombre ="Carlos";, el compilador de Rust detendra el programa y marcara un error.

let mut: Al agregar mut (de mutable) le estoy diciendo explicitamente a Rust: "Voy a
cambiar el valor de esta variable más adelante". Por eso en la linea 8 se puede
actualizar el valor "edad=23;" sin problemas.

{}: (Llaves dentro del texto): Funcionan como marcadores de posición. Cuando se usa
la macro "println!"", Rust tomara las variables que pongas al final 
(separadas por comas) y las inyectara en orden dentro de las llaves.
*/