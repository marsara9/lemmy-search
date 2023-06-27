var preferred_instance = "lemmy.world"

function populateInstances() {
    fetchJson("/instances", result => {

        preferred_instance = getCookie("preferred-instance");

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

$(document).ready(function() {
    $("#submit").click(function() {
        let query = $("#search").val();

        let params = {
            "query" : query,
            "preferred_instance" : preferred_instance,
            "page" : 1
        };
        
        window.location = "/results?" + new URLSearchParams(params).toString();
    });

    $("instance-select").on("change", function() {
        preferred_instance = this.value;
        setCookie("preferred-instance", instance-select);
    });

    populateInstances();
});
