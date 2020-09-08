`ohai.directory`

:   The directory in which Ohai plugins are located.

`ohai.disabled_plugins`

:   An array of Ohai plugins to be disabled on a node. The list of
    plugins included in Ohai can be found in the `ohai/lib/ohai/plugins`
    directory. For example, disabling a single plugin:

    ``` ruby
    ohai.disabled_plugins = [
      :MyPlugin
    ]
    ```

    or disabling multiple plugins:

    ``` ruby
    ohai.disabled_plugins = [
      :MyPlugin,
      :MyPlugin2,
      :MyPlugin3
    ]
    ```

    When a plugin is disabled, the Chef Infra Client log file will
    contain entries similar to:

    ``` ruby
    [2014-06-13T23:49:12+00:00] DEBUG: Skipping disabled plugin MyPlugin
    ```

`ohai.hints_path`

:   The path to the file that contains hints for Ohai.

`ohai.log_level`

:   The level of logging to be stored in a log file.

`ohai.log_location`

:   The location of the log file.

`ohai.plugin_path`

:   An array of paths at which Ohai plugins are located. Default value:
    `[<CHEF_GEM_PATH>/ohai-9.9.9/lib/ohai/plugins]`. When custom Ohai
    plugins are added, the paths must be added to the array. For
    example, a single plugin:

    ``` ruby
    ohai.plugin_path << '/etc/chef/ohai_plugins'
    ```

    and for multiple plugins:

    ``` ruby
    ohai.plugin_path += [
      '/etc/chef/ohai_plugins',
      '/path/to/other/plugins'
      ]
    ```

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

The Ohai executable ignores settings in the client.rb file when Ohai is
run independently of Chef Infra Client.



</div>

</div>