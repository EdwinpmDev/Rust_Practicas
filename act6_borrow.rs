fn main(){
    // 1.- Prestamo inmutable
    let mensaje = String::from("Sistema en linea");

    let lector1 = &mensaje; // Prestamo 1
    let lector2 = &mensaje; // Prestamo 2
    let lector3 = &mensaje; // Prestamo 3

    // Todos pueden leerlo al mismo tiempo sin problema
    println!("Lector 1 ve: {}", lector1);
    println!("Lector 2 ve: {lector2}");
    println!("Lector 3 ve: {lector3}");

    println!("El mensaje original es: {mensaje}");

    // 2.- Prestamo mutable
    // La varuable original TIENE que ser mutable para poder prestarla con permiso de escritura
    let mut jugador = String::from("Edwin");

    // Le prestamos el dato al 'editor' de forma exclusiva
    let editor = &mut jugador;

    // El editor le agrega texto al dato original
    editor.push_str(" ZO");
    println!("El editor lo dejo asi: {jugador}");

    // Una vez que editor dejo de usarse, el dueño original puede volver a verlo.
    println!("El dueño final tiene: {jugador}");


}