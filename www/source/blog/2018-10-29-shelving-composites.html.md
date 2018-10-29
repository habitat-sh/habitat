---
title: Shelving Composites (for now)
date: 2018-10-29
author: Garrett Amini
tags: supervisor, packaging
category: supervisor
classes: body-article
---

A little over one year ago, Christopher Maier [presented the ability to create _composite packages_](https://www.habitat.sh/blog/2017/10/Introducing-Composites/), a special kind of Habitat package that includes other Habitat packages in order to be managed by a single Supervisor. The Habitat community had been finding all sorts of creative ways to use Habitat to incorporate their services under a single Supervisor, so we sought a way to canonically enable them to do so.

This past year has taught us that while we were on the right track with the idea of composite packages, our implementation left much to be desired. Users kept trying to utilize composite packages, but would inevitably find themselves bumping up against limitations of the feature, and would thus end implementing the same sorts of creative solutions we were trying to simplify in the first place.

## What's Happening Now?

We will soon be removing the ability to run composite plans from Habitat. We will provide at least 2 release cycles before the ability to build and run composites is removed, which will be in version 0.69.0 at the earliest. In the short term, this will simplify our roadmap to 1.0 and increase critical code stability. We have learned much over the past year from our initial implementation, and based upon that feedback and with an eye toward hearing more from the community, we're looking forward to being able to build new features to accomplish the same things that composite plans were designed to do.

## The Path Forward

As we near 1.0, we are seeking to make Habitat as robust and reliable as possible. Once we achieve that milestone, we're looking forward to once again meeting the usecases that composites were designed for.

If you have feedback that will help us drive this in the right direction, we'd love to hear from you [in the forums](https://forums.habitat.sh/t/the-plan-for-plan-composites/888). Happy Habitizing!
