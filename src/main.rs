use std::{collections::HashMap, env};
use tp_taller_sql::{errores::Error, sql::OperacionSql};

//use tp_taller_sql::sql::OperacionSql;

fn parsear_where(sentencia: &str, consulta: &mut HashMap<String, String>) -> Result<(), Error> {
    let partes_div_orderby: Vec<&str> = sentencia.trim().split("order by").collect();

    let clausula_where = partes_div_orderby[0].to_string();
    if clausula_where.trim() == "" {
        return Err(Error::new_invalid_syntax_error(String::from(
            "Debe ingresar al menos una condicion para realizar la consulta con WHERE",
        )));
    }
    consulta.insert(String::from("where"), clausula_where);

    Ok(())
}

fn parsear_tabla(tabla: &str, consulta: &mut HashMap<String, String>) -> Result<(), Error> {
    if tabla.trim() == "" {
        return Err(Error::new_invalid_syntax_error(String::from(
            "Debe ingresar una tabla para la consulta",
        )));
    }
    consulta.insert(String::from("tabla"), tabla.to_string());

    Ok(())
}

fn parsear_order_by(
    clausula_orderby: &str,
    consulta: &mut HashMap<String, String>,
) -> Result<(), Error> {
    if clausula_orderby.trim() == "" {
        return Err(Error::new_invalid_syntax_error(String::from(
            "Debe ingresar una columna para ordenar",
        )));
    }
    consulta.insert(String::from("order_by"), clausula_orderby.to_string());

    Ok(())
}

fn parsear_from(
    sentencia: &str,
    consulta_select: &mut HashMap<String, String>,
) -> Result<(), Error> {
    let parte_from = sentencia.trim();
    if parte_from.trim().to_string() == "" {
        return Err(Error::new_invalid_syntax_error(String::from(
            "Debe ingresar una tabla para la consulta",
        )));
    }
    //Divido por "WHERE"
    let partes_div_where: Vec<&str> = parte_from.split("where").collect();

    let partes_div_orderby: Vec<&str>;
    //Si tengo 2 o mas es porque hay clausula where
    if partes_div_where.len() >= 2 {
        parsear_tabla(partes_div_where[0], consulta_select)?;
        parsear_where(partes_div_where[1].trim(), consulta_select)?;
        partes_div_orderby = partes_div_where[1].trim().split("order by").collect();
    } else {
        partes_div_orderby = parte_from.split("order by").collect();
        parsear_tabla(partes_div_orderby[0], consulta_select)?;
    }

    if partes_div_orderby.len() >= 2 {
        parsear_order_by(partes_div_orderby[1], consulta_select)?;
    }

    return Ok(());
}

fn crear_select(consulta: &mut HashMap<String, String>) -> Result<OperacionSql, Error> {
    let columnas = match consulta.remove("columnas") {
        Some(c) => c
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>(),
        None => {
            return Err(Error::new_invalid_syntax_error(String::from(
                "Debe definir columnas a seleccionar",
            )))
        }
    };

    let nombre_tabla = match consulta.remove("tabla") {
        Some(t) => t,
        None => {
            return Err(Error::new_invalid_syntax_error(String::from(
                "Debe definir la tabla de la consulta",
            )))
        }
    };

    // Procesar los valores opcionales de 'where' y 'order_by'
    let clausula_where = consulta.remove("where");
    let clausula_orderby = consulta.remove("order_by");

    Ok(OperacionSql::new_select(
        columnas,
        nombre_tabla,
        clausula_where,
        clausula_orderby,
    ))
}

fn parsear_select(query: &str) -> Result<OperacionSql, Error> {
    let mut consulta_select: HashMap<String, String> = HashMap::new();

    //Divido por "FROM"
    let partes_div_from: Vec<&str> = query.split("from").collect();

    //Si hay 1 o menos partes, no hay from
    if partes_div_from.len() < 2 {
        return Err(Error::new_invalid_syntax_error(String::from(
            "Select debe tener clausula FROM",
        )));
    }

    //Me quedo solo con las columnas y agrego a la consulta
    let columnas = partes_div_from[0].replace("select", "");

    if columnas.trim() == "" {
        return Err(Error::new_invalid_syntax_error(String::from(
            "Debe ingresar columnas en la consulta",
        )));
    }
    consulta_select.insert(String::from("columnas"), columnas);

    parsear_from(partes_div_from[1], &mut consulta_select)?;

    let select: OperacionSql = crear_select(&mut consulta_select)?;

    Ok(select)
}

fn parsear_update(query: &str) -> Result<OperacionSql, Error> {
    let mut consulta_update: HashMap<String, String> = HashMap::new();

    let partes_div_set: Vec<&str> = query.split("set").collect();
    if partes_div_set.len() < 2{
        return Err(Error::new_invalid_syntax_error(String::from("Update debe tener la clausula SET")));
    }

    let tabla = partes_div_set[0].replace("update", "");
    if tabla.trim().is_empty(){
        return Err(Error::new_invalid_syntax_error(String::from("Debe especificar una tabla para actualizar")));
    }

    consulta_update.insert(String::from("tabla"), tabla);

    let parte_where: Vec<&str> = partes_div_set[1].split("where").collect();
    if parte_where.len() < 2{
        if parte_where[0].trim().is_empty(){
            return Err(Error::new_invalid_syntax_error(String::from("Debe incluir las modificaciones a realizar")));
        }
        consulta_update.insert(String::from("modificaciones"), String::from(parte_where[0]));
    } else{
        if parte_where[0].trim().is_empty(){
            return Err(Error::new_invalid_syntax_error(String::from("Debe incluir las modificaciones a realizar")));
        }
        if parte_where[1].trim().is_empty(){
            return Err(Error::new_invalid_syntax_error(String::from("Debe incluir las condiciones para el WHERE")));
        }
        consulta_update.insert(String::from("modificaciones"), String::from(parte_where[0]));
        consulta_update.insert(String::from("where"), String::from(parte_where[1]));
    }

    Ok(OperacionSql::Update { set: vec!["holis".to_string()], nombre_tabla: "holis".to_string(), clausula_where: Some("Holis".to_string()) })
}

fn parsear_query(query: &str) -> Result<OperacionSql, Error> {
    let query = query.trim().to_lowercase();

    if query.starts_with("select") {
        let resultado = parsear_select(&query)?;
        return Ok(resultado);
    }
    else if query.starts_with("update"){
        let resultado = parsear_update(&query)?;
        return Ok(resultado);
    }
    /*else if query.starts_with("insert"){
        let resultado = procesar_insert(&update)?;
        return Ok(resultado);
    }
    else if query.starts_with("delete"){
        let resultado = procesar_delete(&query)?;
        return Ok(resultado);
    } */
    else {
        return Err(Error::new_invalid_syntax_error(String::from(
            "La sentencia debe tener una operacion SQL definida",
        )));
    }
}

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
        Ok(query_parseada) => {query_parseada.mostrar_operacion()}
        Err(e) => {
            e.imprimir_error();
            return;
        }
    }
}
