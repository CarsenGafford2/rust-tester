use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Mini Browser")
        .with_inner_size(tao::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    // HTML with search bar + iframe for content
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body { margin: 0; font-family: sans-serif; }
                #controls { padding: 8px; background: #eee; display: flex; }
                #url { flex: 1; padding: 4px; font-size: 16px; }
                #go { padding: 4px 12px; font-size: 16px; }
                #frame { width: 100%; height: calc(100vh - 48px); border: none; }
            </style>
        </head>
        <body>
            <div id="controls">
                <input id="url" type="text" placeholder="Enter URL or search...">
                <button id="go">Go</button>
            </div>
            <iframe id="frame"></iframe>

            <script>
                const input = document.getElementById('url');
                const button = document.getElementById('go');
                const frame = document.getElementById('frame');

                function navigate() {
                    let value = input.value.trim();
                    if (!value) return;

                    if (value.startsWith("http://") || value.startsWith("https://")) {
                        frame.src = value;
                    } else if (value.includes(".")) {
                        frame.src = "https://" + value;
                    } else {
                        frame.src = "https://google.com/?q=" + encodeURIComponent(value);
                    }
                }

                button.addEventListener('click', navigate);
                input.addEventListener('keydown', (e) => {
                    if (e.key === 'Enter') navigate();
                });
            </script>
        </body>
        </html>
    "#;

    let _webview = WebViewBuilder::new(&window)
        .with_html(html)
        .build()
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}