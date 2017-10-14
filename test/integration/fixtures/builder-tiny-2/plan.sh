pkg_origin="core"
pkg_name="builder-tiny"
pkg_type="composite"
pkg_version="2.0.0"

pkg_services=(
    core/builder-admin
    core/builder-admin-proxy
    core/builder-router
)
pkg_bind_map=(
     [core/builder-admin]="router:core/builder-router"
     [core/builder-admin-proxy]="http:core/builder-admin"
)
