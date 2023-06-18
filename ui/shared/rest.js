function restJson(url, method, body, showSpinner, onResult, onError) {

    let params = {
        method: method,
        credentials: "same-origin",
        cache: "no-cache",
        headers: {}
    }
    if(body) {
        params.body = JSON.stringify(body)
        params.headers["content-type"] = "application/json"
    }

    if(showSpinner) {
        $("#loading-dialog")[0].showModal()
    }

    fetch(url, params).then(response => {        
        if(response.headers.get("content-type") == "application/json") {
            response.json().then(data => {
                if(!response.ok) {
                    let input = $(`#${data.parameter}`)
                    input.addClass("error")
                    if(onError) {
                        onError(data)
                    }
                } else {
                    if(onResult) {
                        onResult(data)
                    }
                }
            })
        } else {
            if(response.ok && onResult) {
                onResult()
            }
            else if(!response.ok && onError) {
                onError(response.body)
            }
        }
        if(response.status == 401) {
            logout()          
        }
    }).then(() => {
        if(showSpinner) {
            $("#loading-dialog")[0].close()
        }
    })
}

function fetchJson(url, onResult, onError) {
    restJson(url, "GET", null, false, onResult, onError)
}

function postJson(url, data, onResult, onError) {
    restJson(url, "POST", data, true, onResult, onError)
}

function putJson(url, data, onResult, onError) {
    restJson(url, "PUT", data, true, onResult, onError)
}

function deleteJson(url, data, onResult, onError) {
    restJson(url, "DELETE", data, true, onResult, onError)
}
