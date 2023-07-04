var preferred_instance = null;

function populateInstances() {
    fetchJson("/instances", result => {

        preferred_instance = getCookie("preferred-instance") || result[0].site.actor_id;
        if(!result.map(instance => instance.site.actor_id).includes(preferred_instance)) {
            preferred_instance = result[0].site.actor_id;
        }

        let select = $("#instance-select");
        result
            .sort(instanceCompare)
            .forEach(instance => {
                let option = $("<option />")
                    .attr("value", instance.site.actor_id)
                    .prop("selected", instance.site.actor_id == preferred_instance);
                option.text(instance.site.name + " (" + dropSchema(instance.site.actor_id) + ")");

                select.append(option);
            })
    })
}

function instanceCompare(lhs, rhs) {
    if (lhs.site.name.toLowerCase() < rhs.site.name.toLowerCase()){
        return -1;
    }
    if (lhs.site.name.toLowerCase() > rhs.site.name.toLowerCase()){
        return 1;
    }
    return 0;
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

function initializeUI() {
    $("#submit").click(function() {
        onSearch();
    });

    $("#search").keydown(function(e){
        if(e.keyCode == 13) {
            onSearch();
        }
    });

    $("#instance-select").on("change", function() {
        preferred_instance = this.value;
        setCookie("preferred-instance", preferred_instance, 3652);
    });
}

function populateInitialFields() {
    getVersion();
    populateInstances();
}

$(document).ready(function() {
    initializeUI();

    populateInitialFields();

    if(onReady) {
        onReady();
    }
});