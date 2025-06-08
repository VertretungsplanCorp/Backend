Das ist das Backend, geschrieben in Rust.

Benutzen:

Info: stelle sicher, dass du Docker[https://docs.docker.com/engine/install/]
installiert hast.

```bash
mkdir vpbackend && cd vpbackend
wget https://github.com/VertretungsplanCorp/Backend/releases/latest/download/vp-backend-bundle.tar.gz
tar -xzvf vp-backend-bundle.tar.gz && rm -r vp-backend-bundle.tar.gz
docker compose up
```

Schema:
| Ressource | Method | Type |
| :-------- | :----: | ---: |
| ping | GET | String |
