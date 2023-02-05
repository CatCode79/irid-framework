//= USES =====================================================================

use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    error::{ExternalError, NotSupportedError, OsError},
    monitor::MonitorHandle,
    window::{CursorGrabMode, CursorIcon, UserAttentionType, WindowId},
};

//= IRID WINDOW BUILDER ======================================================

///
#[derive(Clone, Debug)]
pub struct IridWindowConfig {
    winit_builder: winit::window::WindowBuilder,
}

impl Default for IridWindowConfig {
    fn default() -> Self {
        Self {
            winit_builder: Default::default(),
        }
    }
}

impl IridWindowConfig {
    pub fn new() -> Self {
        IridWindowConfig::default().with_title("Irid Application")
    }

    //- Setters --------------------------------------------------------------

    pub fn with_inner_size<S: Into<winit::dpi::Size>>(mut self, size: S) -> Self {
        self.winit_builder = self.winit_builder.with_inner_size(size);
        self
    }

    pub fn with_min_inner_size<S: Into<winit::dpi::Size>>(mut self, min_size: S) -> Self {
        self.winit_builder = self.winit_builder.with_min_inner_size(min_size);
        self
    }

    pub fn with_max_inner_size<S: Into<winit::dpi::Size>>(mut self, max_size: S) -> Self {
        self.winit_builder = self.winit_builder.with_max_inner_size(max_size);
        self
    }

    pub fn with_position<P: Into<winit::dpi::Position>>(mut self, position: P) -> Self {
        self.winit_builder = self.winit_builder.with_position(position);
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.winit_builder = self.winit_builder.with_resizable(resizable);
        self
    }

    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.winit_builder = self.winit_builder.with_title(title);
        self
    }

    pub fn with_fullscreen(mut self, fullscreen: Option<winit::window::Fullscreen>) -> Self {
        self.winit_builder = self.winit_builder.with_fullscreen(fullscreen);
        self
    }

    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.winit_builder = self.winit_builder.with_maximized(maximized);
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.winit_builder = self.winit_builder.with_visible(visible);
        self
    }

    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.winit_builder = self.winit_builder.with_transparent(transparent);
        self
    }

    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.winit_builder = self.winit_builder.with_decorations(decorations);
        self
    }

    pub fn with_window_icon(mut self, window_icon: Option<winit::window::Icon>) -> Self {
        self.winit_builder = self.winit_builder.with_window_icon(window_icon);
        self
    }

    //- Building -------------------------------------------------------------

    pub fn build(
        self,
        event_loop: &winit::event_loop::EventLoop<()>,
    ) -> Result<IridWindow, OsError> {
        Ok(IridWindow {
            winit_window: self.winit_builder.build(event_loop)?,
        })
    }
}

//= IRID WINDOW ==============================================================

#[derive(Debug)]
pub struct IridWindow {
    winit_window: winit::window::Window,
}

impl Default for IridWindow {
    /// It may panic because of [IridWindowBuilder::build] causes.
    fn default() -> Self {
        IridWindowConfig::new()
            .build(&winit::event_loop::EventLoop::new())
            .unwrap()
    }
}

impl IridWindow {
    #[inline]
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Result<IridWindow, OsError> {
        IridWindowConfig::new().build(event_loop)
    }

    #[inline]
    pub fn id(&self) -> WindowId {
        self.winit_window.id()
    }

    #[inline]
    pub fn scale_factor(&self) -> f64 {
        self.winit_window.scale_factor()
    }

    #[inline]
    pub fn request_redraw(&self) {
        self.winit_window.request_redraw()
    }

    //- Position and Size Functions ------------------------------------------

    #[inline]
    pub fn inner_position(&self) -> Result<PhysicalPosition<i32>, NotSupportedError> {
        self.winit_window.inner_position()
    }

    #[inline]
    pub fn outer_position(&self) -> Result<PhysicalPosition<i32>, NotSupportedError> {
        self.winit_window.outer_position()
    }

    #[inline]
    pub fn set_outer_position<P: Into<winit::dpi::Position>>(&self, position: P) {
        self.winit_window.set_outer_position(position)
    }

    #[inline]
    pub fn inner_size(&self) -> PhysicalSize<u32> {
        self.winit_window.inner_size()
    }

    #[inline]
    pub fn set_inner_size<S: Into<winit::dpi::Size>>(&self, size: S) {
        self.winit_window.set_inner_size(size)
    }

    #[inline]
    pub fn outer_size(&self) -> PhysicalSize<u32> {
        self.winit_window.outer_size()
    }

    #[inline]
    pub fn set_min_inner_size<S: Into<winit::dpi::Size>>(&self, min_size: Option<S>) {
        self.winit_window.set_min_inner_size(min_size)
    }

    #[inline]
    pub fn set_max_inner_size<S: Into<winit::dpi::Size>>(&self, max_size: Option<S>) {
        self.winit_window.set_max_inner_size(max_size)
    }

    //- Misc. Attribute Functions --------------------------------------------

    #[inline]
    pub fn set_title(&self, title: &str) {
        self.winit_window.set_title(title)
    }

    #[inline]
    pub fn set_visible(&self, visible: bool) {
        self.winit_window.set_visible(visible)
    }

    #[inline]
    pub fn set_resizable(&self, resizable: bool) {
        self.winit_window.set_resizable(resizable)
    }

    #[inline]
    pub fn set_minimized(&self, minimized: bool) {
        self.winit_window.set_minimized(minimized)
    }

    #[inline]
    pub fn set_maximized(&self, maximized: bool) {
        self.winit_window.set_maximized(maximized)
    }

    #[inline]
    pub fn is_maximized(&self) -> bool {
        self.winit_window.is_maximized()
    }

    #[inline]
    pub fn set_fullscreen(&self, fullscreen: Option<winit::window::Fullscreen>) {
        self.winit_window.set_fullscreen(fullscreen)
    }

    #[inline]
    pub fn fullscreen(&self) -> Option<winit::window::Fullscreen> {
        self.winit_window.fullscreen()
    }

    #[inline]
    pub fn set_decorations(&self, decorations: bool) {
        self.winit_window.set_decorations(decorations)
    }

    #[inline]
    pub fn set_window_icon(&self, window_icon: Option<winit::window::Icon>) {
        self.winit_window.set_window_icon(window_icon)
    }

    #[inline]
    pub fn set_ime_position<P: Into<winit::dpi::Position>>(&self, position: P) {
        self.winit_window.set_ime_position(position)
    }

    #[inline]
    pub fn focus_window(&self) {
        self.winit_window.focus_window()
    }

    #[inline]
    pub fn request_user_attention(&self, request_type: Option<UserAttentionType>) {
        self.winit_window.request_user_attention(request_type)
    }

    //- Cursor Functions -----------------------------------------------------

    #[inline]
    pub fn set_cursor_icon(&self, cursor: CursorIcon) {
        self.winit_window.set_cursor_icon(cursor)
    }

    #[inline]
    pub fn set_cursor_position<P: Into<winit::dpi::Position>>(
        &self,
        position: P,
    ) -> Result<(), ExternalError> {
        self.winit_window.set_cursor_position(position)
    }

    #[inline]
    pub fn set_cursor_grab(&self, grab: CursorGrabMode) -> Result<(), ExternalError> {
        self.winit_window.set_cursor_grab(grab)
    }

    #[inline]
    pub fn set_cursor_visible(&self, visible: bool) {
        self.winit_window.set_cursor_visible(visible)
    }

    //- Monitor Info Functions -----------------------------------------------

    #[inline]
    pub fn drag_window(&self) -> Result<(), ExternalError> {
        self.winit_window.drag_window()
    }

    #[inline]
    pub fn current_monitor(&self) -> Option<MonitorHandle> {
        self.winit_window.current_monitor()
    }

    #[inline]
    pub fn available_monitors(&self) -> impl Iterator<Item = MonitorHandle> {
        self.winit_window.available_monitors()
    }

    #[inline]
    pub fn primary_monitor(&self) -> Option<MonitorHandle> {
        self.winit_window.primary_monitor()
    }

    //- Wrapper Functions ----------------------------------------------------

    #[inline]
    pub fn expose_inner_window(&self) -> &winit::window::Window {
        &self.winit_window
    }
}
