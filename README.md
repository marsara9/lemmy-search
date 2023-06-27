![docker build:latest](https://github.com/marsara9/lemmy-search/actions/workflows/build-latest.yml/badge.svg)
![docker build:dev](https://github.com/marsara9/lemmy-search/actions/workflows/build-dev.yml/badge.svg)

![GitHub tag (latest SemVer pre-release)](https://img.shields.io/github/v/tag/marsara9/lemmy-search)
![GitHub](https://img.shields.io/github/license/marsara9/lemmy-search)

# Lemmy-Search

The fediverse creates some unique problems when it comes to searching.  Mostly that existing search engines can't deal with the concept that multiple servers may exist that are ultimately hosting the same content.  These same search engines also aren't aware that you only have an account on one, or maybe a select few of these instances.

Lemmy-Search, ya I need a better name, will uniquely search any Lemmy instance and attempt to index the entire ferdiverse and then present a familiar search interface that will allow users to:

* Users can choose a preferred instance.  Such that all links that you open from the search results will automatically open with that user's instance, where they should already be logged in.
* The big search engines let you filter by a particular website, but this doesn't make sense for the fediverse. Instead you can still refine your searches by:
  * Instance -- This will limit your search to just communities that were created on that particular instance.
  * Community -- You can also filter search results by just the particular community.
  * Author -- You can also just search for posts that were made by a particular user.

## How it Works

For any given post that is found, all non-alphanumeric characters are removed a distinct list of words (anything that has a space between it) is taken from both the post title and body.  Then when the user performs a search a similar process is applied to the query and all of those distinct words are then queried from the database.  Posts that then have the highest number of matches are returned first and then those are sorted by the total score of said post.  As it is assumed that if there are more matches from your query the post is more relevant to you, and that posts with a higher score are more trust-worthy.

Note that a post that just contains the same word repeated over and over will still only count for a single match compared to a post that only mentions the word once.


## Road map

For the first release I expect to have the following features:

- [x] Indexing will be limited to a single 'seed instance'.  Now assuming that instance is federated, you should still be able to search across all of the posts that your seed instance is aware of.
- [x] Federated instances of that 'seed instance' will only be indexed so that opening links will work on that target instanace.
- [x] Users can type in any search string and it will match on the contents of any Post.
  - [ ] Common words are automatically removed from the search query to help reduce false positives.
- [x] Preferred Instance selection.  This will be limited to instances that the search engine has found as it indexes the fediverse.
- [ ] Filtering by Instance, Community and/or Author.


Eventually some ideas I'd like to support (in no particular order):

- [ ] Incorporate other fediverse type servers, including Mastodon, Kbin, etc...
- [ ] Include comment data in the index as well.
- [ ] Refine searches by comment authors instead of just post authors.
- [ ] Explore other options of indexing and/or sharing data with other search engine instances.  Essentially have the individual search engines participate in their own mini-fediverse.  This way I can lighten the load on the actual Lemmy instances during a crawl.
- [ ] Language selection.  For now queries don't account for language at all and will just match on what you type.

## Hosting your own instance

I've included a sample docker-compose.yml file that you reference to get things started.  There's no environment variables or anything that you need to pass to the docker container, but there is a [config.yml](./config/config.yml) file that allows you to fine-tune the settings of the search engine and it's associated crawler.

TODO Include a step-by-step guide.

### Docker Tag Reference

|Name|Details|
|----|----|
|vX.Y.Z|This tag will always correspond to a particular release.  It won't receive any updates apart from any critical bugs that may be discovered.|
|latest|This tag will always match the master branch.  It should be the most stable apart from actual releases.  Note that this tag will be updated when a release goes out.|
|dev|This tag will always align with the develop branch.  I cannot guarantee that everything will work on this tag as feature development is on-going.|
|test|This is my local testing tag.  It can be updated multiple times per day and may not align to any particular code in the repository.  I recommend that no-one uses this, or if they do, do so at your own risk.|
