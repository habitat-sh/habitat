A Microsoft Windows registry key can be used as a string in Ruby code,
such as when a registry key is used as the name of a recipe. In Ruby,
when a registry key is enclosed in a double-quoted string (`" "`), the
same backslash character (`\`) that is used to define the registry key
path separator is also used in Ruby to define an escape character.
Therefore, the registry key path separators must be escaped when they
are enclosed in a double-quoted string. For example, the following
registry key:

``` ruby
HKCU\SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\Themes
```

may be enclosed in a single-quoted string with a single backslash:

``` ruby
'HKCU\SOFTWARE\path\to\key\Themes'
```

or may be enclosed in a double-quoted string with an extra backslash as
an escape character:

``` ruby
"HKCU\\SOFTWARE\\path\\to\\key\\Themes"
```