
function onSearch() {
    let query = $("#search").val();

    let params = {
        "query" : query,
        "page" : 1
    };
    
    window.location = "/results?" + new URLSearchParams(params).toString();
}

function onReady() {
    
}
