print "releasing..."
direnv dotenv json .env | from json | load-env
gh release create v0.0.1 --title $"v($env.VERSION)" ./out/* 
