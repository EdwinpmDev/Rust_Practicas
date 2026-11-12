fn main(){
    //ESCENARIO 1.- Loop (Buscando conexión)
    /* No sabemos si se tomara 1 o 100 intentos conectarse. El sistema
    deber seguir intentandolo infinitamente hasta lograrlo. Cuando lo logre, se
    usara el break para detener el ciclo y extraer el numero de conexión.*/

    let mut intentos = 0;

    // Al poner = loop estamos diciendo que se haga de manera infinita
    let id_conexion = loop {
        intentos +=1;
        println!("Buscando un servidor...(Intento: {intentos})");

        if intentos == 3{ //Simulamos que al tercer intento lo logra
            println!("Conexión segura!");
            break 8000; // Detiene el ciclo infinito y devuelve el puerto
        }
    };

    // ESCENARIO 2: El while (descargando datos mientras haya energia)
    /* Ya tenemos conexión. Ahora descargaremos archivos. Pero dependemos de la bateria.
    Se ejecuta MIENTRAS la condición sea verdadera. Si la bateria fuera 0 desde
    el inicio, ni siquiera estaria el ciclo.*/
    let mut bateria = 25; // 25% de bateria
    let mut archivos_descargados = 0;

    println!("\nIniciando descarga en el puerto {id_conexion}...");

    while bateria > 0{
        archivos_descargados +=1;
        bateria -=5; // Cada archivo consume 5% de bateria
        println!("Descargando archivo {archivos_descargados}... Bateria al {bateria}%")
    }

    println!("Descargas detenidas por falta de energia.");

    // ESCENARIO 3.- El for (procesando una cantidad exacta)
    /* El sistema se apagara, pero antes debe instalar los archivos que logro bajar.
    Aquí sabemos EXACTAMENTE cuantos archivos son, asi que usamos un rango planificado.*/

    println!("\nInstalando archivos locales");

    for numero in 1..=archivos_descargados {
        println!("- Descomprimiendo archivo numero {numero}");
    }

    println!("\nSistema listo!");
}
