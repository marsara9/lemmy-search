
function onSearch() {
    let query = $("#search").val();

    let params = {
        "query" : query,
        "page" : 1,
        "mode": $("input[name='mode']:checked").val()
    };
    
    window.location = "/results?" + new URLSearchParams(params).toString();
}

function onReady() {
    
}
