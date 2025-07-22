print "releasing..."
let version = (nu ($env.CURRENT_FILE | path dirname)/read-version.nu)
gh release create $version --title $version --generate-notes --prerelease=false ./out/* 
