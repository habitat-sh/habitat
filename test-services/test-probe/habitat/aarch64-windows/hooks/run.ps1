Write-Host "Running {{pkg.ident}}"
{{ #if bind.thing_with_a_port }}
Write-Host "*************************************************************"
Write-Host "Running with a bound service group for 'thing_with_a_port'"
{{ #each bind.thing_with_a_port.members as |m| ~}}
Write-Host "- {{m.sys.hostname}}"
{{/each ~}}
Write-Host "*************************************************************"
{{ else }}
Write-Host "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
Write-Host "Running WITHOUT a bound service group for 'thing_with_a_port'"
Write-Host "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
{{ /if }}

test-probe -c "{{pkg.svc_config_path}}/config.toml"
