use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct AppError {
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl AppError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    fn with_source(message: impl Into<String>, source: impl Error + Send + Sync + 'static) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|error| error as &(dyn Error + 'static))
    }
}

fn read_config() -> Result<String, AppError> {
    std::fs::read_to_string("missing-config.toml")
        .map_err(|error| AppError::with_source("读取配置文件失败", error))
}

fn start_app() -> Result<(), AppError> {
    let _config = read_config().map_err(|error| AppError::with_source("应用启动失败", error))?;
    Ok(())
}

fn print_error_chain(error: &dyn Error) {
    println!("错误：{error}");

    let mut source = error.source();
    while let Some(error_source) = source {
        println!("原因：{error_source}");
        source = error_source.source();
    }
}

fn main() {
    let standalone_error = AppError::new("这是一个没有 source 的独立错误");
    print_error_chain(&standalone_error);

    println!("---");

    if let Err(error) = start_app() {
        print_error_chain(&error);
    }
}
