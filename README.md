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
