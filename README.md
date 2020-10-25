[![Rust-Build Actions Status](https://github.com/phideg/rusty-web-server/workflows/Rust/badge.svg)](https://github.com/phideg/rusty-web-server/actions)


# rusty-web-server
A very simple static file web server written in [Rust](www.rust-lang.org) based on [tokio-minihttp](https://github.com/tokio-rs/tokio-minihttp)

## Motivation
Why another basic web server? It should be a really minimal web server - just enough to locally serve [reveal.js](https://github.com/hakimel/reveal.js) slides. This project is basically a simpler version of [basic-http-server](https://github.com/brson/basic-http-server). Rotor was replaced by tokio-minihttp.

## How to build
Install latest stable version of [Rust](https://www.rust-lang.org/en-US/install.html).

1. Open the command line of your choice. 
2. Clone this repository
```
# > git clone git@github.com:phideg/rusty-web-server.git
```
3. Change into the projects directory
```
# > cd rusty-web-server
```
4. Execute the build command
```
# > cargo build --release
```
5. After successful build you will find the rusty-web-server executable in the `target/release` directory

## How to use it
Copy the rusty-web-server executable into the root folder of the static web content you want to serve and start the server.

USAGE:  
    rusty-web-server.exe [ADDR]  

FLAGS:  
    -h, --help       Prints help information  
    -V, --version    Prints version information  
  
ARGS:  
    <ADDR>    Sets the IP:PORT combination (default "127.0.0.1:8080")  

## License

MIT


