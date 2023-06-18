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
    populateInstances()
});
