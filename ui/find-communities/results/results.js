function getQueryParameters() {
    const urlParameters = new URLSearchParams(window.location.search);
    return {
        "query": urlParameters.get("query"),
        "page":  urlParameters.get("page") || 1
    };
}

function query(queryString, page, instance) {

    queryParameters = new URLSearchParams({
        "query" : queryString,
        "page" : page,
        "home_instance" : dropSchema(instance)
    }).toString()

    fetchJson("/api/find/communities?" + queryParameters, result => {

        let response_time = Math.round((result.time_taken.secs + (result.time_taken.nanos / 1_000_000_000)) * 100) / 100;

        $("#response-time").text(
            `Found ${result.total_results} results in ${response_time} seconds`
        );

        let list = $("<ol/>");

        result.communities.forEach(community => {
            let item = buildSearchResult(community);
            list.append(item);
        });
        $("#results").empty();
        $("#results").append(list);

        buildPageControls(result.total_pages);
    })
}

function buildPageControls(total_pages) {
    const urlParameters = new URLSearchParams(window.location.search);
    let query = urlParameters.get("query");
    let page = Math.max(parseInt(urlParameters.get("page"), 10) || 1, 1);

    let page_control = $("#page-control")
        .empty();

    if(page > 1) {
        let params = {
            "query" : query,
            "home_instance" : dropSchema(home_instance),
            "page" : page - 1
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
            "home_instance" : dropSchema(home_instance),
            "page" : page + 1
        };
        
        let href = "/results?" + new URLSearchParams(params).toString();

        let next = $("<a />")
            .attr("href", href);

        next.text("Next >");

        page_control.append(next);
    }
}

function buildSearchResult(community) {
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
    return matches[1];
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
    
    window.location = "/find-communities/results?" + new URLSearchParams(params).toString();
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
        home_instance
    );
}
