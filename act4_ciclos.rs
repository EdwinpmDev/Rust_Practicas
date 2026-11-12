fn main(){
    // 1.- Loop: El ciclo infinito por defecto
    let mut contador = 0;
    loop {
        contador +=1;
        if contador == 3{
            println!("Rompemos el contador infinito en {contador}");
            break; // La unica forma de salir del loop
        }
    }

    // 2.- while: El clasico, se ejecuta mientras la condicion sea verdadera.
    let mut energia = 3;
    while energia > 0 {
        println!("Entrenando....energia restante {energia}");
        energia -= 1;
    }

    // 3.- for: El top de Rust, ideal para rangos y listas.
    println!("Contando con for:");
    for numero in 1..=5{
        println!("- {numero}"); 
    }
}

/* 
loop: Es exclusivo de Rust (y algún otro lenguaje moderno). Es un ciclo
infinito. En C++ usaria while(true), pero Rust introdujo "loop" porque le permite
al compilador hacer optimizaciones especiales y saber con certeza que el programa
no saldra de ahi a menos que se encuentre con un break

while: Funciona exactamente igual a C++. Evalua una condición antes de cada
iteración y se detiene cuando esta se vuelve falsa. Igual que con el if, la 
condición no lleva parentesis.

for: En rust, el for se usa siempre como un iterador. La sintaxis "1..=5" crea
un rango inclusivo (del 1 al 5). Si usaras 1..5 (sin el signo de igual) contaria del
1 al 4. Es mucho más seguro que manejar contadores manuales porque es imposible
equivocarse sumando de más o accediendo a memoria que no existe. 
(el error: "Off-By-One" de C++)
*/