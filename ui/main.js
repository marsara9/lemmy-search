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
        window.location = "/results?query="+encodeURIComponent(query)+"&page=1"
    })

    populateInstances()
});
