pub enum OperacionSql {
    Select{
        columnas: Vec<String>,
        nombre_tabla: String,
        clausula_where: Option<String>,
        clausula_orderby: Option<String> 
    },
    Update{
        set: Vec<String>,
        nombre_tabla: String,
        clausula_where: Option<String>
    },
    Insert{
        nombre_tabla: String,
        columnas: Vec<String>,
        valores: Vec<String>
    },
    Delete{
        nombre_tabla: String,
        clausula_where: Option<String>
    }
}

impl OperacionSql{
    pub fn new_select(columnas: Vec<String>, nombre_tabla: String, clausula_where: Option<String>, clausula_orderby: Option<String>) -> Self{
        OperacionSql::Select { columnas, nombre_tabla, clausula_where, clausula_orderby }
    }

    pub fn new_update(set: Vec<String>, nombre_tabla: String, clausula_where: Option<String>) -> Self{
        OperacionSql::Update { set, nombre_tabla, clausula_where }
    }

    pub fn new_insert(nombre_tabla: String, columnas: Vec<String>, valores: Vec<String>) -> Self{
        OperacionSql::Insert { nombre_tabla, columnas, valores }
    }

    pub fn new_delete(nombre_tabla: String, clausula_where: Option<String>) -> Self{
        OperacionSql::Delete { nombre_tabla, clausula_where }
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