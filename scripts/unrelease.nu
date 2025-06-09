print "reverting release..."
direnv dotenv json .env | from json | load-env
gh release delete $"v($env.VERSION)"
