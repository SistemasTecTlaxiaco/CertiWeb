#![no_std] // No usamos la biblioteca estándar de Rust

use soroban_sdk::{contract, contractimpl, Env, String, Map, symbol_short};



/// Definimos la clave de almacenamiento como una constante para evitar errores tipográficos
const DOCUMENTS_KEY: soroban_sdk::Symbol = symbol_short!("DOCUMENTS");
const HISTORIAL_KEY: soroban_sdk::Symbol = symbol_short!("HISTORIAL");
const RESPALDOS_KEY: soroban_sdk::Symbol = symbol_short!("RESPALDOS");
const USUARIOS_KEY: soroban_sdk::Symbol = symbol_short!("USUARIOS");
const NOTIFICAS_KEY: soroban_sdk::Symbol = symbol_short!("NOTIFICA");
const SESIONES_KEY: soroban_sdk::Symbol = symbol_short!("SESIONES");
const INFORMES_KEY: soroban_sdk::Symbol = symbol_short!("INFORMES");
const EXPORTAC_KEY: soroban_sdk::Symbol = symbol_short!("EXPORTAC");
const INTEGRA_KEY: soroban_sdk::Symbol = symbol_short!("INTEGRA");

#[contract]
pub struct DocumentosContract;

#[contractimpl]
impl DocumentosContract {
    // --- CREATE (Dar de alta un documento) ---
    /// Añade un nuevo documento al registro.
    /// Panics si el documento con el mismo ID ya existe.
    ///
    /// # Arguments
    ///
    /// * `id_documento` - El ID del documento (i32).
    /// * `titulo` - El título del documento (String).
    /// * `estado` - El estado del documento (String).
    /// * `fecha` - La fecha de creación del documento (u64).
    pub fn registrar_documento(env: Env, id_documento: i32, titulo: String, estado: String, fecha: u64) {
        // Obtenemos el mapa de documentos del almacenamiento persistente.
        // Si no existe, crea un mapa vacío.
        let mut documentos: Map<i32, (String, String, u64)> = env
            .storage()
            .persistent()
            .get(&DOCUMENTS_KEY)
            .unwrap_or(Map::new(&env)); // Crea un mapa vacío si no hay nada

        // Verificamos si el documento ya existe para evitar duplicados
        if documentos.contains_key(id_documento) {
            panic!("Documento con el mismo ID ya existe"); // Error si el documento ya está registrado
        }

        // Añadimos el nuevo documento con sus detalles (titulo, estado y fecha)
        documentos.set(id_documento, (titulo, estado, fecha));
        // Guardamos el mapa actualizado en el almacenamiento persistente
        env.storage().persistent().set(&DOCUMENTS_KEY, &documentos);
    }
 //metodo para obtener todos los documentos   
    pub fn obtener_documento(env: Env, id_documento: i32) -> Option<(String, String, u64)> {
        let documentos: Map<i32, (String, String, u64)> = env
            .storage()
            .persistent()
            .get(&DOCUMENTS_KEY)
            .unwrap_or(Map::new(&env));
    
        documentos.get(id_documento)
    }
//metodo para realizar una consulta al historial   /// Añade una entrada al historial de un documento.
    pub fn consulta_historial(env: Env, id_historial: i32, fecha: u64, resultado: String) {
        let mut historial: Map<i32, (u64, String)> = env
            .storage()
            .persistent()
            .get(&HISTORIAL_KEY)
            .unwrap_or(Map::new(&env));

        if historial.contains_key(id_historial) {
            panic!("Historial con el mismo ID ya existe");
        }

        historial.set(id_historial, (fecha, resultado));
        env.storage().persistent().set(&HISTORIAL_KEY, &historial);
    }
    //  --- Obtener una entrada del historial por ID ---
    pub fn obtener_historial(env: Env, id_historial: i32) -> Option<(u64, String)> {
        let historial: Map<i32, (u64, String)> = env
            .storage()
            .persistent()
            .get(&HISTORIAL_KEY)
            .unwrap_or(Map::new(&env));

        historial.get(id_historial)
    }
// --- Realizar un respaldo de un documento --- 
    /// Añade un respaldo de un documento al registro.
    pub fn realizar_respaldo(
        env: Env,
        id_respaldo: i32,
        fecha: u64,
        ubicacion: String,
        autor: String,
    ) {
        let mut respaldos: Map<i32, (u64, String, String)> = env
            .storage()
            .persistent()
            .get(&RESPALDOS_KEY)
            .unwrap_or(Map::new(&env));
    
        if respaldos.contains_key(id_respaldo) {
            panic!("Respaldo con ese ID ya existe");
        }  
        respaldos.set(id_respaldo, (fecha, ubicacion, autor));
        env.storage().persistent().set(&RESPALDOS_KEY, &respaldos);
    }
 // --- Obtener un respaldo por ID ---
    /// Devuelve un respaldo de un documento por su ID.
    /// Devuelve `None` si no existe.   
    pub fn consultar_respaldo(env: Env, id_respaldo: i32) -> Option<(u64, String, String)> {
        let respaldos: Map<i32, (u64, String, String)> = env
            .storage()
            .persistent()
            .get(&RESPALDOS_KEY)
            .unwrap_or(Map::new(&env));
    
        respaldos.get(id_respaldo)
    }





// ConsultaDocumento
// pub fn buscar_documentos(env: Env, criterio: String) -> Vec<(i32, String, String, u64)> {
//     let mut resultados: Vec<(i32, String, String, u64)> = Vec::new(&env);

//     let documentos = DocumentosContract::obtener_todos_documentos(&env);
//     let criterio_bytes = criterio.to_val().to_bytes();  // Reemplazar con el método adecuado

//     for (id, titulo, estado, timestamp) in documentos {
//         let titulo_bytes = titulo.to_val().to_bytes();  // Usar to_bytes o equivalente
//         let estado_bytes = estado.to_val().to_bytes();  // Lo mismo aquí

//         if DocumentosContract::bytes_contains(&titulo_bytes.as_slice(), &criterio_bytes.as_slice()) ||
//            DocumentosContract::bytes_contains(&estado_bytes.as_slice(), &criterio_bytes.as_slice()) {
//             resultados.push((id, titulo, estado, timestamp));
//         }
//     }
//     resultados
// }


// // Función auxiliar para comparar bytes
// fn bytes_contains(haystack: &[u8], needle: &[u8]) -> bool {
//     haystack.windows(needle.len()).any(|window| window == needle)
// }

// pub fn obtener_todos_documentos(env: &Env) -> Vec<(i32, String, String, u64)> {
//     let documentos: Map<i32, (String, String, u64)> = env
//         .storage()
//         .persistent()
//         .get(&DOCUMENTS_KEY)
//         .unwrap_or(Map::new(env));
    
//     let mut result = Vec::new(env);
//     for (id, (titulo, estado, fecha)) in documentos {
//         result.push_back((id, titulo, estado, fecha));
//     }
//     result
// }
// }






// pub fn mostrar_resultados(env: Env) -> Vec<(i32, soroban_sdk::String, soroban_sdk::String, u64)> {
//     let mut resultados: Vec<(i32, String, String, u64)> = Vec::new();
//     for (id, titulo, estado, timestamp) in DocumentosContract::obtener_todos_documentos(&env) {
// // Suponiendo que no necesitas to_bytes()
// resultados.push((id, titulo, estado, timestamp));
//     }
//     resultados
// }



// ValidacionDocumento
pub fn verificar_documento(env: Env, id_documento: i32, nuevo_resultado: String) {
    let mut historial: Map<i32, (u64, String)> = env
        .storage()
        .persistent()
        .get(&HISTORIAL_KEY)
        .unwrap_or(Map::new(&env));

    let fecha_actual = env.ledger().timestamp();
    historial.set(id_documento, (fecha_actual, nuevo_resultado));
    env.storage().persistent().set(&HISTORIAL_KEY, &historial);
}

pub fn actualizar_estado(env: Env, id_documento: i32, nuevo_estado: String) {
    let mut documentos: Map<i32, (soroban_sdk::String, soroban_sdk::String, u64)> = env
        .storage()
        .persistent()
        .get(&DOCUMENTS_KEY)
        .unwrap_or(Map::new(&env));

    if let Some((titulo, _, fecha)) = documentos.get(id_documento) {
        documentos.set(id_documento, (titulo.clone(), nuevo_estado, fecha));
        env.storage().persistent().set(&DOCUMENTS_KEY, &documentos);
    } else {
        panic!("Documento no encontrado");
    }
}

// GestionUsuarios
pub fn crear_usuario(env: Env, id_usuario: i32, nombre: String, rol: String) {
    let mut usuarios: Map<i32, (String, String)> = env
        .storage()
        .persistent()
        .get(&USUARIOS_KEY)
        .unwrap_or(Map::new(&env));

    if usuarios.contains_key(id_usuario) {
        panic!("Usuario ya existe");
    }

    usuarios.set(id_usuario, (nombre, rol));
    env.storage().persistent().set(&USUARIOS_KEY, &usuarios);
}

pub fn asignar_rol(env: Env, id_usuario: i32, nuevo_rol: String) {
    let mut usuarios: Map<i32, (String, String)> = env
        .storage()
        .persistent()
        .get(&USUARIOS_KEY)
        .unwrap_or(Map::new(&env));

    if let Some((nombre, _)) = usuarios.get(id_usuario) {
        usuarios.set(id_usuario, (nombre, nuevo_rol));
        env.storage().persistent().set(&USUARIOS_KEY, &usuarios);
    } else {
        panic!("Usuario no encontrado");
    }
}

// ConfiguracionNotificaciones
pub fn configurar_notificacion(env: Env, id_notificacion: i32, tipo: String) {
    let mut notificaciones: Map<i32, String> = env
        .storage()
        .persistent()
        .get(&NOTIFICAS_KEY)
        .unwrap_or(Map::new(&env));

    notificaciones.set(id_notificacion, tipo);
    env.storage().persistent().set(&NOTIFICAS_KEY, &notificaciones);
}

pub fn enviar_notificacion(env: Env, id_notificacion: i32) -> Option<String> {
    let notificaciones: Map<i32, String> = env
        .storage()
        .persistent()
        .get(&NOTIFICAS_KEY)
        .unwrap_or(Map::new(&env));

    notificaciones.get(id_notificacion)
}

// AutenticacionAutorizacion
pub fn autenticar_usuario(env: Env, id_sesion: i32, credenciales: String) {
    let mut sesiones: Map<i32, String> = env
        .storage()
        .persistent()
        .get(&SESIONES_KEY)
        .unwrap_or(Map::new(&env));

    sesiones.set(id_sesion, credenciales);
    env.storage().persistent().set(&SESIONES_KEY, &sesiones);
}

pub fn asignar_permisos(env: Env, id_sesion: i32, permisos: String) {
    let mut sesiones: Map<i32, String> = env
        .storage()
        .persistent()
        .get(&SESIONES_KEY)
        .unwrap_or(Map::new(&env));

    if sesiones.contains_key(id_sesion) {
        sesiones.set(id_sesion, permisos);
        env.storage().persistent().set(&SESIONES_KEY, &sesiones);
    } else {
        panic!("Sesión no encontrada");
    }
}

// GeneracionInformes
pub fn generar_informe(env: Env, id_informe: i32, formato: String) {
    let mut informes: Map<i32, (u64, String)> = env
        .storage()
        .persistent()
        .get(&INFORMES_KEY)
        .unwrap_or(Map::new(&env));

    let fecha_generacion = env.ledger().timestamp();
    informes.set(id_informe, (fecha_generacion, formato));
    env.storage().persistent().set(&INFORMES_KEY, &informes);
}

pub fn exportar_informe(env: Env, id_informe: i32) -> Option<(u64, String)> {
    let informes: Map<i32, (u64, String)> = env
        .storage()
        .persistent()
        .get(&INFORMES_KEY)
        .unwrap_or(Map::new(&env));

    informes.get(id_informe)
}

// ExportacionDatos
pub fn exportar_datos(env: Env, id_exportacion: i32, formato: String) {
    let mut exportaciones: Map<i32, String> = env
        .storage()
        .persistent()
        .get(&EXPORTAC_KEY)
        .unwrap_or(Map::new(&env));

    exportaciones.set(id_exportacion, formato);
    env.storage().persistent().set(&EXPORTAC_KEY, &exportaciones);
}

pub fn validar_exportacion(env: Env, id_exportacion: i32) -> Option<String> {
    let exportaciones: Map<i32, String> = env
        .storage()
        .persistent()
        .get(&EXPORTAC_KEY)
        .unwrap_or(Map::new(&env));

    exportaciones.get(id_exportacion)
}

// IntegracionSistemaAcademico
pub fn sincronizar_datos(env: Env, id_integracion: i32, nombre_sistema: String) {
    let mut integraciones: Map<i32, String> = env
        .storage()
        .persistent()
        .get(&INTEGRA_KEY)
        .unwrap_or(Map::new(&env));

    integraciones.set(id_integracion, nombre_sistema);
    env.storage().persistent().set(&INTEGRA_KEY, &integraciones);
}

pub fn verificar_integracion(env: Env, id_integracion: i32) -> Option<String> {
    let integraciones: Map<i32, String> = env
        .storage()
        .persistent()
        .get(&INTEGRA_KEY)
        .unwrap_or(Map::new(&env));

    integraciones.get(id_integracion)
}

}

#[cfg(test)]
mod test;
