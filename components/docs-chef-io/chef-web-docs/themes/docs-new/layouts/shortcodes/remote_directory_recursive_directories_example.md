This section contains a more detailed example of how Chef Infra Client
manages recursive directory structures:

-   A cookbook named `cumbria` that is used to build a website
-   A subfolder in the `/files/default` directory named `/website`
-   A file named `index.html`, which is the root page for the website
-   Directories within `/website` named `/cities`, `/places`, and
    `/football`, which contains pages about cities, places, and football
    teams
-   A directory named `/images`, which contains images

These files are placed in the `/files/default` directory in the
`cumbria` cookbook, like this:

``` text
cumbria
  /files
    /default
      /website
        index.html
        /cities
          carisle.html
          kendal.html
          penrith.html
          windermere.html
        /football
          carisle_united.html
        /images
          carisle_united.png
          furness_abbey.png
          hadrians_wall.png
          kendal.png
        /places
          furness_abbey.html
          hadrians_wall.html
```

The **remote_directory** resource can be used to build a website using
these files. This website is being run on an Apache web server. The
resource would be similar to the following:

``` ruby
remote_directory "/var/www/html" do
  files_mode '0440'
  files_owner 'yan'
  mode '0770'
  owner 'hamilton'
  source "website"
end
```

When Chef Infra Client runs, the **remote_directory** resource will
tell Chef Infra Client to copy the directory tree from the cookbook to
the file system using the structure defined in cookbook:

``` text
/var
  /www
    /html
      index.html
      /cities
        carisle.html
        kendal.html
        penrith.html
        windermere.html
      /football
        carisle_united.html
      /images
        carisle_united.png
        furness_abbey.png
        hadrians_wall.png
        kendal.png
      /places
        furness_abbey.html
        hadrians_wall.html
```

Chef Infra Client will manage the permissions of the entire directory
structure below `/html`, for a total of 12 files and 4 directories. For
example:

``` bash
dr-xr-xr-x 2 root     root 4096 /var/www/html
dr--r----- 1 yan      root 4096 /var/www/html/index.html
drwxrwx--- 2 hamilton root 4096 /var/www/html/cities
dr--r----- 1 yan      root 4096 /var/www/html/cities/carlisle.html
dr--r----- 1 yan      root 4096 /var/www/html/cities/kendal.html
dr--r----- 1 yan      root 4096 /var/www/html/cities/penrith.html
dr--r----- 1 yan      root 4096 /var/www/html/cities/windermere.html
drwxrwx--- 2 hamilton root 4096 /var/www/html/football
dr--r----- 1 yan      root 4096 /var/www/html/football/carlisle_united.html
drwxrwx--- 2 hamilton root 4096 /var/www/html/images
dr--r----- 1 yan      root 4096 /var/www/html/images/carlisle_united/png
dr--r----- 1 yan      root 4096 /var/www/html/images/furness_abbey/png
dr--r----- 1 yan      root 4096 /var/www/html/images/hadrians_wall.png
dr--r----- 1 yan      root 4096 /var/www/html/images/kendal.png
drwxrwx--- 2 hamilton root 4096 /var/www/html/places
dr--r----- 1 yan      root 4096 /var/www/html/places/furness_abbey.html
dr--r----- 1 yan      root 4096 /var/www/html/places/hadrians_wall.html
```