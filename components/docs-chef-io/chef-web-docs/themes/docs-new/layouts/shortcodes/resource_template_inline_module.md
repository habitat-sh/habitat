A template helper module can be defined inline on a per-resource basis.
This approach can be useful when a template requires more complex
information. For example:

``` ruby
template '/path' do
  helpers do

    def hello_world
      'hello world'
    end

    def app
      node['app']
    end

    def app_conf(setting)
      node['app']['setting']
    end

  end
end
```

where the `hello_world`, `app`, and `app_conf(setting)` methods comprise
the module that extends a template.