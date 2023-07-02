

$(document).ready(function() {
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
        setCookie("preferred-instance", preferred_instance);
    });

    populateInstances();
});
