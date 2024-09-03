use std::collections::HashMap;

use crate::{errores::Error, sql::OperacionSql};

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
    if parte_from.trim() == "" {
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

    Ok(())
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

    let select: OperacionSql = OperacionSql::crear_select(&mut consulta_select)?;

    Ok(select)
}

fn parsear_update(query: &str) -> Result<OperacionSql, Error> {
    let mut consulta_update: HashMap<String, String> = HashMap::new();

    let partes_div_set: Vec<&str> = query.split("set").collect();
    if partes_div_set.len() < 2 {
        return Err(Error::new_invalid_syntax_error(String::from(
            "Update debe tener la clausula SET",
        )));
    }

    let tabla = partes_div_set[0].replace("update", "");
    if tabla.trim().is_empty() {
        return Err(Error::new_invalid_syntax_error(String::from(
            "Debe especificar una tabla para actualizar",
        )));
    }

    consulta_update.insert(String::from("tabla"), tabla);

    let parte_where: Vec<&str> = partes_div_set[1].split("where").collect();
    if parte_where.len() < 2 {
        if parte_where[0].trim().is_empty() {
            return Err(Error::new_invalid_syntax_error(String::from(
                "Debe incluir las modificaciones a realizar",
            )));
        }
        consulta_update.insert(String::from("modificaciones"), String::from(parte_where[0]));
    } else {
        if parte_where[0].trim().is_empty() {
            return Err(Error::new_invalid_syntax_error(String::from(
                "Debe incluir las modificaciones a realizar",
            )));
        }
        if parte_where[1].trim().is_empty() {
            return Err(Error::new_invalid_syntax_error(String::from(
                "Debe incluir las condiciones para el WHERE",
            )));
        }
        consulta_update.insert(String::from("modificaciones"), String::from(parte_where[0]));
        consulta_update.insert(String::from("where"), String::from(parte_where[1]));
    }

    let update: OperacionSql = OperacionSql::crear_update(&mut consulta_update)?;
    Ok(update)
}

fn extraer_contenido_parentesis(s: &str) -> Result<&str, &str> {
    if let Some(inicio) = s.find('(') {
        if let Some(fin) = s.find(')') {
            if inicio < fin {
                return Ok(&s[inicio + 1..fin]);
            }
        }
    }
    Err("El formato de las columnas o valores no es correcto, deben ir entre parentesis")
}

fn parsear_insert(query: &str) -> Result<OperacionSql, Error> {
    let mut consulta_insert: HashMap<String, String> = HashMap::new();

    let partes_div_values: Vec<&str> = query.split("values").collect();
    if partes_div_values.len() < 2 {
        return Err(Error::new_invalid_syntax_error(String::from(
            "Debe completar el INSERT con valores para las columnas",
        )));
    }

    match extraer_contenido_parentesis(partes_div_values[1]) {
        Ok(contenido) => {
            consulta_insert.insert(String::from("valores"), String::from(contenido));
        }
        Err(e) => return Err(Error::new_invalid_syntax_error(String::from(e))),
    }

    match extraer_contenido_parentesis(partes_div_values[0]) {
        Ok(contenido) => {
            consulta_insert.insert(String::from("columnas"), String::from(contenido));
            println!("Resultado de la busqueda de columnas: {}", contenido);
        }
        Err(e) => return Err(Error::new_invalid_syntax_error(String::from(e))),
    }

    let div_insert: Vec<&str> = partes_div_values[0].split("(").collect();
    consulta_insert.insert(
        String::from("tabla"),
        String::from(div_insert[0].replace("insert into", "").trim()),
    );

    let insert = OperacionSql::crear_insert(&mut consulta_insert)?;

    Ok(insert)
}

fn parsear_delete(query: &str) -> Result<OperacionSql, Error> {
    let mut consulta_delete: HashMap<String, String> = HashMap::new();

    let partes_div_where: Vec<&str> = query.split("where").collect();
    if partes_div_where.len() >= 2 {
        if partes_div_where[1].trim().is_empty(){
            return Err(Error::new_invalid_syntax_error(String::from("Debe incluir al menos una condicion para usar WHERE")));
        }
        consulta_delete.insert(String::from("where"), String::from(partes_div_where[1]));
    } 

    if partes_div_where[0].trim().is_empty(){
        return Err(Error::new_invalid_syntax_error(String::from("Debe indicar de que tabla desea hacer DELETE")));
    }

    let partes_div_from: Vec<&str> = partes_div_where[0].split("from").collect();

    if partes_div_from[1].trim().is_empty(){
        return Err(Error::new_invalid_syntax_error(String::from("Debe indicar de que tabla desea hacer DELETE")));
    }

    consulta_delete.insert(String::from("tabla"), partes_div_where[0].replace("delete from", ""));

    let delete = OperacionSql::crear_delete(&mut consulta_delete)?;

    Ok(delete)
}


pub fn parsear_query(query: &str) -> Result<OperacionSql, Error> {
    let query = query.trim().to_lowercase();

    if query.starts_with("select") {
        let resultado = parsear_select(&query)?;
        Ok(resultado)
    } else if query.starts_with("update") {
        let resultado = parsear_update(&query)?;
        return Ok(resultado);
    } else if query.starts_with("insert into") {
        let resultado = parsear_insert(&query)?;
        return Ok(resultado);
    } else if query.starts_with("delete from") {
        let resultado = parsear_delete(&query)?;
        return Ok(resultado);
    } else {
        return Err(Error::new_invalid_syntax_error(String::from(
            "La sentencia debe tener una operacion SQL definida",
        )));
    }
}