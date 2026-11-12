//1.- Definimos el molde. Esto no ocupa memoria todavia.
struct Jugador {
    nombre: String, // Usamos String porque el jugador sera dueño de este texto
    nivel: i32,
    energia: i32,
    esta_vivo: bool,
}

fn main() {
    // 2.- Usamos el molde para crear una instancia
    let mut nuevo_jugador = Jugador {
        nombre: String::from("Edwin"),
        nivel: 1,
        energia: 100,
        esta_vivo: true,
    };

    // 3.- Accedemos a sus datos usando un punto (.)
    println!("Jugador: {}", nuevo_jugador.nombre);
    println!("Nivel actual: {}", nuevo_jugador.nivel);
    
    // 4.- Como la variable es "mut", podemos modificar sus piezas individuales
    nuevo_jugador.energia -= 20;
    println!("La energia actual del jugador es: {}", nuevo_jugador.energia);
}