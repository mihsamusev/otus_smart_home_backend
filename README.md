## API routes

- room
  - [x] `POST /room/{room_id}`
  - [ ] `GET /room/`
  - [x] `GET /room/{room_id}`
  - [ ] `DELETE /room/{room_id}`
- device
  - [x] `POST /device/{room_id}/`
  - [ ] `GET /device/{room_id}/`
  - [x] `GET /device/{room_id}/{device_id}`
  - [ ] `DELETE /device/{room_id}/{device_id}`
- status
  - [ ] `GET /status/`
  - [ ] `GET /status/{room_id}`
  - [ ] `GET /status/{room_id}/{device_id}`

## Example

Turn on TCP socket device server to communicate with it device through HTTP API. You can run multiple servers on different ports to emulate more than one device.

```
cargo run --example net_socket_emulator -- --address 127.0.0.1:8080
```

Start the smart home HTTP server

```
cargo run
```

Try the following `curl` commands:

```bash
# create a kitchen
curl -X POST "127.0.0.1:8888/room/kitchen" -v
curl -X POST "127.0.0.1:8888/room/bathroom" -v
curl -X GET "127.0.0.1:8888/room/"  -v
curl -X GET "127.0.0.1:8888/room/kitchen"  -v

# add device to kitchen
curl -X POST "127.0.0.1:8888/room/kitchen/device" -H 'Content-Type: application/json' -d '{"name": "socket", "address": "127.0.0.1:9999", "device_type": "tcp_socket"}' -v


curl -X DELETE "127.0.0.1:8888/room/kitchen"  -v
```
