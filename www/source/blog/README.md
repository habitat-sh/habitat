The Habitat blog exists in our repo with everything else and provides you with a simple way to publish your own Habitat content to the habitat site! 

You can open a pull request that includes a markdown file like this one with a format of `{date}-{title}.html.md`. For  example `2017-04-11-Hello-World.html.md`. 

To have your signature show up neatly in the blog, add metadata at the top of the page that should look something like this:

~~~sh
---
title: Welcome to our blog!
date: 2017-09-20
author: Tasha Drew
tags: blog
category: product
classes: body-article
---
~~~

If you want to reference media in your blog, [add a folder with the same name as your blog here](https://github.com/habitat-sh/habitat/tree/master/www/source/blog/media). 

Be sure to make a pull request to add yourself to the [authors list with your biography](https://github.com/habitat-sh/habitat/blob/0ff9083f999014bca06edddb781bacd95e0c5410/www/data/author_bios.yml), too! 

Obviously with _your_ name and the date you wrote your article. Once you've opened that PR we can vet the content and on approval it will get published out to the habitat.sh site! 

Thanks for taking the time read and as always thanks for using Habitat!
