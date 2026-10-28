#!/usr/bin/env python3
"""Serveur de développement local avec Live Reload automatique (SSE).

- Surveille le dossier 'ui/' en temps réel.
- Injecte à la volée un script de reload dans les pages HTML.
- Rafraîchit les feuilles de style CSS instantanément (hot-reload sans recharger la page).
- Recharge la page complète dès qu'un fichier HTML ou JS est modifié.
"""

import http.server
import os
import sys
import threading
import time
from pathlib import Path

# Remonte à la racine du dépôt depuis tools/dev/
ROOT_DIR = Path(__file__).resolve().parent.parent.parent / "ui"
PORT = int(os.environ.get("PORT", 7001))

SUBSCRIBERS = []
SUBSCRIBERS_LOCK = threading.Lock()

RELOAD_SCRIPT = b"""
<script>
(() => {
  const es = new EventSource('/__livereload');
  es.onmessage = (e) => {
    const data = JSON.parse(e.data);
    if (data.type === 'css') {
      document.querySelectorAll('link[rel="stylesheet"]').forEach(link => {
        const url = new URL(link.href, location.href);
        url.searchParams.set('_t', Date.now());
        link.href = url.href;
      });
    } else {
      location.reload();
    }
  };
})();
</script>
</body>
"""

class LiveReloadHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(ROOT_DIR), **kwargs)

    def do_GET(self):
        if self.path == "/__livereload":
            self.send_response(200)
            self.send_header("Content-Type", "text/event-stream")
            self.send_header("Cache-Control", "no-cache")
            self.send_header("Connection", "keep-alive")
            self.send_header("Access-Control-Allow-Origin", "*")
            self.end_headers()

            queue = []
            lock = threading.Condition()
            with SUBSCRIBERS_LOCK:
                SUBSCRIBERS.append((queue, lock))

            try:
                self.wfile.write(b": ping\n\n")
                self.wfile.flush()
                while True:
                    with lock:
                        while not queue:
                            lock.wait(timeout=15)
                            if not queue:
                                self.wfile.write(b": ping\n\n")
                                self.wfile.flush()
                        while queue:
                            msg = queue.pop(0)
                            self.wfile.write(f"data: {msg}\n\n".encode("utf-8"))
                            self.wfile.flush()
            except (BrokenPipeError, ConnectionResetError):
                pass
            finally:
                with SUBSCRIBERS_LOCK:
                    if (queue, lock) in SUBSCRIBERS:
                        SUBSCRIBERS.remove((queue, lock))
            return

        return super().do_GET()

    def send_head(self):
        path = self.translate_path(self.path)
        if os.path.isdir(path):
            index = os.path.join(path, "index.html")
            if os.path.exists(index):
                path = index

        if path.endswith(".html") and os.path.exists(path):
            try:
                with open(path, "rb") as f:
                    content = f.read()
                if b"</body>" in content:
                    content = content.replace(b"</body>", RELOAD_SCRIPT)
                else:
                    content = content + RELOAD_SCRIPT

                self.send_response(200)
                self.send_header("Content-Type", "text/html; charset=utf-8")
                self.send_header("Content-Length", str(len(content)))
                self.send_header("Cache-Control", "no-store")
                self.end_headers()
                
                import io
                return io.BytesIO(content)
            except OSError:
                pass

        response = super().send_head()
        if self.path.endswith(".css"):
            self.send_header("Cache-Control", "no-cache, must-revalidate")
        return response

    def log_message(self, format, *args):
        if len(args) > 0 and "/__livereload" in str(args[0]):
            return
        super().log_message(format, *args)


def broadcast(msg: str):
    with SUBSCRIBERS_LOCK:
        for queue, lock in SUBSCRIBERS:
            with lock:
                queue.append(msg)
                lock.notify()


def file_watcher():
    """Surveille les modifications de fichiers dans ui/."""
    mtimes = {}

    def scan():
        current = {}
        for p in ROOT_DIR.rglob("*"):
            if p.is_file():
                try:
                    current[p] = p.stat().st_mtime
                except OSError:
                    pass
        return current

    mtimes = scan()
    while True:
        time.sleep(0.15)
        current = scan()
        changed = []
        for p, mt in current.items():
            if p not in mtimes or mtimes[p] != mt:
                changed.append(p)
        mtimes = current

        if changed:
            css_only = all(p.suffix == ".css" for p in changed)
            event_type = "css" if css_only else "reload"
            names = ", ".join(p.name for p in changed[:3])
            print(f"[LiveReload] Modification ({names}) -> notification ({event_type})")
            broadcast(f'{{"type": "{event_type}"}}')


def main():
    threading.Thread(target=file_watcher, daemon=True).start()
    server = http.server.ThreadingHTTPServer(("127.0.0.1", PORT), LiveReloadHandler)
    print(f"[LiveReload] Serveur démarré sur http://localhost:{PORT}/ (Dossier : {ROOT_DIR})")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nArrêt du serveur.")


if __name__ == "__main__":
    main()
