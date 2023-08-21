
$(document).ready(function() {

    const urlParameters = new URLSearchParams(window.location.search);

    let actor_id = urlParameters.get("actor_id");
    let actor_id_domain = new URL(actor_id).hostname;

    $("#source-link")
        .attr("href", actor_id);

    $("#source-instance")
        .text(actor_id_domain);
});