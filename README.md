Das ist das Backend, geschrieben in Rust.

Use:

```bash
wget https://github.com/VertretungsplanCorp/Backend/releases/latest/download/vp-backend-bundle.tar.gz
tar -xzvf vp-backend-bundle vpbackend
cd vpbackend
docker compose up
```

Schema:
| Ressource | Method | Type |
| :-------- | :----: | ---: |
| ping | GET | String |
