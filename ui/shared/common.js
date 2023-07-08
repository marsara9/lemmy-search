var home_instance = null;

function populateInstances() {
    fetchJson("/instances", result => {

        home_instance = getCookie("home-instance") || result[0].actor_id;
        if(!result.map(instance => instance.actor_id).includes(home_instance)) {
            home_instance = result[0].actor_id;
        }

        let select = $("#instance-select");
        result.sort(instanceCompare)
            .forEach(instance => {
                let option = $("<option />")
                    .attr("value", instance.actor_id)
                    .prop("selected", instance.actor_id == home_instance);
                option.text(instance.name + " (" + dropSchema(instance.actor_id) + ")");

                select.append(option);
            })
        $("#instance-select").selectize({
            sortField: 'text'
        });
    })
}

function instanceCompare(lhs, rhs) {
    if (lhs.name.toLowerCase() < rhs.name.toLowerCase()){
        return -1;
    }
    if (lhs.name.toLowerCase() > rhs.name.toLowerCase()){
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

        if(typeof onInstanceChanged === "function") {
            onInstanceChanged();
        }
    });
}

function populateInitialFields() {
    getVersion();
    populateInstances();
}

$(document).ready(function() {

    initializeUI();
    populateInitialFields();

    home_instance = getCookie("home-instance");

    if(typeof onReady === "function") {
        onReady();
    }
});