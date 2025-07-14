print "releasing..."
direnv dotenv json .env | from json | load-env
let version = $"v($env.VERSION)"
gh release create $"($version)" --title $version --generate-notes --prerelease=false ./out/* 
