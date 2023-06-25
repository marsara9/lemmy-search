var preferred_instance = "https://lemmy.world/"

function checkQueryParamers() {
    const urlParameters = new URLSearchParams(window.location.search);
    return urlParameters.has("query");
}

function query(queryString) {
    fetchJson("/search" + queryString, result => {
        let list = $("<ol/>");
        result.posts.forEach(post => {
            let item = $("<li/>")
                .addClass("search-result");
            if (post.url) {
                let url = $("<img/>");
                url.addClass("post-url");
                url.attr("src", post.url);
                item.append(url);
            }

            let post_name = $("<a/>")
                .addClass("post-name")
                .attr("href", preferred_instance + "post/" + post.remote_id);
            post_name.text(post.name);
            item.append(post_name);

            let post_citation = $("<div/>")
                .addClass("post-citation");

            if(post.author.avatar) {
                let post_author_avatar = $("<img />")
                    .attr("src", post.author.avatar);
                post_citation.append(post_author_avatar);
            }

            let post_author = $("<a/>")
                .attr("href", preferred_instance + "u/" + post.author.name);
            post_author.text(post.author.display_name ?? post.author.name);
            post_citation.append(post_author);

            let divider = $("<span/>");
            divider.text(" | ");
            post_citation.append(divider);

            if(post.community.icon) {
                let post_community_icon = $("<img />")
                    .attr("src", post.community.icon);
                post_citation.append(post_community_icon);
            }

            let post_community = $("<a/>")
                .attr("href", preferred_instance + "c/" + post.community.name);
            post_community.text(post.community.title ?? post.community.name);
            post_citation.append(post_community);

            item.append(post_citation);

            let post_body = $("<p>/")
                .addClass("post-body");
            if(post.body != null) {
                post_body.append(getPostQueryBody(result.original_query_terms, post.body));
            }
            item.append(post_body);

            list.append(item);
        });
        $("#results").empty();
        $("#results").append(list);
    })
}

function getPostQueryBody(queryTerms, body) {
    let regex = "(" + queryTerms.join(")|(") + ")";
    let split_body = body.split(new RegExp(regex, "ig"))
        .filter(token => token != null);
    var length = 0;
    let spans = split_body.map(token => {

        if (length >= 200) {
            return null;
        }

        let span = $("<span />");
        if (queryTerms.includes(token)) {
            span.addClass("search-term");
        }

        let sub = Math.min(200 - length, token.length);
        span.text(token.substring(0, sub));
        length += sub;

        return span;
    }).filter(span => span != null);

    if(spans.length < split_body.length) {
        let more = $("<span />");
        more.text("...");
        spans.push(more);
    }
    return spans;
}

$(document).ready(function() {
    if (!checkQueryParamers()) {
        window.location = "/";
        return;
    }

    query(window.location.search);
});
