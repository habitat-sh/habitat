pkg_origin="core"
pkg_name="builder-api-only"
pkg_type="composite"
pkg_version="1.0.0"

pkg_services=(
    core/builder-api
    core/builder-api-proxy
    # core/builder-router
)
pkg_bind_map=(
    # [core/builder-api]="router:core/builder-router"
    [core/builder-api-proxy]="http:core/builder-api"
 )
