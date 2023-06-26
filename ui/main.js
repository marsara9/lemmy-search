var preferred_instance = "lemmy.world"

function populateInstances() {
    fetchJson("/instances", result => {
        for(instance in result) {
            $("#instance-select").append($("<option>", {
                value : instance.url,
                text : instance.name
            }))
        }
    })
}

$(document).ready(function() {
    $("#submit").click(function() {
        let query = $("#input").val()

        let params = {
            "query" : query,
            "preferred_instance" : preferred_instance,
            "page" : 1
        }
        
        window.location = "/results?" + new URLSearchParams(params).toString()
    })

    populateInstances()
});
