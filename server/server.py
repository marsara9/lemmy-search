from wsgiref.simple_server import make_server
from server.search import Application

hostName = "0.0.0.0"
serverPort = 8080

if __name__ == "__main__":
    try:
        app = Application()

        with make_server(hostName, serverPort, app) as httpd:
            print(f"Serving on port {serverPort}...")
            httpd.serve_forever()
    except KeyboardInterrupt:
        print("Goodbye.")
