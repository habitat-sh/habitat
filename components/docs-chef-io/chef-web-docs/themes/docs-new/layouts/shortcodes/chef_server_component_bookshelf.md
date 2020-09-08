Bookshelf is used to store cookbook content---files, templates, and so
on---that have been uploaded to the Chef Infra Server as part of a
cookbook version. Cookbook content is stored by content checksum. If two
different cookbooks or different versions of the same cookbook include
the same file or template, Bookshelf will store that file only once. The
cookbook content managed by Bookshelf is stored in flat files and is
separated from the Chef Infra Server and search index repositories.