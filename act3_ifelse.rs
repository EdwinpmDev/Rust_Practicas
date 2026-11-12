fn main(){
    let edad = 22;

    // 1.- Condición clasica (if / else if / else)
    if edad>=18{
        println!("Ya tienes la mayoria de edad");
    }else if edad < 0{
        println!("Edad invalida");
    } else {
        println!("Todavia eres menor de edad");
    }

    // El if es una expresión (asigna valores directamente)
    let etapa = if edad >= 18 {"Adulto"} else {"Joven"};

    println!("Actualmente estas en la etapa de: {etapa}");
}

/* 
Cero parentesis: No se necesita envolver la condición entre parentesis como en
"if (edad >= 18)". Rust prefiere mantenerlo limpio, similar a Python

Llaves obligatorias: En C++ se puede omitir las llaves {} si tu if solo tiene
una linea de código. En Rust las llaves son obligatorias siempre. Esto esta
diseñado a proposito para evitar errores clasicos donde la identación visual
engaña sobre lo que realmente esta dentro del bloque.

El if puede devolver valores: Como se puede ver en la declaración de la variable
"etapa", un bloque if es una "expresión", lo que significa que evalua y retorna
un resultado. Esto elimina por completo la necesidad del operador ternario
(condicion ? verdadero : falso) que se usa en C++. Solo hay una regla: Tanto el
bloque if como el else deben devolver exactamente el mismo tipo de dato

Un ejemplo de la regla siendo rota es:
fn main() {
    let edad = 22;

    // ERROR: Esto no compilará
    let etapa = if edad >= 18 { 
        "Adulto"  // Devuelve un texto (&str)
    } else { 
        0         // Devuelve un número entero (i32)
    };
}

Eso fallaria porque 
*/