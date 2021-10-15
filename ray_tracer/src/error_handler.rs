// Error handling using custum error type

pub type RayTracerResult<T> = Result<T, RayTracerError>;

type IOError = std::io::Error;

impl From<std::io::Error> for RayTracerError {
    fn from(err: std::io::Error) -> Self {
        return RayTracerError::IOError(err);
    }
}

#[derive(Debug)]
pub enum RayTracerError {
    OutsideImageBuffer,
    IOError(IOError),
}