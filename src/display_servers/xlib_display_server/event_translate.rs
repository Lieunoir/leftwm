use super::DisplayEvent;
use super::Window;
use super::WindowHandle;
use super::XWrap;
use crate::utils::logging::*;
use x11_dl::xlib;

pub fn from_xevent(xw: &XWrap, raw_event: xlib::XEvent) -> Option<DisplayEvent> {
    match raw_event.get_type() {
        // new window is created
        xlib::MapRequest => {
            let event = xlib::XMapRequestEvent::from(raw_event);
            match xw.get_window_attrs(event.window) {
                Ok(attr) => {
                    if attr.override_redirect > 0 {
                        None
                    } else {
                        let name = xw.get_window_name(event.window);
                        //let _trans = xw.get_transient_for(event.window);
                        let w = Window::new(WindowHandle::XlibHandle(event.window), name);
                        Some(DisplayEvent::WindowCreate(w))
                    }
                }
                Err(_) => None,
            }
        }

        // window is deleted
        xlib::UnmapNotify => {
            let event = xlib::XUnmapEvent::from(raw_event);
            let _h = WindowHandle::XlibHandle(event.window);
            log_xevent(&format!("UnmapNotify {:?}", event));
            None
            //Some(EventQueueItem::WindowDelete(h))
        }

        xlib::CreateNotify => {
            let _event = xlib::XCreateWindowEvent::from(raw_event);
            //log_xevent(&format!("CreateNotify_EVENT: {:?}", event));
            None
            //if event.parent == kw.get_default_root() {
            //    let name = xw.get_window_name(event.window);
            //    let w = Window::new(WindowHandle::XlibHandle(event.window), name);
            //    Some(DisplayEvent::WindowCreate(w))
            //} else {
            //    None
            //}
        }

        // window is deleted
        xlib::DestroyNotify => {
            //log_xevent( &format!("DestroyNotify") );
            let event = xlib::XDestroyWindowEvent::from(raw_event);
            log_xevent(&format!("DestroyNotify: {:#?}", event));
            let h = WindowHandle::XlibHandle(event.window);
            //let h = WindowHandle::XlibHandle(event.window + 2);
            Some(DisplayEvent::WindowDestroy(h))
        }

        xlib::ClientMessage => {
            let event = xlib::XClientMessageEvent::from(raw_event);
            log_xevent(&format!("ClientMessage: {:#?} ", event));
            None
        }

        xlib::ButtonPress => {
            let event = xlib::XButtonPressedEvent::from(raw_event);
            log_xevent(&format!("ButtonPress: {:#?} ", event));
            None
        }
        xlib::EnterNotify => {
            let event = xlib::XEnterWindowEvent::from(raw_event);
            log_xevent(&format!("EnterNotify: {:#?} ", event));
            let h = WindowHandle::XlibHandle(event.window);
            let mouse_loc = xw.get_pointer_location();
            match mouse_loc {
                Some(loc) => Some(DisplayEvent::FocusedWindow(h, loc.0, loc.1)),
                None => None,
            }
        }
        xlib::LeaveNotify => {
            let event = xlib::XLeaveWindowEvent::from(raw_event);
            log_xevent(&format!("LeaveNotify: {:#?} ", event));
            None
        }
        xlib::PropertyNotify => {
            let event = xlib::XPropertyEvent::from(raw_event);
            log_xevent(&format!("PropertyNotify: {:#?} ", event));
            None
        }

        xlib::MapNotify => {
            let event = xlib::XMappingEvent::from(raw_event);
            log_xevent(&format!("MapNotify: {:#?} ", event));
            None
        }
        xlib::KeyPress => {
            let event = xlib::XKeyEvent::from(raw_event);
            let sym = xw.keycode_to_keysym(event.keycode);
            //log_xevent( &format!("KeyPress: {:#?} ", event) );
            Some(DisplayEvent::KeyCombo(event.state, sym))
        }
        xlib::KeyRelease => {
            let event = xlib::XKeyEvent::from(raw_event);
            log_xevent(&format!("release: {:#?} ", event));
            None
        }
        xlib::ButtonRelease => {
            log_xevent(&format!("ButtonRelease"));
            None
        }
        xlib::MotionNotify => {
            let event = xlib::XMotionEvent::from(raw_event);
            let h = WindowHandle::XlibHandle(event.window);
            Some(DisplayEvent::Movement(h, event.x_root, event.y_root))
        }
        xlib::FocusIn => {
            let event = xlib::XFocusChangeEvent::from(raw_event);
            let h = WindowHandle::XlibHandle(event.window);
            //log_xevent( &format!("FocusIn: {:#?} ", event) );
            //Some(DisplayEvent::FocusedWindow(h))
            let mouse_loc = xw.get_pointer_location();
            match mouse_loc {
                Some(loc) => Some(DisplayEvent::FocusedWindow(h, loc.0, loc.1)),
                None => None,
            }
        }
        xlib::FocusOut => {
            log_xevent(&format!("FocusOut"));
            None
        }
        xlib::KeymapNotify => {
            log_xevent(&format!("KeymapNotify"));
            None
        }
        xlib::Expose => {
            log_xevent(&format!("Expose"));
            None
        }
        xlib::GraphicsExpose => {
            log_xevent(&format!("GraphicsExpose"));
            None
        }
        xlib::NoExpose => {
            log_xevent(&format!("NoExpose"));
            None
        }
        xlib::VisibilityNotify => {
            log_xevent(&format!("VisibilityNotify"));
            None
        }
        xlib::ReparentNotify => {
            log_xevent(&format!("ReparentNotify"));
            None
        }
        xlib::ConfigureNotify => {
            let event = xlib::XConfigureEvent::from(raw_event);
            log_xevent(&format!("ConfigureNotify: {:#?}", event));
            None
        }
        xlib::ConfigureRequest => {
            log_xevent(&format!("ConfigureRequest"));
            None
        }
        xlib::GravityNotify => {
            log_xevent(&format!("GravityNotify"));
            None
        }
        xlib::ResizeRequest => {
            log_xevent(&format!("ResizeRequest"));
            None
        }
        xlib::CirculateNotify => {
            log_xevent(&format!("CirculateNotify"));
            None
        }
        xlib::CirculateRequest => {
            log_xevent(&format!("CirculateRequest"));
            None
        }
        xlib::SelectionClear => {
            log_xevent(&format!("SelectionClear"));
            None
        }
        xlib::SelectionRequest => {
            log_xevent(&format!("SelectionRequest"));
            None
        }
        xlib::SelectionNotify => {
            log_xevent(&format!("SelectionNotify"));
            None
        }
        xlib::ColormapNotify => {
            log_xevent(&format!("ColormapNotify"));
            None
        }
        xlib::MappingNotify => {
            log_xevent(&format!("MappingNotify"));
            None
        }
        xlib::GenericEvent => {
            log_xevent(&format!("GenericEvent"));
            None
        }
        _ => None,
    }
}
