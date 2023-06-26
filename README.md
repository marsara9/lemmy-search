![docker build:latest](https://github.com/marsara9/lemmy-search/actions/workflows/build-latest.yml/badge.svg)
![docker build:dev](https://github.com/marsara9/lemmy-search/actions/workflows/build-dev.yml/badge.svg)

![GitHub tag (latest SemVer pre-release)](https://img.shields.io/github/v/tag/marsara9/lemmy-search)
![GitHub](https://img.shields.io/github/license/marsara9/lemmy-search)

![Discord](https://discord.gg/TW332maubQ)

# Lemmy-Search

The fediverse creates some unique problems when it comes to searching.  Mostly that existing search engines can't deal with the concept that multiple servers may exist that are ultimately hosting the same content.  These same search engines also aren't awawre that you only have an account on one, or maybe a select few of these instances.

Lemmy-Search, ya I need a better name, will uniquely search any Lemmy instance and attempt to index the entire ferdiverse and then present a familiar search interface that will allow users to:

1. Users can choose a preferred instance.  Such that all links that you open from the search results will automatically open with that user's instance, where they should already be logged in.
2. The big search engines let you filter by a parituclar website, but this doesn't make sense for the fediverse.. Instead you can still refine your searches by:
    a. Instance -- This will limit your search to just communities that were created on that particular instance.
    b. Community -- You can also filter search results by just the particular community.
    c. Author -- You can also just search for posts that were made by a particular user.

## Roadmap

For the first release I expect to have the following features:

* Indexing will be limited to a single 'seed instance'.  Now assuming that instance is federated, you should still be able to search across all of the posts that your seed instance is aware of.
* Users can type in any search string and it will match on the contents of any Post or Comment within a Post that the seed instance is aware about.
  * Common words are automatically removed from the search query to help reduce false positives.
* Prefereed Instance selection.  This will be limited to instances that the search engine has found as it indexes the fediverse.
  * Because of changes that appear to be in 0.18 of Lemmy, this may now be limited to just the seed instance.  (Still investigating).
* Filtering by Instance, Community and/or Author.


Eventually some ideas I'd like to support (in no particular order):

* Incoporate other fediverse type servers, including Mastadon, Kbin, etc...
* Refine searches by comment authors instead of just post authors.
* Explore other options to list multiple search engines talk to each other to help reduce the amount of network requests required to crawl a Lemmy instance.
* Language selection.  For now it only supports English.
  * Any posts that indicate that they are for any other langauge other than English or Undetermined are discarded, to reduce false positive matches.

## Hosting your own instance

I've included a sample docker-compose.yml file that you refence to get things started.  There's no environment variables or anything that you need to pass to the docker container, but there is a [config.yml](./config/config.yml) file that allows you to fine-tune the seettings of the search engine and it's associated cralwer.

TODO Include a step-by-step guide.
