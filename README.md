# UDP Message Client

a practice using Rust to build duo-thread udp client that can send and receive messages

### Build
```
make build
```

### Use
```
./udpmsgclient {origin_address} {destination_address}
```

### TODOs

1. use ncurses to build a UI
2. change destination address in-app
3. consider using [pnet](https://github.com/libpnet/libpnet) lib