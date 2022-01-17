//= USES ===========================================================================================

use pollster::FutureExt;
use thiserror::Error;

//= ERRORS =========================================================================================

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Error)]
pub(crate) enum AdapterError {
    #[error("An adapter compatible with the given surface could not be obtained")]
    NotObtained,
}

//= ADAPTER WRAPPER ================================================================================

/// Handle to a physical graphics and/or compute device.
///
/// Adapters can be used to open a connection to the corresponding [`Device`]
/// on the host system by using [`Adapter::request_device`].
#[derive(Debug)]
pub struct Adapter {
    wgpu_adapter: wgpu::Adapter,
}

impl Adapter {
    //- Constructors -------------------------------------------------------------------------------

    /// Retrieves an Adapter which matches the given surface.
    /// Some options are "soft", so treated as non-mandatory. Others are "hard".
    /// If no adapters are found that suffice all the "hard" options, Err is returned.
    pub(crate) fn new(
        wgpu_instance: &wgpu::Instance,
        adapter_options: wgpu::RequestAdapterOptions<'_>,
    ) -> Result<Self, AdapterError> {
        let wgpu_adapter =
            async { wgpu_instance.request_adapter(&adapter_options).await }.block_on();

        if let Some(wgpu_adapter) = wgpu_adapter {
            Ok(Self { wgpu_adapter })
        } else {
            Err(AdapterError::NotObtained)
        }
    }

    //- Wrapped Methods ----------------------------------------------------------------------------

    /// Requests a connection to a physical device, creating a logical device.
    ///
    /// Returns the Device together with a Queue that executes command buffers.
    ///
    /// # Arguments
    ///
    /// - `desc` - Description of the features and limits requested from the given device.
    /// - `trace_path` - Can be used for API call tracing, if that feature is
    ///   enabled in `wgpu-core`.
    ///
    /// # Panics
    ///
    /// - Features specified by `desc` are not supported by this adapter.
    /// - Unsafe features were requested but not enabled when requesting the adapter.
    /// - Limits requested exceed the values provided by the adapter.
    /// - Adapter does not support all features wgpu requires to safely operate.
    pub fn request_device(
        &self,
        desc: &wgpu::DeviceDescriptor<'_>,
        trace_path: Option<&std::path::Path>,
    ) -> impl std::future::Future<
        Output = Result<(wgpu::Device, wgpu::Queue), wgpu::RequestDeviceError>,
    > + Send {
        self.wgpu_adapter.request_device(desc, trace_path)
    }

    /// Get info about the adapter itself.
    pub fn get_info(&self) -> wgpu::AdapterInfo {
        self.wgpu_adapter.get_info()
    }

    //- Crate-Public Methods -----------------------------------------------------------------------

    // This method MUST remains public at the crate level.
    pub(crate) fn expose_wrapped_adapter(&self) -> &wgpu::Adapter {
        &self.wgpu_adapter
    }
}
