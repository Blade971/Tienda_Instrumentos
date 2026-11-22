Tienda de Instrumentos en Solana (Anchor)

Este proyecto es un Smart Contract desarrollado con Anchor en Solana que permite administrar una tienda de instrumentos musicales en la blockchain.

El programa permite:

Crear una tienda

Agregar instrumentos

Eliminar instrumentos

Cambiar disponibilidad de instrumentos

Consultar el inventario

Todo el inventario queda almacenado en una cuenta PDA en la blockchain de Solana.

Tecnologías utilizadas

Rust

Anchor Framework

Solana Blockchain

Funcionalidades del programa

Crear tienda
Permite crear una tienda asociada a un owner (propietario).

Cada tienda guarda:

Nombre de la tienda

Dirección del propietario

Lista de instrumentos

crear_tienda(nombre: String)

Ejemplo:

"Mi tienda musical" Agregar instrumento

Permite agregar un instrumento al inventario de la tienda.

Información guardada:

Marca

Modelo

Precio

Disponibilidad en stock

agregar_instrumento( marca: String, modelo: String, precio: u64, en_stock: bool )

Ejemplo:

Marca: Fender Modelo: Stratocaster Precio: 1500 Stock: true Eliminar instrumento

Permite eliminar un instrumento del inventario usando su modelo.

eliminar_instrumento(modelo: String)

Si el instrumento no existe se lanza un error:

InstrumentoNoEncontrado Cambiar disponibilidad

Cambia el estado de disponibilidad en stock de un instrumento.

Si estaba:

true → false

o

false → true cambiar_disponibilidad(modelo: String) Ver instrumentos

Permite visualizar todos los instrumentos registrados en la tienda.

El resultado se imprime en los logs del programa.

Ejemplo de salida:

Listado de instrumentos en la tienda 'Mi tienda musical':

#1: Fender Stratocaster | Precio: 1500 | Stock: true #2: Gibson Les Paul | Precio: 2200 | Stock: false ver_instrumentos() Estructura de datos TiendaInstrumentos

Cuenta principal que almacena toda la información.

pub struct TiendaInstrumentos { pub nombre: String, pub owner: Pubkey, pub instrumentos: Vec, } Instrumento

Representa un instrumento dentro del inventario.

pub struct Instrumento { pub marca: String, pub modelo: String, pub precio: u64, pub en_stock: bool, } Seguridad

Las operaciones de modificación requieren que el owner firme la transacción.

#[account(mut, has_one = owner)]

Esto evita que otros usuarios puedan modificar la tienda.

PDA utilizada

La tienda se guarda en una Program Derived Address (PDA) usando:

seeds = ["tienda", owner]

Esto asegura que cada propietario tenga su propia tienda única.

Gracias por leer!! By Blade971
Esto asegura que cada propietario tenga su propia tienda única.

Gracias por leer!! By Blade971
