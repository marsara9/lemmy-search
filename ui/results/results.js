function checkQueryParamers() {
    const urlParameters = new URLSearchParams(window.location.search);
    return urlParameters.has("query")
}

function query(queryString) {
    fetchJson("/search" + queryString, result => {
        console.log(result)
    })
}

$(document).ready(function() {
    if (!checkQueryParamers()) {
        window.location = "/"
    }
});
