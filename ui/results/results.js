var preferred_instance = null;

function checkQueryParameters() {
    const urlParameters = new URLSearchParams(window.location.search);
    $("#search").val(urlParameters.get("query"));
    return urlParameters.has("query");
}

function populateInstances() {
    fetchJson("/instances", result => {

        preferred_instance = getCookie("preferred-instance") ?? result[0].site.actor_id;

        let select = $("#instance-select");
        result.forEach(instance => {
            let option = $("<option />")
                .attr("value", instance.site.actor_id)
                .prop("selected", instance.site.actor_id == preferred_instance);
            option.text(instance.site.name);

            select.append(option);
        })
    })
}

function query(queryString) {
    fetchJson("/search" + queryString, result => {

        let response_time = Math.round((result.time_taken.secs + (result.time_taken.nanos / 1_000_000_000)) * 100) / 100;

        $("#response-time").text(
            "Found " + result.posts.length + " results in " + response_time + " seconds"
        );

        let list = $("<ol/>");

        result.posts.forEach(post => {
            let item = buildSearchResult(post, result.original_query_terms);
            list.append(item);
        });
        $("#results").empty();
        $("#results").append(list);
    })
}

function buildSearchResult(post, original_query_terms) {
    let item = $("<li/>")
        .addClass("search-result");
    if (post.ur && isImage(post.url)) {
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

    if(post.author.avatar && isImage(post.author.avatar)) {
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

    if(post.community.icon && isImage(post.community.icon)) {
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
        post_body.append(getPostQueryBody(original_query_terms, post.body));
    }
    item.append(post_body);

    return item;
}

function getPostQueryBody(queryTerms, body) {
    let regex = "(\s" + queryTerms.join("\s)|(\s") + "\s)";
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

function isImage(url) {
    return /\.(jpg|jpeg|png|webp|avif|gif|svg)$/.test(url);
}

function dropSchema(instance_actor_id) {
    return instance_actor_id.substring(8, instance_actor_id.length-1);
}

$(document).ready(function() {
    let header = $(".header");
    let contentPlacement = header.position().top + header.outerHeight();
    $('#results').css('margin-top',contentPlacement);

    if (!checkQueryParameters()) {
        window.location = "/";
        return;
    }

    $("#submit").click(function() {
        let query = $("#search").val();

        let params = {
            "query" : query,
            "preferred_instance" : dropSchema(preferred_instance),
            "page" : 1
        };
        
        window.location = "/results?" + new URLSearchParams(params).toString();
    });

    $("#instance-select").on("change", function() {
        preferred_instance = this.value;
        setCookie("preferred-instance", instance-select);
    });

    populateInstances();

    query(window.location.search);
});
