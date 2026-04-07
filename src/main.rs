use std::sync::{Arc, Mutex};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;
use std::sync::mpsc;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Mini Browser")
        .with_inner_size(tao::dpi::LogicalSize::new(1200.0, 800.0))
        .build(&event_loop)
        .unwrap();

    let html = r#"
        <!DOCTYPE html>
        <html>
        <head style="margin:0; padding:0;">
            <style>
                * { box-sizing: border-box; }
                body { margin:0; padding:0; font-family:Segoe UI, sans-serif; height:100vh; display:flex; flex-direction:column; }
                #toolbar { display:flex; gap:10px; padding:12px; background:#f7f7f7; border-bottom:1px solid #ddd; flex-shrink:0; }
                #url { flex:1; font-size:18px; padding:8px; border:1px solid #bbb; border-radius:4px; }
                #go { font-size:16px; padding:8px 16px; cursor:pointer; background:#007bff; color:white; border:none; border-radius:4px; }
                #go:hover { background:#0056b3; }
                #content { flex:1; overflow:auto; padding:20px; }
                #loading { color:#666; }
                #error { color:red; }
            </style>
        </head>
        <body>
            <div id="toolbar">
                <input id="url" placeholder="Enter URL or search" />
                <button id="go">Go</button>
            </div>
            <div id="content">
                <p id="loading" style="display:none;">Loading...</p>
                <div id="page"></div>
            </div>

            <script>
                function go() {
                    const raw = document.getElementById('url').value.trim();
                    if (!raw) return;

                    let finalUrl;
                    if (raw.startsWith('http://') || raw.startsWith('https://')) {
                        finalUrl = raw;
                    } else if (raw.includes('.')) {
                        finalUrl = `https://${raw}`;
                    } else {
                        finalUrl = `https://www.google.com/search?q=${encodeURIComponent(raw)}`;
                    }

                    document.getElementById('loading').style.display = 'block';
                    document.getElementById('page').innerHTML = '';
                    window.ipc.postMessage(finalUrl);
                }

                window.showContent = function(content) {
                    document.getElementById('loading').style.display = 'none';
                    document.getElementById('page').innerHTML = content;
                };

                window.showError = function(error) {
                    document.getElementById('loading').style.display = 'none';
                    document.getElementById('page').innerHTML = '<p style="color:red;">Error: ' + error + '</p>';
                };

                document.getElementById('url').addEventListener('keydown', function(event) {
                    if (event.key === 'Enter') go();
                });

                document.getElementById('go').addEventListener('click', go);
            </script>
        </body>
        </html>
    "#;

    let (tx, rx) = mpsc::channel();
    let tx_clone = Arc::new(Mutex::new(tx));

    let webview = WebViewBuilder::new(&window)
        .with_html(html)
        .with_ipc_handler(move |_req| {
            let url = _req.body().to_string();
            let tx = Arc::clone(&tx_clone);
            
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    match reqwest::get(&url).await {
                        Ok(resp) => {
                            match resp.text().await {
                                Ok(html) => {
                                    let _ = tx.lock().unwrap().send(("success".to_string(), html));
                                }
                                Err(e) => {
                                    let _ = tx.lock().unwrap().send(("error".to_string(), format!("Failed to read response: {}", e)));
                                }
                            }
                        }
                        Err(e) => {
                            let _ = tx.lock().unwrap().send(("error".to_string(), format!("Failed to fetch: {}", e)));
                        }
                    }
                });
            });
        })
        .build()
        .unwrap();

    let webview = Arc::new(Mutex::new(webview));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Check for responses from the fetch thread
        if let Ok((status, content)) = rx.try_recv() {
            let script = if status == "success" {
                format!(r#"window.showContent({});"#, serde_json::to_string(&content).unwrap_or_else(|_| "''".to_string()))
            } else {
                format!(r#"window.showError({});"#, serde_json::to_string(&content).unwrap_or_else(|_| "''".to_string()))
            };
            if let Ok(wv) = webview.lock() {
                let _ = wv.evaluate_script(&script);
            }
        }

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}