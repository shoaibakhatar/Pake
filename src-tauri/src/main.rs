#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
    app_lib::run_with(|app| {
        // get the main window created by app_lib
        let window = app.get_window("main").unwrap();

        // JS to inject:
        // 1) override window.open to load URL in same window
        // 2) intercept clicks on <a target="_blank"> and navigate in place
        // 3) capture cases where sites call window.open or create target _blank links dynamically
        let injected_js = r#"
(function () {
  try {
    // override window.open
    const originalOpen = window.open;
    window.open = function (url, name, features) {
      if (!url) return null;
      // navigate current window instead of opening a new one
      window.location.href = url;
      return {
        closed: false,
        focus() {},
        close() {}
      };
    };

    // intercept clicks on <a target="_blank">
    document.addEventListener('click', function (e) {
      let t = e.target;
      while (t && t.tagName !== 'A') t = t.parentElement;
      if (t && t.tagName === 'A' && t.target === '_blank' && t.href) {
        e.preventDefault();
        window.location.href = t.href;
      }
    }, true);

    // optional: intercept meta tags or other code that creates new windows
    const observer = new MutationObserver(function (mutations) {
      for (const m of mutations) {
        if (m.addedNodes) {
          m.addedNodes.forEach(node => {
            if (node.nodeType === 1 && node.tagName === 'A') {
              if (node.target === '_blank' && node.href) {
                node.addEventListener('click', function (e) {
                  e.preventDefault();
                  window.location.href = node.href;
                }, true);
              }
            }
          });
        }
      }
    });
    observer.observe(document, { childList: true, subtree: true });
  } catch (err) {
    // best effort — ignore errors
    console.error('injected script error', err);
  }
})();"#;

        // Evaluate the JS in the webview context
        // ignore the Result — it's best-effort (page might not be loaded yet)
        let _ = window.eval(injected_js);
    });
}

// Add to src-tauri/src/main.rs

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod window;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            window::go_back,
            window::go_forward,
            window::reload_page
        ])
        .setup(|app| {
            // Your existing setup code here
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
