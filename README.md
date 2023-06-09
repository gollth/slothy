# sIoThy

[![CI](https://github.com/gollth/slothy/actions/workflows/ci.yaml/badge.svg)](https://github.com/gollth/slothy/actions/workflows/ci.yaml)

Soil IoT Humidity Monitor

## ESP8266

![pinout](https://i0.wp.com/edistechlab.com/wp-content/uploads/2021/04/WeMos-d1-mini-Pin-out.png)


## ADC-Mux Shield

Since the ESP8266 has only one analog input (`A0`), we have to use a analog signal multiplexer. For that we
use the [74HC4051D](https://www.mouser.com/datasheet/2/408/74HC4051D_datasheet_en_20201203-959201.pdf):

![74HC4051D](./74hc4051D.png)

This is mounted on a customly designed shield, where the ESP socks into:

![shield](./shield.jpg)

# Backend

```bash
sudo apt install sqlite3
cat migrations/00_bootstrap.sql | sqlite3 db.sqlite
cargo install cargo-watch
cargo watch -cqw src -x run  # Restart server during development on every file change
```
