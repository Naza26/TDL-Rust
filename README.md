# Rust-LAND
### Integrantes:
- Lourengo Caridade, Lucia 104880
- Rueda, Nazarena 106280
- Su, Agustina Doly 105708

### Cómo correr la aplicación

Correr server: `cargo run --config_name`

```
cd server
cargo run config.txt
```

Correr client: `cargo run --host --port`

```
cd client
cargo run 127.0.0.1 9244
```

### Aplicación para speed dating

#### Semana 6/11

Diagrama de secuencia para creación de salas

![rooms](avances/land-6_11-rooms.png)

Diagrama de secuencia para threads en el server

![threads](avances/land-6_11-server_threads.png)


# RUST

## Historia de Rust

- Rust surgió de un proyecto personal iniciado en 2006 por Graydon Hoare, empleado de Mozilla Research.
- Mozilla patrocinó oficialmente el proyecto en 2009.
- A partir de 2015, cuando estuvo la primera versión estable, empresas como Amazon, Discord, Dropbox, Meta, Google y Microsoft adoptaron Rust.
- En diciembre de 2022, se convirtió en el primer lenguaje distinto de C y ensamblador compatible con el desarrollo del kernel de Linux.

## Sintaxis

### Hola Mundo!

```rust
fn main() {
    println!("Hello, World!");
}
```
### Macro

### Pattern Matching

La coincidencia de patrones se puede realizar utilizando la palabra clave match.

```rust
fn main() {
    let mut values = vec![1, 2, 3, 4];

    match values.len() {
        0 => println!("Empty"),
        1 => println!("One value"),
        // pattern matching can use ranges of integers
        2..=10 => println!("Between two and ten values"),
        11 => println!("Eleven values"),
        // A `_` pattern is called a "wildcard", it matches any value
        _ => println!("Many values"),
    };
}
```

### Predicate

### Closures

### Types

### Punteros / Referencias

### Azucar Sintáctica

### Ownership

### Lifetimes

### Traits

### Iterators

Los bucles For en Rust funcionan en un estilo funcional como operaciones sobre un tipo de iterador.

```rust
for x in 0..100 {
   f(x);
}
```

### Expression-Oriented

Rust es un lenguaje orientado a la expresión, es decir, cada construcción es una expresión y por consecuencia tiene un valor. 
Se utiliza la expresión if en lugar del condicional ternario de C. Dado que los retornos son implícitos, no es necesario que una función termine con una expresión de retorno; si se omite el punto y coma, el valor de la última expresión de la función se utiliza como valor de retorno

```rust
fn factorial(i: u64) -> u64 {
    if i == 0 {
        1
    } else {
        i * factorial(i - 1)
    }
}
```

### Cargo

### Concurrencia




