# PUFA Helper

Server working on 3000 port. You should expose it

## Installation

### Docker

```shell
$ docker run -p 3000:3000 --init --rm filipponik/pufa-rs
```

### Compile by yourself

```shell
git clone git@github.com:Filipponik/pufa-rs.git
cargo run --release
```

## Usage

Go to http://localhost:3000 and get today pufa word