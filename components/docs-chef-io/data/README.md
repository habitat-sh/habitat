The Data Folder

The data folder is where you can store additional data for Hugo to use when generating your site. Data files aren’t used to generate standalone pages; rather, they’re meant to be supplemental to content files. This feature can extend the content in case your front matter fields grow out of control. Or perhaps you want to show a larger dataset in a template (see example below). In both cases, it’s a good idea to outsource the data in their own files.

These files must be YAML, JSON, XML, or TOML files (using the .yml, .yaml, .json, .xml, or .toml extension). The data will be accessible as a map in the .Site.Data variable.

https://gohugo.io/templates/data-templates/#the-data-folder
