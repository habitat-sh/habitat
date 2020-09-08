If processes is started by using the **execute** or **script** resources
(or any of the resources based on those two resources, such as
**bash**), use the `environment` attribute to alter the environment that
will be passed to the process.

``` bash
bash 'env_test' do
  code <<-EOF
  echo $FOO
EOF
  environment ({ 'FOO' => 'bar' })
end
```

The only environment being altered is the one being passed to the child
process that is started by the **bash** resource. This will not affect
the Chef Infra Client environment or any child processes.