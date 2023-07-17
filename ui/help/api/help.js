function populateInitialFields() {
    getVersion();
    getDonationLink();
}

function getDonationLink() {
    fetchJson("/api/donate", result => {
        $("#donate")
            .attr("href", result.url)
            .text(result.text);
    })
}

function getVersion() {
    fetchJson("/api/version", result => {
        $("#version").text(result.version);
    });
}

$(document).ready(function() {
    populateInitialFields();
});
