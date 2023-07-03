var preferred_instance = null;

function populateInstances() {
    fetchJson("/instances", result => {

        preferred_instance = getCookie("preferred-instance") || result[0].site.actor_id;
        if(!result.map(instance => instance.site.actor_id).includes(preferred_instance)) {
            preferred_instance = result[0].site.actor_id;
        }

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

function dropSchema(instance_actor_id) {
    return instance_actor_id.substring(8, instance_actor_id.length-1);
}

function onSearch() {
    let query = $("#search").val();

    let params = {
        "query" : query,
        "preferred_instance" : dropSchema(preferred_instance),
        "page" : 1
    };
    
    window.location = "/results?" + new URLSearchParams(params).toString();
}

function getVersion() {
    fetchJson("/version", result => {
        $("#version").text(result.version);
    });
}
