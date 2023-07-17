function restJson(url, method, onResult, onError) {

    let params = {
        method: method,
        headers: {}
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
    })
}

function fetchJson(url, onResult, onError) {
    restJson(url, "GET", null, false, onResult, onError)
}
