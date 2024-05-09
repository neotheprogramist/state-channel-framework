from http.server import BaseHTTPRequestHandler, HTTPServer

class RequestHandler(BaseHTTPRequestHandler):
    def _log_request(self):
        print(f"Received {self.command} request for {self.path}")
        print("Headers:")
        for header, value in self.headers.items():
            print(f"{header}: {value}")

    def do_GET(self):
        self._log_request()
        self.send_response(200)
        self.send_header('Content-type', 'text/html')
        self.end_headers()
        self.wfile.write(bytes("<html><body><h1>GET request received</h1></body></html>", "utf8"))

    def do_POST(self):
        self._log_request()
        content_length = int(self.headers['Content-Length'])
        post_data = self.rfile.read(content_length)
        print("Body:")
        print(post_data.decode('utf-8'))

        self.send_response(200)
        self.send_header('Content-type', 'text/html')
        self.end_headers()
        self.wfile.write(bytes("<html><body><h1>POST request received</h1></body></html>", "utf8"))

def run(server_class=HTTPServer, handler_class=RequestHandler, port=8080):
    server_address = ('', port)
    httpd = server_class(server_address, handler_class)
    print(f"Starting http server on port {port}...")
    httpd.serve_forever()

if __name__ == '__main__':
    run()
