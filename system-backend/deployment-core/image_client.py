"""The unprivileged controller talks to the VM builder over a Unix socket only."""
import http.client
import json
import socket

SOCKET = '/run/netcore-image-builder/api.sock'


class Connection(http.client.HTTPConnection):
    def __init__(self, path=SOCKET, timeout=20):
        super().__init__('localhost', timeout=timeout)
        self.path = path

    def connect(self):
        self.sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.sock.settimeout(self.timeout)
        self.sock.connect(self.path)


class ImageClient:
    def __init__(self, path=SOCKET):
        self.path = path

    def request(self, path, data=None):
        connection = Connection(self.path)
        try:
            body = None if data is None else json.dumps(data)
            connection.request('GET' if data is None else 'POST', path, body,
                               {'Content-Type': 'application/json'})
            response = connection.getresponse()
            result = json.loads(response.read(2 * 1024 * 1024))
            if response.status >= 400:
                raise ValueError(result.get('error', 'Imagebuilder-Anfrage fehlgeschlagen'))
            return result
        except (OSError, http.client.HTTPException) as exc:
            raise OSError('Imagebuilder nicht erreichbar. Auf der Ubuntu-VM install/install-vm.sh ausführen; '
                          'systemctl status netcore-image-builder prüfen.') from exc
        finally:
            connection.close()

    def status(self):
        try:
            return self.request('/status')
        except OSError as exc:
            return {'available': False, 'error': str(exc), 'jobs': [], 'artifacts': []}

    def download(self, path, target):
        connection = Connection(self.path, timeout=60)
        try:
            headers = {'Range': target.headers['Range']} if target.headers.get('Range') else {}
            connection.request('GET', path, headers=headers)
            response = connection.getresponse()
            target.send_response(response.status)
            for key in ('Content-Type', 'Content-Length', 'Content-Disposition', 'Content-Range', 'Accept-Ranges'):
                if response.getheader(key):
                    target.send_header(key, response.getheader(key))
            target.send_header('Cache-Control', 'no-store')
            target.send_header('X-Content-Type-Options', 'nosniff')
            target.end_headers()
            while chunk := response.read(1024 * 1024):
                target.wfile.write(chunk)
        finally:
            connection.close()
