var home_instance = null;

function populateInstances() {
    fetchJson("/instances", result => {

        home_instance = getCookie("home-instance") || result[0].site.actor_id;
        if(!result.map(instance => instance.site.actor_id).includes(home_instance)) {
            home_instance = result[0].site.actor_id;
        }

        let select = $("#instance-list");
        result.sort(instanceCompare)
            .forEach(instance => {
                let option = $("<option />")
                    .attr("value", instance.site.actor_id)
                    .prop("selected", instance.site.actor_id == home_instance);
                option.text(instance.site.name + " (" + dropSchema(instance.site.actor_id) + ")");

                select.append(option);
            })
        $("#instance-select").val(home_instance);

        if(onReady) {
            onReady();
        }
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
        "home_instance" : dropSchema(home_instance),
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
    $( "#search-form" ).on( "submit", function( event ) {
        onSearch();
        event.preventDefault();
      });

    $("#instance-select").on("change", function() {
        home_instance = this.value;
        setCookie("home-instance", home_instance, 3652);
    });
}

function populateInitialFields() {
    getVersion();
    populateInstances();
}

$(document).ready(function() {
    initializeUI();

    populateInitialFields();
});