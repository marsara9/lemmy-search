var preferred_instance = "lemmy.world"

function populateInstances() {
    fetchJson("/instances", result => {
        let select = $("#instance-select");
        result.forEach(instance => {
            let option = $("<option />")
                .attr("value", instance.site.actor_id);
            option.text(instance.site.name);

            select.append(option);
        })
    })
}

$(document).ready(function() {
    $("#submit").click(function() {
        let query = $("#search").val()

        let params = {
            "query" : query,
            "preferred_instance" : preferred_instance,
            "page" : 1
        }
        
        window.location = "/results?" + new URLSearchParams(params).toString()
    })

    populateInstances()
});
