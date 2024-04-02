# Overview
Octo-journey is a test server. You are a rather strange person who hunts for Octopus in the depths of the sea, kidnaps them and then names them. Maybe you don't have any freinds and this is your way of making them? Or perhaps you have some illegal Octopus trading going on? Whatever I don't care. It deliberatly does very little and some requests are sub-optimal as a method of generating load...honest.

# Getting Started

The server is equiped with a CLI with the following attributes:

```
Usage: octo-journey.exe [OPTIONS]

Options:
  -a, --address <address>  Server address [env: OCTO_SERVER_ADDRESS=] [default: 0.0.0.0]       
  -p, --port <port>        Server port [env: OCTO_SERVER_PORT=] [default: 8080]
  -v...                    Set the log level [env: OCTO_SERVER_VERBOSITY=]
  -h, --help               Print help
  -V, --version            Print version
```
It can be ran simply by:

1. Cargo -> `cargo run -- <CLI OPTIONS>` 
2. Docker (if you have build the image) -> `docker run -p 8080:8080 <IMAGE NAME> <CLI OPTIONS> `


# Endpoints

The server endpoints can be viewed as an OpenAPI Spec by going to the `/swagger-ui` route, this also lets you test out the API.


