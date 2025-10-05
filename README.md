Das ist das Backend, geschrieben in Rust.

## Installieren

> Info: stelle sicher, dass du [Docker](https://docs.docker.com/engine/install/) installiert hast.
> Optional kann [Nushell](https://nushell.sh) verwendet werden.

### bash:

```bash
mkdir vpbackend && cd vpbackend
wget https://github.com/VertretungsplanCorp/Backend/releases/latest/download/vp-backend-bundle.tar.gz
tar -xzvf vp-backend-bundle.tar.gz && rm -r vp-backend-bundle.tar.gz
docker compose up -d
```

### nushell:

```nu
mkdir vpbackend
cd vpbackend
http get https://github.com/VertretungsplanCorp/Backend/releases/latest/download/vp-backend-bundle.tar.gz | save -f vp-backend-bundle.tar.gz
tar -xzvf vp-backend-bundle.tar.gz
rm vp-backend-bundle.tar.gz
docker compose up -d
```

## Updaten

### bash:

```bash
bash update.sh
```

### nushell:

```nu
nu update.nu
```

## Api-Schema

| Resource        | Method |  Type  |                            Desc |
| :-------------- | :----: | :----: | ------------------------------: |
| ping            |  GET   | String |   out: `Hallo aus dem Backend!` |
| get_klasse      |  GET   |  JSON  |  `/get_klasse?stufe=5?klasse=a` |
| get_stufe       |  GET   |  JSON  |            `/get_stufe?stufe=5` |
| get_stufen      |  GET   |  JSON  |      `/get_stufen?von=5?bis=12` |
| get_unterstufe  |  GET   |  JSON  |   `/get_unterstufe` von 5 bis 8 |
| get_mittelstufe |  GET   |  JSON  | `/get_mittelstufe` von 9 bis 11 |
| get_oberstufe   |  GET   |  JSON  |  `/get_oberstufe` von 12 bis 13 |
