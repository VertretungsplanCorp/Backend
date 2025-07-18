print "updating..."
http get https://github.com/VertretungsplanCorp/Backend/releases/latest/download/vp-backend-bundle.tar.gz | save -f vp-backend-bundle.tar.gz 
tar -xzvf vp-backend-bundle.tar.gz
rm vp-backend-bundle.tar.gz
docker compose cp vpbackend vp_server_service:.
docker compose cp .env vp_server_service:.
docker compose cp server.nu vp_server_service:.
