---
title: Hello World!
date: 2017-04-11
author: Ian Henry
tags: community
category: Community
---

As we mentioned during the weekly issue triage this week we've decided to move some of the project's communication over to an "engineering blog". Now, if you're reading this then congratulations you found it! It's basically going to function an awful lot like the currently existing [blog on chef.io](https://blog.chef.io). In fact a majority of the content that will be available here, will also be available over there.

That is to say. Not much is changing. This page is a first iteration on getting more control over some of our communication plane which should hopefully enable us to share useful information to you more rapidly. With that in mind, this is an experiment for the time being and is subject to change depending on the state of the world and the project. Obviously this blog is going to be feature sparse to begin with, but it should be a rapid and effective way to express things like workflow examples, upcoming feature changes, community event follow-ups and the like.

Since the blog is going to exist in our repo with everything else it also provides you (yes, you, the person reading this sentence) with a super simple way to publish your own Habitat content to the habitat site! You can quite easily open a pull-request that includes a markdown file like this one with a format of `{date}-{title}.html.md`. Or, for a more specific example we can look at the filename of the post you're currently reading `2017-04-11-Hello-World.html.md`. The only extra bit of information you'll need in your file is some metadata at the top of the page that should look something like this:

~~~sh
---
title: Hello World!
date: 2017-04-11
author: Ian Henry
tags: community-updates
category: Community
---
~~~
Obviously with _your_ name and the date you wrote your article. Once you've opened that PR we can vet the content and on approval it should get published out to the habitat.sh site! All in all we're perpetually trying to find ways to communicate with the community more effectively and create more content to help you with doing things in Habitat. Hopefully this change will push us further down the path of useful community interaction and contribution.

Thanks for taking the time read and as always thanks for using Habitat!
