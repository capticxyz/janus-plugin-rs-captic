/// Utilities to make it easier to maintain Janus session state between plugin callbacks.

use std::ops::Deref;
use std::sync::Arc;
use super::PluginHandle;

/// A representation of session state associated with a Janus plugin session handle.
#[derive(Debug)]
pub struct SessionHandle<T> {
    pub handle: *mut PluginHandle,
    state: T,
}

impl<T> SessionHandle<T> {

    /// Allocates a boxed, reference-counted SessionHandle associated with an opaque Janus handle
    /// (whose plugin_handle will then point to the contents of the box).
    pub fn establish(handle: *mut PluginHandle, state: T) -> Box<Arc<Self>> {
        let result = Box::new(Arc::new(Self { handle, state: state }));
        unsafe { (*handle).plugin_handle = result.as_ref() as *const _ as *mut _ };
        result
    }

    /// Retrieves the reference-counted SessionHandle pointed to by an opaque Janus handle.
    pub fn from_ptr<'a>(handle: *mut PluginHandle) -> &'a Arc<Self> {
        unsafe { &*((*handle).plugin_handle as *mut Arc<Self>) }
    }
}

impl<T> Deref for SessionHandle<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.state
    }
}

// the pointer is opaque to Janus code, so this handle is threadsafe to the extent that the state is

unsafe impl<T: Sync> Sync for SessionHandle<T> {}
unsafe impl<T: Send> Send for SessionHandle<T> {}