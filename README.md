## API routes
- room
    - [x] `POST /room/`
    - [ ] `GET /room/`
    - [x] `GET /room/{id}`
    - [ ] `DELETE /room/{id}`
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
```bash
curl -X POST "127.0.0.1:8888/room" -H 'Content-Type: application/json' -d '{"name": "kitchen"}' -v
curl -X GET "127.0.0.1:8888/room/kitchen"  -v

curl -X POST "127.0.0.1:8888/room/kitchen/device" -H 'Content-Type: application/json' -d '{"name": "socket", "address": "127.0.0.1:9999", "device_type": "tcp_socket"}' -v
```