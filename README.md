Exercise in creating a smart home HTTP backend for [OTUS Rust Developer](https://otus.ru/lessons/rust-developer/?int_source=courses_catalog&int_term=programming)

[![](https://github.com/mihsamusev/otus_smart_home_backend/actions/workflows/build.yml/badge.svg)](https://github.com/mihsamusev/otus_smart_home_backend/actions/workflows/build.yml)

## API routes

- room
  - [x] `POST /room/{room_id}`
  - [x] `GET /room`
  - [x] `GET /room/{room_id}`
  - [x] `DELETE /room/{room_id}`
- device
  - [x] `POST /device/{room_id}`
  - [x] `GET /device/{room_id}/{device_id}`
  - [x] `DELETE /device/{room_id}/{device_id}`
- status
  - [x] `GET /status/{room_id}`
  - [x] `GET /status/{room_id}/{device_id}`

## Example

Turn on TCP socket device server to communicate with it device through HTTP API. You can run multiple servers on different ports to emulate more than one device.

```
cargo run --example net_socket_emulator -- --address 127.0.0.1:8080
cargo run --example net_socket_emulator -- --address 127.0.0.1:8090
```

Start the smart home HTTP server

```
cargo run
```

Interact with an api using [imported Postman collection](https://learning.postman.com/docs/getting-started/importing-and-exporting-data/#importing-postman-data) from the following [JSON link](https://www.getpostman.com/collections/84aaab4202ef73a0b0b5), or try the following `curl` commands in your terminal:

```bash
# create a kitchen nad bathroom
curl -X POST "127.0.0.1:8888/room/kitchen"
curl -X POST "127.0.0.1:8888/room/bathroom"
curl -X GET "127.0.0.1:8888/room"
curl -X GET "127.0.0.1:8888/room/kitchen"

# add 2 devices to kitchen
curl -X POST "127.0.0.1:8888/device/kitchen" -H 'Content-Type: application/json' -d '{"device_name": "socket_1", "address": "127.0.0.1:8080", "device_type": "tcp_socket"}'

# add 1 device to bathroom
curl -X POST "127.0.0.1:8888/device/bathroom" -H 'Content-Type: application/json' -d '{"device_name": "socket_1", "address": "127.0.0.1:8090", "device_type": "tcp_socket"}'

curl -X POST "127.0.0.1:8888/device/bathroom" -H 'Content-Type: application/json' -d '{"device_name": "socket_2", "address": "127.0.0.1:8091", "device_type": "tcp_socket"}'

# see the rooms layout with devices
curl -X GET "127.0.0.1:8888/room"

# ask devices for their statuses
curl -X GET "127.0.0.1:8888/status/bathroom"
curl -X GET "127.0.0.1:8888/status/kitchen"

# delete a device and see how many are left
curl -X DELETE "127.0.0.1:8888/device/bathroom/socket_1"
curl -X GET "127.0.0.1:8888/room"
```
