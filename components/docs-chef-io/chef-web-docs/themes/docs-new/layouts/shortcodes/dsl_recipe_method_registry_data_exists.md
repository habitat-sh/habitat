Use the `registry_data_exists?` method to find out if a Microsoft
Windows registry key contains the specified data of the specified type
under the value.

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

{{ readFile "themes/docs-new/layouts/shortcodes/notes_registry_key_not_if_only_if.md" | markdownify }}



</div>

</div>

The syntax for the `registry_data_exists?` method is as follows:

``` ruby
registry_data_exists?(
  KEY_PATH,
  { name: 'NAME', type: TYPE, data: DATA },
  ARCHITECTURE
)
```

where:

-   `KEY_PATH` is the path to the registry key value. The path must
    include the registry hive, which can be specified either as its full
    name or as the 3- or 4-letter abbreviation. For example, both
    `HKLM\SECURITY` and `HKEY_LOCAL_MACHINE\SECURITY` are both valid and
    equivalent. The following hives are valid: `HKEY_LOCAL_MACHINE`,
    `HKLM`, `HKEY_CURRENT_CONFIG`, `HKCC`, `HKEY_CLASSES_ROOT`, `HKCR`,
    `HKEY_USERS`, `HKU`, `HKEY_CURRENT_USER`, and `HKCU`.
-   `{ name: 'NAME', type: TYPE, data: DATA }` is a hash that contains
    the expected name, type, and data of the registry key value
-   `type:` represents the values available for registry keys in
    Microsoft Windows. Use `:binary` for REG_BINARY, `:string` for
    REG_SZ, `:multi_string` for REG_MULTI_SZ, `:expand_string` for
    REG_EXPAND_SZ, `:dword` for REG_DWORD, `:dword_big_endian` for
    REG_DWORD_BIG_ENDIAN, or `:qword` for REG_QWORD.
-   `ARCHITECTURE` is one of the following values: `:x86_64`, `:i386`,
    or `:machine`. Set to `:i386` to read or write 32-bit registry keys
    on 64-bit machines running Microsoft Windows. Set to`:x86_64` to
    force write to a 64-bit registry location, however Chef Infra Client
    returns an exception if `:x86_64` is used on a 32-bit machine. Set
    to `:machine` to allow Chef Infra Client to allow Chef Infra Client
    to use the appropriate key location based on your node's
    architecture. Default value: `:machine`.

This method will return `true` or `false`.