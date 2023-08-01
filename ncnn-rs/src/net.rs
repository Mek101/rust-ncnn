use crate::datareader::DataReader;
use crate::Extractor;
use ncnn_bind::*;
use std::ffi::{CString, OsStr};
use std::os::unix::ffi::OsStrExt;

enum LoadMethod {
    None,
    Path { path: CString },
    DataReader { datareader: DataReader },
}

pub struct NetBuilder {
    ptr: Option<ncnn_net_t>,
    param: LoadMethod,
    model: LoadMethod,
}

unsafe impl Send for NetBuilder {}

impl NetBuilder {
    #[cfg(target_family = "unix")]
    fn os_str_to_cstr(src: &OsStr) -> Option<CString> {
        CString::new(src.as_bytes()).ok()
    }

    #[cfg(not(target_family = "unix"))]
    fn os_str_to_cstr(src: &OsStr) -> Option<CString> {
        CString::new(src.to_string_lossy().as_bytes()).ok()
    }

    pub fn new() -> anyhow::Result<NetBuilder> {
        let ptr = unsafe { ncnn_net_create() };
        anyhow::ensure!(!ptr.is_null(), "Could not create Net");

        Ok(Self {
            ptr: Some(ptr),
            param: LoadMethod::None,
            model: LoadMethod::None,
        })
    }

    pub fn set_option(self, opt: &crate::option::Option) -> Self {
        unsafe {
            ncnn_net_set_option(self.ptr.unwrap(), opt.ptr());
        }
        self
    }

    pub fn set_param_path(mut self, param_path: impl AsRef<OsStr>) -> anyhow::Result<Self> {
        let path = Self::os_str_to_cstr(param_path.as_ref())
            .ok_or_else(|| anyhow::anyhow!("Invalid param path"))?;
        self.param = LoadMethod::Path { path };
        Ok(self)
    }

    pub fn set_model_path(mut self, param_path: impl AsRef<OsStr>) -> anyhow::Result<Self> {
        let path = Self::os_str_to_cstr(param_path.as_ref())
            .ok_or_else(|| anyhow::anyhow!("Invalid model path"))?;
        self.model = LoadMethod::Path { path };
        Ok(self)
    }

    pub fn set_param_datareader(mut self, datareader: DataReader) -> Self {
        self.param = LoadMethod::DataReader { datareader };
        self
    }

    pub fn set_model_datareader(mut self, datareader: DataReader) -> Self {
        self.model = LoadMethod::DataReader { datareader };
        self
    }

    pub fn build(mut self) -> anyhow::Result<Net> {
        let net = self.ptr.take().unwrap();

        match &self.param {
            LoadMethod::None => anyhow::bail!("No param loading method specified"),
            LoadMethod::Path { path } => {
                if unsafe { ncnn_net_load_param(net, path.as_ptr()) } != 0 {
                    anyhow::bail!("Error loading params from file");
                }
            }
            LoadMethod::DataReader { datareader } => {
                if unsafe { ncnn_net_load_param_datareader(net, datareader.ptr()) } != 0 {
                    anyhow::bail!("Error loading params from datareader")
                }
            }
        }

        match &self.model {
            LoadMethod::None => anyhow::bail!("No model loading method specified"),
            LoadMethod::Path { path } => {
                if unsafe { ncnn_net_load_model(net, path.as_ptr()) } != 0 {
                    anyhow::bail!("Error loading model from file");
                }
            }
            LoadMethod::DataReader { datareader } => {
                if unsafe { ncnn_net_load_model_datareader(net, datareader.ptr()) } != 0 {
                    anyhow::bail!("Error loading model from datareader")
                }
            }
        }

        Ok(Net { ptr: net })
    }
}

impl Drop for NetBuilder {
    fn drop(&mut self) {
        if let Some(ptr) = self.ptr.take() {
            unsafe { ncnn_net_destroy(ptr) };
        }
    }
}

pub struct Net {
    ptr: ncnn_net_t,
}

unsafe impl Send for Net {}

impl Net {
    pub fn create_extractor(&mut self) -> Extractor<'_> {
        let ptr = unsafe { ncnn_extractor_create(self.ptr) };
        Extractor::from_ptr(ptr)
    }
}

impl Drop for Net {
    fn drop(&mut self) {
        unsafe {
            ncnn_net_destroy(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_send<T: Send>() -> bool {
        true
    }
    fn is_sync<T: Sync>() -> bool {
        true
    }

    #[test]
    fn load_not_exist_model() {
        let _ = NetBuilder::new()
            .unwrap()
            .set_param_path("not_exist.param").unwrap()
            .set_model_path("not_exist.bin").unwrap()
            .build()
            .map(|_| ())
            .expect_err("Expected files to not be found");
    }

    #[test]
    fn check_sync_send() {
        assert!(is_send::<Net>());
        //assert!(is_sync::<Net>());
    }
}
