Gixie Clock Brightness Control
---

Automatic brightness control depending on the time of day

## How to build
_If you do not need to build for the ARM platform, then just remove `.cargo` folder._

```bash
$ rustup target add armv7-unknown-linux-musleabihf
$ cargo build --release
```

## How to use
- create `config.yaml`
```yaml
coord:                       # map position
  latitude: 59.33258
  longitude: 18.06490
clock:
  timezone: 1                # your clock time zone
  server: "ws://127.0.0.1"   # gixie clock websocket server
  date_fmt: "%Y-%m-%d %H:%M" # date format for suninfo command
brightness:
  min: 10                    # nighttime brightness
  max: 250                   # daytime brightness
  num: 14                    # gixie clock cmdNum
```

- launch
```bash
$ gixie-bright-control -c config.yaml
```

- or just use as cli app
```bash
$ gixie-bright-control get
10
$ gixie-bright-control set 100
$ gixie-bright-control suninfo
sunrise: 2042-08-15 04:20
 sunset: 2042-08-15 20:07
```

## How to get clock websocket command
- install [Gixie Clock](https://play.google.com/store/apps/details?id=uni.UNICB90ED7) app
- install [Wireshark](https://www.wireshark.org/)
- configure router for [Packet Sniffer](https://wiki.mikrotik.com/wiki/Manual:Tools/Packet_Sniffer)
- run and sniff
