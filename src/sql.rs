use std::collections::HashMap;

use crate::errores::Error;

pub enum OperacionSql {
    Select {
        columnas: Vec<String>,
        nombre_tabla: String,
        clausula_where: Option<String>,
        clausula_orderby: Option<String>,
    },
    Update {
        set: Vec<String>,
        nombre_tabla: String,
        clausula_where: Option<String>,
    },
    Insert {
        nombre_tabla: String,
        columnas: Vec<String>,
        valores: Vec<String>,
    },
    Delete {
        nombre_tabla: String,
        clausula_where: Option<String>,
    },
}

impl OperacionSql {
    fn new_select(
        columnas: Vec<String>,
        nombre_tabla: String,
        clausula_where: Option<String>,
        clausula_orderby: Option<String>,
    ) -> Self {
        OperacionSql::Select {
            columnas,
            nombre_tabla,
            clausula_where,
            clausula_orderby,
        }
    }

    fn new_update(
        set: Vec<String>,
        nombre_tabla: String,
        clausula_where: Option<String>,
    ) -> Self {
        OperacionSql::Update {
            set,
            nombre_tabla,
            clausula_where,
        }
    }

    fn new_insert(nombre_tabla: String, columnas: Vec<String>, valores: Vec<String>) -> Self {
        OperacionSql::Insert {
            nombre_tabla,
            columnas,
            valores,
        }
    }

    fn new_delete(nombre_tabla: String, clausula_where: Option<String>) -> Self {
        OperacionSql::Delete {
            nombre_tabla,
            clausula_where,
        }
    }

    pub fn crear_select(consulta: &mut HashMap<String, String>) -> Result<OperacionSql, Error> {
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

    pub fn crear_update(consulta: &mut HashMap<String, String>) -> Result<OperacionSql, Error> {
        let modificaciones = match consulta.remove("modificaciones") {
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

        Ok(OperacionSql::new_update(
            modificaciones,
            nombre_tabla,
            clausula_where,
        ))
    }

    pub fn crear_insert(consulta: &mut HashMap<String, String>) -> Result<OperacionSql, Error> {
        let columnas = match consulta.remove("columnas") {
            Some(c) => c
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>(),
            None => {
                return Err(Error::new_invalid_syntax_error(String::from(
                    "Debe definir columnas donde insertar",
                )))
            }
        };

        let valores = match consulta.remove("valores") {
            Some(c) => c
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>(),
            None => {
                return Err(Error::new_invalid_syntax_error(String::from(
                    "Debe definir valores a insertar",
                )))
            }
        };

        if columnas.len() != valores.len() {
            return Err(Error::new_invalid_syntax_error(String::from(
                "La cantidad de valores debe ser igual a la cantidad de columnas",
            )));
        }

        let nombre_tabla = match consulta.remove("tabla") {
            Some(t) => t,
            None => {
                return Err(Error::new_invalid_syntax_error(String::from(
                    "Debe definir la tabla en la que insertar",
                )))
            }
        };

        Ok(OperacionSql::new_insert(nombre_tabla, columnas, valores))
    }

    pub fn crear_delete(consulta: &mut HashMap<String, String>) -> Result<OperacionSql, Error> {
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

        Ok(OperacionSql::new_delete(nombre_tabla, clausula_where))
    }

    pub fn mostrar_operacion(&self) {
        match self {
            OperacionSql::Select {
                columnas,
                nombre_tabla,
                clausula_where,
                clausula_orderby,
            } => {
                println!("SELECT:");
                println!("Columnas: {:?}", columnas);
                println!("Nombre Tabla: {}", nombre_tabla);
                if let Some(where_clause) = clausula_where {
                    println!("WHERE: {}", where_clause);
                }
                if let Some(order_by_clause) = clausula_orderby {
                    println!("ORDER BY: {}", order_by_clause);
                }
            }
            OperacionSql::Update {
                set,
                nombre_tabla,
                clausula_where,
            } => {
                println!("UPDATE:");
                println!("SET: {:?}", set);
                println!("Nombre Tabla: {}", nombre_tabla);
                if let Some(where_clause) = clausula_where {
                    println!("WHERE: {}", where_clause);
                }
            }
            OperacionSql::Insert {
                nombre_tabla,
                columnas,
                valores,
            } => {
                println!("INSERT:");
                println!("Nombre Tabla: {}", nombre_tabla);
                println!("Columnas: {:?}", columnas);
                println!("Valores: {:?}", valores);
            }
            OperacionSql::Delete {
                nombre_tabla,
                clausula_where,
            } => {
                println!("DELETE:");
                println!("Nombre Tabla: {}", nombre_tabla);
                if let Some(where_clause) = clausula_where {
                    println!("WHERE: {}", where_clause);
                }
            }
        }
    }
}
