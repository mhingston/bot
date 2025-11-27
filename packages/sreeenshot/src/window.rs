use anyhow::Context;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    window::{Window, WindowAttributes},
};

pub fn create_fullscreen_window(
    event_loop: &winit::event_loop::ActiveEventLoop,
    size: PhysicalSize<u32>,
) -> anyhow::Result<Window> {
    // Get primary monitor to get its position
    let primary_monitor = event_loop
        .primary_monitor()
        .or_else(|| event_loop.available_monitors().next());
    
    let position = if let Some(monitor) = primary_monitor {
        // Get monitor position (logical coordinates)
        monitor.position()
    } else {
        // Fallback to (0, 0)
        PhysicalPosition::new(0, 0)
    };
    
    let window = event_loop
        .create_window(
            WindowAttributes::default()
                .with_inner_size(size)
                .with_position(position)
                .with_title("Screenshot")
                .with_resizable(false)
                .with_decorations(false)
                .with_visible(false),
        )
        .context("Failed to create window")?;

    // Configure macOS-specific window properties to hide from dock
    #[cfg(target_os = "macos")]
    {
        configure_macos_window(&window);
    }

    Ok(window)
}

#[cfg(target_os = "macos")]
fn configure_macos_window(window: &Window) {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    
    let handle = match window.window_handle() {
        Ok(h) => h,
        Err(_) => return,
    };
    
    if let RawWindowHandle::AppKit(handle) = handle.as_raw() {
        unsafe {
            use objc::runtime::{Class, Object};
            use objc::{msg_send, sel, sel_impl};
            
            // Get NSWindow from NSView
            let ns_view: *mut Object = handle.ns_view.as_ptr() as *mut Object;
            let ns_window: *mut Object = msg_send![ns_view, window];
            
            let ns_app_class = match Class::get("NSApplication") {
                Some(c) => c,
                None => return,
            };
            let ns_app: *mut Object = msg_send![ns_app_class, sharedApplication];
            
            // Set activation policy to accessory (doesn't show in dock)
            // NSApplicationActivationPolicyAccessory = 1
            let _: () = msg_send![ns_app, setActivationPolicy: 1u64];
            
            // Set window level to cover everything including menu bar
            // Use kCGOverlayWindowLevelKey (25) to cover menu bar and dock
            let _: () = msg_send![ns_window, setLevel: 25i64];
            
            // Set collection behavior to stay on all spaces (don't move to new space)
            // NSWindowCollectionBehaviorCanJoinAllSpaces = 128
            let _: () = msg_send![ns_window, setCollectionBehavior: 128u64];
            
            // Make window background completely transparent
            // We'll handle the overlay effect in the rendering code
            let ns_color_class = match Class::get("NSColor") {
                Some(c) => c,
                None => return,
            };
            let clear_color: *mut Object = msg_send![ns_color_class, clearColor];
            let _: () = msg_send![ns_window, setBackgroundColor: clear_color];
            
            // Make window non-opaque to allow transparency
            let _: () = msg_send![ns_window, setOpaque: false];
            
            // Enable window transparency
            let _: () = msg_send![ns_window, setHasShadow: false];
            
            // Set contentView to be transparent
            let content_view: *mut Object = msg_send![ns_window, contentView];
            if !content_view.is_null() {
                let _: () = msg_send![content_view, setWantsLayer: true];
            }
        }
    }
}

