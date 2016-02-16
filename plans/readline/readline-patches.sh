# The root URL for all official patch files
_patch_url_base=$_url_base/${pkg_name}-${_base_version}-patches/${pkg_name}${_base_version//.}

# All official patch file URLs
_patch_files=(
  ${_patch_url_base}-001
  ${_patch_url_base}-002
  ${_patch_url_base}-003
  ${_patch_url_base}-004
  ${_patch_url_base}-005
  ${_patch_url_base}-006
  ${_patch_url_base}-007
  ${_patch_url_base}-008
)

# All official patch file shasums
_patch_shasums=(
  1a79bbb6eaee750e0d6f7f3d059b30a45fc54e8e388a8e05e9c3ae598590146f
  39e304c7a526888f9e112e733848215736fb7b9d540729b9e31f3347b7a1e0a5
  ec41bdd8b00fd884e847708513df41d51b1243cecb680189e31b7173d01ca52f
  4547b906fb2570866c21887807de5dee19838a60a1afb66385b272155e4355cc
  877788f9228d1a9907a4bcfe3d6dd0439c08d728949458b41208d9bf9060274b
  5c237ab3c6c97c23cf52b2a118adc265b7fb411b57c93a5f7c221d50fafbe556
  4d79b5a2adec3c2e8114cbd3d63c1771f7c6cf64035368624903d257014f5bea
  3bc093cf526ceac23eb80256b0ec87fa1735540d659742107b6284d635c43787
)
