let exec_name = "vp-backend"
let out_dir = "out" | path expand
let tools_dir = "tools" | path expand
let exec_dir = ["target", "release"] | path join | path expand
let bundle_target = [$out_dir, "vp-backend-bundle.tar.gz"] | path join

print "building..."
cargo build --release

print "bundling..."
mkdir $out_dir
tar -czvf $bundle_target -C $exec_dir $exec_name -C $tools_dir . 

print "finished making bundle."
