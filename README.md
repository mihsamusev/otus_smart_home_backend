Описание/Пошаговая инструкция выполнения домашнего задания:
Реализовать с использованием веб-фреймворка HTTP сервер, реализующий функционал ""Умного дома"":
Библиотека "Умный дом" предоставляет динамическую структуру дома в комнатах которого расположены устройства.

Дом имеет название и содержит несколько помещений.

Библиотека позволяет запросить список помещений в доме, а также добавлять и удалять помещения.
Помещение имеет уникальное название и содержит названия нескольких устройств.
Устройство имеет уникальное в рамках помещения имя.
Библиотека позволяет получать список устройств в помещении, а также добавлять и удалять устройства.
Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома.

Эта функция принимает в качестве аргумента обобщённый тип, позволяющий получить текстовую информацию
о состоянии устройства, для включения в отчёт. Эта информация должна предоставляться
для каждого устройства на основе данных о положении устройства в доме: имени комнаты и имени устройства.

Если устройство не найдено в источнике информации, то вместо текста о состоянии вернуть сообщение об ошибке.
В качестве источника данных о состоянии устройств можно использовать произвольный mock-объект.
Написать клиент с запросами к HTTP API умного дома.
Написать example общения с умным домом через HTTP клиент.
Дополнительное задание: хранить структуру дома в базе данных.

Routes:

```
POST /room/
GET /room/
GET /room/{id}
DELETE /room/{id}

POST /device/{room_id}/
GET /device/{room_id}/
GET /device/{room_id}/{device_id}
DELETE /device/{room_id}/{device_id}

GET /status/{room_id}
GET /status/{room_id}/{device_id}
```

```
curl -X POST "127.0.0.1:8888/room/kitchen" -v
curl -X POST "127.0.0.1:8888/room/bathroom" -v
curl -X POST "127.0.0.1:8888/room/kitchen/device" -H 'Content-Type: application/json' -d '{"name": "socket_1", "address": "127.0.0.1:8080", "device_type": "tcp_socket"}' -v
curl -X POST "127.0.0.1:8888/room/kitchen/device" -H 'Content-Type: application/json' -d '{"name": "socket_2", "address": "127.0.0.1:8090", "device_type": "tcp_socket"}' -v
curl -X POST "127.0.0.1:8888/room/kitchen" -v
curl -X GET "127.0.0.1:8888/status/kitchen/socket_1 -v
curl -X GET "127.0.0.1:8888/status/kitchen/socket_2 -v
```

##

## On database

RoomTable
|room_id| room_name|
|---|---|
|0|kitchen|
|1|bathroom|

DeviceTable
|device_id (unique) |room_id|device_id(unique)|address (unique) |device_type|
|---|---|---|---|---|
|0|0|socket_1|127.0.0.1:8000|TcpSocket|
|1|0|socket_2|127.0.0.1:9000|TcpSocket|
|2|1|thermo_1|127.0.0.1:8080|UdpThermo|

Maybe HTTP API shouldnt know anything about types, it just infrastructure directing strings into

Assumptions -> devices can be in the smart house but not physically available
devices can be in both smart house and physically availlable but uable to connect -> give same error -> connection error

```rust
DeviceQuery {
    device_type: DeviceType,
    address: SocketAddr,
    query: String
}
fn query(devise_repository, query: DeviceQuery) -> DeviceResponse;
```
