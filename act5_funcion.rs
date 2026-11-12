fn main(){
    let precio_base = 200;
    let descuento_aplicado = calcular_descuento(precio_base, 15);

    println!("El precio con descuento es de {descuento_aplicado}");
}

// Nueva función: recibe dos parametros (i32) y retorna un entero (i32)
fn calcular_descuento(precio: i32, porcentaje: i32) -> i32 {
    let descuento = (precio * porcentaje) / 100;

    // Aquí no hay palabra "return" ni coma al final o punto y coma
    precio - descuento

}

/* 
Tipos obligatorios en los parametros: "(precio: i32)" A diferencia de las variables con let
donde Rust suele adivinar si es un numero o un texto, en las funciones se esta obligado a 
decirle exactamente que tipo de dato es cada parametro. i32 significa entero de 32bits.
(el equivalente al int de C++)

La flecha de retorno: (-> i32): Asi es como se anuncia el tipo de dato que devolvera la funcion
Si la función no devuelve nada (como el main) entonces solo se omite esa flecha

El retorno implicito (sin punto y coma): En Rust, si la ultima linea de la función no tiene punto
y coma, el compilador automaticamente asume que ese es el valor que se quiere retornar. No se 
necesita escribir "return precio - descuento". Al quitarle el ; la linea se convierte en una 
expresión que muestra el valor hacia afuera.

Se puede usar la palabra "return" si se quiere salir antes de tiempo de la funcion 
(dentro de un if por ejemplo)
pero al final de la funcion la practica estandar es dejar la ultima linea sin el ;
*/