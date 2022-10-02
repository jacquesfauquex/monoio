

# httpdicom monoio micro servicios

## Objetivos

### Micro servicios muy especializados

La reescritura de httpdicom en RUST es ocasión de implementar una orquestación de micro servicios muy especializados. Muy especializado en este contexto significa por ejemplo un micro servicio especializado en operaciones SELECT desde una base de datos específica. Tal grado de especialización facilita la definición de APIs minimalistas de consumo, de tal forma que el procesamiento pueda empezar inmediatamente, sin parseo sofisticado del request.

### No blocante

Cada micro servicio está escrito con código no bloquante en cuanto a ciclos de procesadores. Para lograr eso, adoptamos rust monoio como runtime de micro servicio, en particular porque adopta la arquitectura de kernel no blocante io_uring disponible en los  linux recientes.

### Servidor permanente

Cada servicio es permante, para evitar la repetición de los costos de inicialización y finalización.

### TCP RPC

Cada servicio es de tipo RPC echo y usa el protocolo TCP de la siguiente forma :

- client conexión 
-  client request stream in
-  server response stream out
-  server conexión closing

### Stateless

Cada servicio es absolutamente stateless.

### No seguro

Los micro servicios trabajan aislados de internet sin complicación relativa a seguridad, para favorecer al celeridad.

La seguridad tiene que implementarse en los proxy frontends expuestos a internet.



## Categorías de micro servicio

- sql query
- read/write from ssd
- streamed processing



### Excluidos de la arquitectura de micro servicios 

#### HTTP

El consumo http está excluido de los micro servicios. Para eso usamos curl hyper directamente desde rust.

- https://www.youtube.com/watch?v=HFH2vZRTKrA
- https://www.youtube.com/watch?v=okGUxW_i9yk&t=2103s
- https://daniel.haxx.se/blog/2020/10/09/rust-in-curl-with-hyper/
- https://github.com/curl/curl/blob/master/docs/HYPER.md
- https://github.com/hyperium/hyper
- https://github.com/hyperium/hyper/blob/master/examples/client.rs
- https://hyper.rs/guides/
- https://hyper.rs/guides/client/basic/

Existe un crate "curl", pero la integración es menos profunda que con "hyper".

https://docs.rs/curl/latest/curl/

#### XSLT
Usamos para eso un servico tcp provisto por node.js que integra saxon-js 2 y transformadores compilados a formato sef.

- node.js se descarga con package instalador para mac. Luego instalar via npm saxon-js y xslt3
- https://www.saxonica.com/saxon-js/documentation2/index.html#!starting/installing-nodejs
- finalmente, crear un server.js (https://nodejs.org/en/docs/guides/getting-started-guide/)
```javascript
const http = require('http');

const hostname = '127.0.0.1';
const port = 3000;

const server = http.createServer((req, res) => {
  res.statusCode = 200;
  res.setHeader('Content-Type', 'text/plain');
  res.end('Hello World');
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
```
- y ejecutarlo
```sh
$ node server.js
Server running at http://127.0.0.1:3000/
```



- https://www.saxonica.com/saxon-js/documentation2/index.html

## Prototipo

### Servidor de echo build

```sh
cd MONOIO/examples
cargo run --color=always --example rpc_one_echo --release or
```

### Código servidor

```rust
//! An echo example.
//!
//! Run the example and `nc 127.0.0.1 50002` in another shell.
//! All your input will be echoed out.

use monoio::{
    io::{AsyncReadRent, AsyncWriteRentExt},
    net::{TcpListener, TcpStream},
};

#[monoio::main(driver = "fusion")]
async fn main() {
    // tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE).init();
    let listener = TcpListener::bind("127.0.0.1:50002").unwrap();
    println!("listening");
    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((stream, addr)) => {
                println!("accepted a connection from {}", addr);
                monoio::spawn(echo(stream));
            }
            Err(e) => {
                println!("accepted connection failed: {}", e);
                return;
            }
        }
    }
}

async fn echo(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf: Vec<u8> = Vec::with_capacity(8 * 1024);
    loop {
        // read
        let (res, _buf) = stream.read(buf).await;
        buf = _buf;
        let res: usize = res?;
        if res == 0 {
            return Ok(());
        }

        // write all
        let (res, _buf) = stream.write_all(buf).await;
        buf = _buf;
        res?;

        // clear
        buf.clear();
    }
}
```

## Cliente

### nc
```sh
nc ip port
```

### bash
```bash
nc ip port
```

### monoio
```rust
nc ip port
```

