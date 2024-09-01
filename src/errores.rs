pub enum Error {
    InvalidTable { codigo: String, descripcion: String },
    InvalidColumn { codigo: String, descripcion: String },
    InvalidSyntax { codigo: String, descripcion: String },
    GenericError { codigo: String, descripcion: String },
}

impl Error {
    pub fn new_invalid_table_error(desc: String) -> Self {
        Error::InvalidTable {
            codigo: String::from("INVALID_TABLE"),
            descripcion: desc,
        }
    }

    pub fn new_invalid_column_error(desc: String) -> Self {
        Error::InvalidColumn {
            codigo: String::from("INVALID_COLUMN"),
            descripcion: desc,
        }
    }

    pub fn new_invalid_syntax_error(desc: String) -> Self {
        Error::InvalidSyntax {
            codigo: String::from("INVALID_SYNTAX"),
            descripcion: desc,
        }
    }

    pub fn new_generic_error(desc: String) -> Self {
        Error::GenericError {
            codigo: String::from("ERROR"),
            descripcion: desc,
        }
    }

    pub fn imprimir_error(&self) {
        match self {
            Error::InvalidTable {
                codigo,
                descripcion,
            }
            | Error::InvalidColumn {
                codigo,
                descripcion,
            }
            | Error::InvalidSyntax {
                codigo,
                descripcion,
            }
            | Error::GenericError {
                codigo,
                descripcion,
            } => {
                eprintln!("{}: {}", codigo, descripcion);
            }
        }
    }
}
