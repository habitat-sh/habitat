The following arguments can be used with the `not_if` or `only_if` guard
properties:

`:user`

:   Specify the user that a command will run as. For example:

    ``` ruby
    not_if 'grep adam /etc/passwd', :user => 'adam'
    ```

`:group`

:   Specify the group that a command will run as. For example:

    ``` ruby
    not_if 'grep adam /etc/passwd', :group => 'adam'
    ```

`:environment`

:   Specify a Hash of environment variables to be set. For example:

    ``` ruby
    not_if 'grep adam /etc/passwd', :environment => {
      'HOME' => '/home/adam'
    }
    ```

`:cwd`

:   Set the current working directory before running a command. For
    example:

    ``` ruby
    not_if 'grep adam passwd', :cwd => '/etc'
    ```

`:timeout`

:   Set a timeout for a command. For example:

    ``` ruby
    not_if 'sleep 10000', :timeout => 10
    ```