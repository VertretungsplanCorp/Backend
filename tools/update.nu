print "updating..."
http get https://github.com/VertretungsplanCorp/Backend/releases/latest/download/vp-backend-bundle.tar.gz | save -f vp-backend-bundle.tar.gz 
tar -xzvf vp-backend-bundle.tar.gz
rm vp-backend-bundle.tar.gz
