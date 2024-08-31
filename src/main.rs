use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Verificamos que se hayan pasado exactamente 3 argumentos (el nombre del programa y dos argumentos)
    if args.len() != 3 {
        println!("Error: Cantidad de argumentos incorrectos");
        return();
    }

    // Accedemos a los argumentos
    let ruta_tablas = &args[1];
    let query = &args[2];

    // Imprimimos los argumentos
    println!("Ruta ingresada: {}", ruta_tablas);
    println!("Query ingresada: {}", query);
}
