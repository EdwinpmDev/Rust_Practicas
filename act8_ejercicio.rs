// SIMULADOR DE COMBATO CONTRA UN JEFE

//REGLAS PARA EL EJERCICO
/* 
1.- La estructura (Struct)
Crear un molde llamado "Jefe". Debe tener tres propiedades:

-Nombre (de tipo String)
-Vida (de tipo i32)
-Derrotado (de tipo bool)

2.- Función de cálculo (Retorno y ciclo for)
Crea una función separada (afuera del main) llamada "calcular_dano_combo"

-Debe recibir un parametro llamado "golpes" (de tipo i32)
-Debe usar un ciclo for para contar desde 1 hasta el numero de golpes. Por cada golpe
sumara 15 puntos de daño a un acumulador.
-Debe retornar (-> i32) y el daño total calculado. (Ejemplo: Si le pasa 3 golpes, el ciclo
suma 15+15+15 y retorna 45).

3.- Función de acción (Prestamo &mut y condicionales if)
Crea otra función llamada recibir_ataque

-Debe recibir dos cosas: al jefe prestado para poder modificarlo (objetivo: &mut Jefe)
y el daño a restar (dano: i32)
-Debe restarle el daño a la vida del jefe
-Usa un if/else: Si la vida del jefe cae a 0 o menos, fuerza su vida a que sea 0. Cambia
su estado "derrotado" a "true", e imprime "El jefe [nombre] ha caido!" Si no, imprime
"Al jefe le quedan [vida] puntos"

4.- La arena de combate (variables y ciclos while)
En tu fn main():

-Crea la variable mutable de tu jefe (ej. "Sephirot" con 100 de vida y derrotado en "false")
-Crea una variable "numero_de_golpes = 2"
-El ciclo principal: Crea un ciclo "while" que se ejecute mientras el jefe NO este derrotado:
dentro del ciclo:
1.- Llamar a "calcular_dano_combo" pasandole los golpes
2.- Pasarle ese daño a "recibir_ataque" (recuerda usar el &mut cuando le pase la variable al jefe)

Extra: Si en el while se pone !jefe.derrotado, el signo de exclamación ! significa "NO"
(mientras no este derrotado). O tambien se puede usar "while jefe.derrotado == false"

*/