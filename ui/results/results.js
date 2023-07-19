function getQueryParameters() {
    const urlParameters = new URLSearchParams(window.location.search);
    return {
        "query": urlParameters.get("query"),
        "page": urlParameters.get("page") || 1,
        "mode": urlParameters.get("mode")
    };
}

function query(queryString, page, mode, instance) {

    queryParameters = new URLSearchParams({
        "query" : queryString,
        "page" : page,
        "home_instance" : dropSchema(instance)
    }).toString()

    fetchJson(`/api/search/${mode}?` + queryParameters, result => {

        let response_time = Math.round((result.time_taken.secs + (result.time_taken.nanos / 1_000_000_000)) * 100) / 100;

        $("#response-time").text(
            "Found " + result.total_results + " results in " + response_time + " seconds"
        );

        let list = $("<ol/>");

        switch(mode) {
            case "posts":
                result.posts.forEach(post => {
                    let item = buildPostSearchResult(post, result.original_query_terms);
                    list.append(item);
                });
                break;
            case "communities":
                result.communities.forEach(post => {
                    let item = buildCommunitySearchResult(post, result.original_query_terms);
                    list.append(item);
                });
                break;
        }
        
        $("#results").empty();
        $("#results").append(list);

        buildPageControls(result.total_pages, mode);
    })
}

function buildPageControls(total_pages, mode) {
    const urlParameters = new URLSearchParams(window.location.search);
    let query = urlParameters.get("query");
    let page = Math.max(parseInt(urlParameters.get("page"), 10) || 1, 1);

    let page_control = $("#page-control")
        .empty();

    if(page > 1) {
        let params = {
            "query" : query,
            "page" : page - 1,
            "mode" : mode
        };
        
        let href = "/results?" + new URLSearchParams(params).toString();

        let previous = $("<a />")
            .attr("href", href);

        previous.text("< Previous");

        page_control.append(previous);
    }
    if(page > 1 && page < total_pages) {
        page_control.append($("<span> | </span>"));
    }
    if(page < total_pages) {
        let params = {
            "query" : query,
            "page" : page + 1,
            "mode" : mode
        };
        
        let href = "/results?" + new URLSearchParams(params).toString();

        let next = $("<a />")
            .attr("href", href);

        next.text("Next >");

        page_control.append(next);
    }
}

function buildPostSearchResult(post, original_query_terms) {
    let item = $("<li/>")
        .addClass("search-result");

    let post_name = $("<a/>")
        .addClass("post-name")
        .attr("href", home_instance + "post/" + post.remote_id);
    post_name.text(post.name);
    item.append(post_name);

    let post_citation = $("<div/>")
        .addClass("post-citation");

    if(post.author.avatar && isImage(post.author.avatar)) {
        let post_author_avatar = $("<img />")
            .attr("src", post.author.avatar);
        post_citation.append(post_author_avatar);
    }

    let post_author = $("<a/>");
    if(post.author.actor_id.startsWith(home_instance)) {
        post_author.attr("href", post.author.actor_id);
    } else {
        let instance = new URL(post.author.actor_id).hostname;
        let href = home_instance + "u/" + post.author.name + "@" + instance;

        post_author.attr("href", href);
    }
    post_author.text(post.author.display_name ?? post.author.name);
    post_citation.append(post_author);

    let divider = $("<span/>");
    divider.text(" | ");
    post_citation.append(divider);

    if(post.community.icon && isImage(post.community.icon)) {
        let post_community_icon = $("<img />")
            .attr("src", post.community.icon);
        post_citation.append(post_community_icon);
    }

    let post_community = $("<a/>");
    if(post.community.actor_id.startsWith(home_instance)) {
        post_community.attr("href", post.community.actor_id);
    } else {
        let instance = new URL(post.community.actor_id).hostname;
        let href = home_instance + "c/" + post.community.name + "@" + instance;

        post_community.attr("href", href);
    }
    post_community.text(post.community.title ?? post.community.name);
    post_citation.append(post_community);

    let divider2 = $("<span/>");
    divider2.text(" | ");
    post_citation.append(divider2);

    let updated = $("<span/>")
        .addClass("date");
    let date = new Date(post.updated);
    updated.text(date.toLocaleString());
    post_citation.append(updated);

    item.append(post_citation);

    let post_body = $("<p>/")
        .addClass("post-body");
    if(post.body != null) {
        post_body.append(getPostQueryBody(original_query_terms, post.body));
    }
    item.append(post_body);

    return item;
}

const MAX_DESCRIPTION_LENGTH = 200;

function getPostQueryBody(queryTerms, body) {
    let regex = "(\\s" + queryTerms.join("\\s)|(\\s") + "\\s)";
    let split_body = body.split(new RegExp(regex, "ig"))
        .filter(token => token != null);
    var length = 0;
    let spans = split_body.map(token => {

        if (length >= MAX_DESCRIPTION_LENGTH) {
            return null;
        }

        let span = $("<span />");
        if (queryTerms.includes(token.toLowerCase().trim())) {
            span.addClass("search-term");
        }

        if (token.length + length > MAX_DESCRIPTION_LENGTH) {
            let max_remaining = Math.min(200 - length, token.length);
            let substring = token.substring(0, max_remaining);
            let text = substring.substring(0, substring.lastIndexOf(" "));

            span.text(text);

            length += text.length;
        } else {
            span.text(token);

            length += token.length;
        }
        
        return span;
    }).filter(span => span != null);

    if(body.length > 200) {
        let more = $("<span />");
        more.text("...");
        spans.push(more);
    }
    return spans;
}

function buildCommunitySearchResult(community) {
    let item = $("<li/>")
        .addClass("search-result");

    let container = $("<div/>");

    if(community.icon && isImage(community.icon)) {
        let community_icon = $("<img />")
            .attr("src", community.icon);
            container.append(community_icon);

        let divider1 = $("<span> | </span>");
        container.append(divider1);
    }

    let community_link = $("<a/>");
    if(community.actor_id.startsWith(home_instance)) {
        community_link.attr("href", community.actor_id);
    } else {
        let instance = new URL(community.actor_id).hostname;
        let href = home_instance + "c/" + community.name + "@" + instance;

        community_link.attr("href", href);
    }
    community_link.text(community.title ?? community.name);
    container.append(community_link);

    let divider2 = $("<span> @ </span>");
    container.append(divider2);

    let instance = getInstanceForCommunity(community);
    let instance_link = $("<a/>")
        .attr("href", `https://${instance}/`);
    instance_link.text(instance);
    container.append(instance_link);
    
    let divider3 = $("<span> | </span>");
    container.append(divider3);

    let number_of_matches = $("<span/>")
        .addClass("matches");
    number_of_matches.text(`${community.number_of_matches} matches`);
    container.append(number_of_matches);

    item.append(container);

    return item;
}

const INSTANCE_MATCH = new RegExp("^https:\/\/(?<domain>.+)\/c\/.*$");

function getInstanceForCommunity(community) {
    let matches = community.actor_id.match(INSTANCE_MATCH);
    if(matches) {
        return matches[1];
    } else {
        return null;
    }
}

function isImage(url) {
    return /\.(jpg|jpeg|png|webp|avif|gif|svg)$/.test(url);
}

function onInstanceChanged() {
    onSearch();
}

function onSearch() {
    let query = $("#search").val();

    let params = {
        "query" : query,
        "page" : 1
    };
    
    window.location = "/results?" + new URLSearchParams(params).toString();
}

function onReady() {

    const queryParameters = getQueryParameters();
    if(!queryParameters["query"]) {
        window.location = "/";
    }

    $("#search").val(queryParameters["query"]);

    query(
        queryParameters["query"],
        queryParameters["page"],
        queryParameters["mode"],
        home_instance
    );
}
