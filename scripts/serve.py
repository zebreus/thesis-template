 #!/usr/bin/env python3
    
import http.server
import sys

DIRECTORY = "."
INDEX_FILE = sys.argv[1]
    
class MyHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)
    
    def do_GET(self):
        if self.path == '/':
            self.path = '/' + INDEX_FILE
        return http.server.SimpleHTTPRequestHandler.do_GET(self)

    def end_headers(self):
        self.send_my_headers()
        http.server.SimpleHTTPRequestHandler.end_headers(self)
    
    def send_my_headers(self):
        self.send_header("Cache-Control", "no-cache, no-store, must-revalidate")
        self.send_header("Pragma", "no-cache")
        self.send_header("Expires", "0")
    
    
if __name__ == '__main__':
    http.server.test(HandlerClass=MyHTTPRequestHandler)