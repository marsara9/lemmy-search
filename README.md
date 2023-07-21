[![CircleCI](https://img.shields.io/circleci/build/github/marsara9/lemmy-search/master?label=master)](https://dl.circleci.com/status-badge/redirect/gh/marsara9/lemmy-search/tree/master)
[![CircleCI](https://img.shields.io/circleci/build/github/marsara9/lemmy-search/develop?label=develop)](https://dl.circleci.com/status-badge/redirect/gh/marsara9/lemmy-search/tree/develop)

![GitHub tag (latest SemVer pre-release)](https://img.shields.io/github/v/tag/marsara9/lemmy-search)
![GitHub](https://img.shields.io/github/license/marsara9/lemmy-search)

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/marsara9z)

# Lemmy-Search

The fediverse creates some unique problems when it comes to searching.  Mostly that existing search engines can't deal with the concept that multiple servers may exist that are ultimately hosting the same content.  These same search engines also aren't aware that you only have an account on one, or maybe a select few of these instances.

Lemmy-Search will uniquely search any Lemmy instance and attempt to index the entire ferdiverse and then present a familiar search interface that will allow users to:

* Users can choose a preferred instance.  Such that all links that you open from the search results will automatically open with that user's instance, where they should already be logged in.
* The big search engines let you filter by a particular website, but this doesn't make sense for the fediverse. Instead you can still refine your searches by:
  * Instance -- This will limit your search to just communities that were created on that particular instance.
  * Community -- You can also filter search results by just the particular community.
  * Author -- You can also just search for posts that were made by a particular user.
  * NSFW -- You can choose to include or exclude NSFW (disabled by default) posts.

![landing page](https://i.ibb.co/nrz4m81/lemmy-search-home.png)

![search results page](https://i.ibb.co/WWmPqvS/lemmy-search-results.png)

## Available Filters:

As mentioned above there are several filters that can be used inside your queries, these can be combined together, but you may not reuse the same one multiple times.

* Instance: `instance:<domain>` (Only show posts that belong to communities on the mentioned instance)
* Community: `community:!<name>@<domain>` (Only show posts from the given community)
* Author: `author:@<user>@<domain>` (Only show posts by the given user)
* NSFW: `!safeoff` (allows NSFW results to be included)
* Since: `since:YYYY-MM-DD` (Only show posts that have been made since the provided date)
* Until: `until:YYYY-MM-DD` (Only show posts that have been made up until the provided date)

## How it Works

Periodically a crawler will index the posts from a given seed-instance.  These posts will be storied in a local database and the title and body of the post will be converted to a `tsvector`.  This then allows the search engine to use postgres's built-in text-searching (https://www.postgresql.org/docs/current/datatype-textsearch.html).  Searches that the users then perform are converted to a `tsquery` and compared to the data stored in the database.  Postgres automatically ranks and orders the search results based on various factors from the number of matches and how close the words are etc... This is then returned directly to the client and presented back to the user.

## Road map

For the first release I expect to have the following features:

- [x] Indexing will be limited to a single 'seed instance'.  Now assuming that instance is federated, you should still be able to search across all of the posts that your seed instance is aware of.
- [x] Federated instances of that 'seed instance' will only be indexed so that opening links will work on that target instance.
- [x] Users can type in any search string and it will match on the contents of any Post.
  - [x] Short words are automatically removed from the search query to help reduce false positives.
- [x] Preferred Instance selection.  This will be limited to instances that the search engine has found as it indexes the fediverse.
- [x] Filtering by Instance, Community and/or Author.


Eventually some ideas I'd like to support (in no particular order):

- [ ] Incorporate other fediverse type servers, including Mastodon, Kbin, etc...
- [ ] Include comment data in the index as well. (BOCKED until a Lemmy bug can be fixed).
- [ ] Explore other options of indexing and/or sharing data with other search engine instances.  Essentially have the individual search engines participate in their own mini-fediverse.  This way I can lighten the load on the actual Lemmy instances during a crawl.
- [ ] Language selection.  For now queries don't account for language at all and will just match on what you type.

## Hosting your own instance

I've included a sample docker-compose.yml file that you reference to get things started.  There's no environment variables or anything that you need to pass to the docker container, but there is a [config.yml](./config/config.yml) file that allows you to fine-tune the settings of the search engine and it's associated crawler.

### Step by Step guide

To setup your own instance or begin development, start with pulling down a copy of the [docker-compose.yml](./docker/docker-compose.yml) file.  You'll then want to edit any usernames and/or passwords, but the default values should work for development right out of the box.  

One exception though, is you'll want to modify which tag to pull down.  If you're just wanting to stand-up your own instance you can refer to the table below to see which tag you should use.  However if you wanting to do actual development, you'll want to uncomment the section that builds from the dockerfile. that looks something like this:

```yml
  build:
    context: ../
    dockerfile: dev.dockerfile
```

Next, pull down a copy of the [config.yml](./config/config.yml) file.  If you edited any values in the `docker-compose.yml` file you'll want to then make the same changes here.  Also make sure you place this in the volume that you've mapped to the `lemmy-search` service.

Finally you'll want to pull a copy of the [nginx.conf](./nginx/nginx.conf).  The default configuration assumes that you have SSL certificates and are planning to host publicly as an HTTPS server.  Feel free to modify this as needed, no special headers need to be passed to Nginx, but it is assumed to run at the root of the domain, i.e. not in a subpath.  (I haven't actually tested running this on a subpath, it may just work.)

Assuming you have everything configured correctly, you should now just be able to call `docker compose up -d` and the server should start up.

Due note that crawling of your seed instance is a process that only runs at a regular interval.  So you may need to wait 24hrs for the initial crawl to finish.  Alternatively you can edit [mod.rs](./server/src/crawler/mod.rs) to change that interval to whatever you want, but you should keep it so that it's a fairly long time between runs.  If a new crawler starts while an existing one is still running, they will both start writing the same entries to the database. For development purposes there's a config property `development_mode` that enables a few QOL features, specifically for development, including an endpoint `/crawl` that you can send a simple GET request to that will start an instance of the crawler.

***PLEASE try and use your own private Lemmy instance for development.  This instance MUST be running on port 443 though, so it'll have to be on a separate machine or different sub-domain.***

### Docker Tag Reference

|Name|Details|
|----|----|
|vX.Y.Z|This tag will always correspond to a particular release.  It won't receive any updates apart from any critical bugs that may be discovered.|
|latest|This tag will always match the master branch.  It should be the most stable apart from actual releases.  Note that this tag will be updated when a release goes out.|
|dev|This tag will always align with the develop branch.  I cannot guarantee that everything will work on this tag as feature development is on-going.|
|test|This is my local testing tag.  It can be updated multiple times per day and may not align to any particular code in the repository.  I recommend that no-one uses this, or if they do, do so at your own risk.|

## Contributors

<a href="https://github.com/marsara9/lemmy-search/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=marsara9/lemmy-search" />
</a>

Made with [contrib.rocks](https://contrib.rocks).
