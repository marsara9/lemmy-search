var preferred_instance


function checkQueryParamers() {
    const urlParameters = new URLSearchParams(window.location.search);
    return urlParameters.has("query");
}

function query(queryString) {
    fetchJson("/search" + queryString, result => {
        let list = $("<ol></ol>");
        result.search_results.forEach(post => {
            let item = $("<li></li>")
                .addClass("search-result");
            if (post.url) {
                let url = $("<img></img>");
                url.addClass("post-url");
                url.attr("src", post.url);
                item.append(url);
            }

            let post_name = $("<a></a>")
                .addClass("post-name")
                .attr("href", preferred_instance + "post/" + post.remote_id);
            post_name.text(post.name);
            item.append(post_name);

            let post_citation = $("<div></div>")
                .addClass("post-citation");

            let post_author = $("<a></a>")
                .attr("href", preferred_instance + "u/" + post.author_actor_id);
            post_author.text(post.author_name);

            let divider = $("<span></span>");
            divider.text(" | ");

            let post_community = $("<a></a>")
                .attr("href", preferred_instance + "c/" + post.community_actor_id);
            post_community.text(post.community_name);

            post_citation
                .append(post_author)
                .append(divider)
                .append(post_community);

            item.append(post_citation);

            let post_body = $("<p></p>")
                .addClass("post-body");
            post_body.append(getPostQueryBody(result.original_query_terms, post.body));
            item.append(post_body);

            list.append(item);
        });
        $("#results").empty();
        $("#results").append(list);
    })
}

function getPostQueryBody(queryTerms, body) {
    let regex = "(" + queryTerms.join(")|(") + ")";
    let split_body = body.split(new RegExp(regex, "ig"));
    var length = 0;
    return split_body.map(token => {
        let span = $("<span />");
        if (queryTerms.contains(token)) {
            span.addClass("search-term");
        }
        span.text(token);

        var result = null;
        if(length < 200) {
            result = span;
        }
        length += token.length;
    });
}

$(document).ready(function() {
    if (!checkQueryParamers()) {
        window.location = "/";
        return;
    }

    query(window.location.search);
});
