from tools.http import HttpTools
import os

class Application:

    def __init__(self):
        return

    def __call__(self, environ, start_response):
        http = HttpTools(environ, start_response)
        match http.request.method:
            case "GET":
                return self.get(http)
            case "POST":
                return self.post(http)
            case "PUT":
                return self.put(http)
        return http.send_basic_error(404, "Not Found")

    def get(self, http : HttpTools):
        match http.request.path:
            case _:
                filename = None
                try:
                    filename = http.fix_path(http.request.path)
                    if filename != None and len(filename) > 0:
                        return self.get_file(http, filename)
                except Exception as e:
                    return http.send_basic_error(404, f"Not found: '{filename}'", e)
        return http.send_json_error(404, "Not Found")

    def post(self, http : HttpTools):        
        return http.send_json_error(404, "Not Found")

    def put(self, http : HttpTools):
        return http.send_json_error(404, "Not Found")

    def get_file(self, http : HttpTools, filepath : str):
 
        if filepath.endswith(".html"):
            type = "text/html"
        elif filepath.endswith(".js"):
            type = "text/javascript"
        elif filepath.endswith(".css"):
            type = "text/css"
        elif filepath.endswith(".png"):
            type = "image/png"
        elif filepath.endswith(".svg"):
            type = "image/svg+xml"
        else:
            type = None

        filename = os.getcwd() + os.sep + filepath
        print(f"\033[92mLoading static file '{filename}'\033[0m")
        with open(filename, "rb") as file:
            file_content = file.read()
            http.start_response("200 OK", [
                ("Content-Type", type),
                ("Content-Length", str(len(file_content)))
            ])
            return [file_content]
