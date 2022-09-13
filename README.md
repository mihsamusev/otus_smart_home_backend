Routes:
```sh
# room CRUD
POST /room/
GET /room/
GET /room/{id}
DELETE /room/{id}

# device CRUD
POST /device/{room_id}/
GET /device/{room_id}/
GET /device/{room_id}/{device_id}
DELETE /device/{room_id}/{device_id}

# status
GET /status/
GET /status/{room_id}
GET /status/{room_id}/{device_id}
```

```
curl -X POST "127.0.0.1:8888/room" -H 'Content-Type: application/json' -d '{"name": "kitchen"}' -v
curl -X GET "127.0.0.1:8888/room/kitchen"  -v

curl -X POST "127.0.0.1:8888/room/kitchen/device" -H 'Content-Type: application/json' -d '{"name": "socket", "address": "127.0.0.1:9999", "device_type": "tcp_socket"}' -v
```




Maybe HTTP API shouldnt know anything about types, it just infrastructure directing strings into 


Assumptions -> devices can be in the smart house but not physically available
devices can be in both smart house and physically availlable but uable to connect -> give same error -> connection error