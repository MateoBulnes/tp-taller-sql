use tp_taller_sql::parser::parsear_query;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        if let Some(programa) = args.first() {
            eprintln!("Error - Cantidad incorrecta de argumentos. Debe ingresar: {} con [ruta a tablas] [consulta]", programa);
        }
        return;
    }

    let query = args[2].trim().to_lowercase();

    match parsear_query(&query) {
        Ok(query_parseada) => query_parseada.mostrar_operacion(),
        Err(e) => {
            e.imprimir_error();
        }
    }
}
