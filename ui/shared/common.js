var home_instance = null;

function populateInstances() {
    fetchJson("/api/instances", result => {

        home_instance = getCookie("home-instance") || result[0].actor_id;
        if(!result.map(instance => instance.actor_id).includes(home_instance)) {
            home_instance = result[0].actor_id;
        }

        let select = $("#instance-select");
        select.empty();
        result.sort(instanceCompare)
            .forEach(instance => {
                let option = $("<option />")
                    .attr("value", instance.actor_id)
                    .prop("selected", instance.actor_id == home_instance);
                option.text(instance.name + " (" + dropSchema(instance.actor_id) + ")");

                select.append(option);
            })
        NiceSelect.bind(document.getElementById("instance-select"), {searchable: true, placeholder: 'Select your home instance', searchtext: 'Search for instances'});
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

function getVersion() {
    fetchJson("/api/version", result => {
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
    getDonationLink();
    populateInstances();
}

function getDonationLink() {
    fetchJson("/api/donate", result => {
        $("#donate")
            .attr("href", result.url)
            .text(result.text);
    })
}

$(document).ready(function() {

    initializeUI();
    populateInitialFields();

    home_instance = getCookie("home-instance");

    if(typeof onReady === "function") {
        onReady();
    }
});
