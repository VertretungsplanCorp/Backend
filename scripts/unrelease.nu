print "reverting release..."
let version = (nu ($env.CURRENT_FILE | path dirname)/read-version.nu)
gh release delete $version 
