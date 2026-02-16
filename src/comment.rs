use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::sync::LazyLock;

static COMMENTS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let res: Result<HashMap<String, String>, Box<dyn Error>> = (|| {
        let Ok(path) = env::var("CYME_COMMENTS") else {
            return Ok(Default::default());
        };
        Ok(serde_json::from_reader(File::open(path)?)?)
    })();
    res.unwrap_or_else(|err| {
        log::error!("failed to load comments: {err:?}");
        Default::default()
    })
});

pub fn get_comment(serial: &str) -> Option<&'static str> {
    COMMENTS.get(serial).map(|s| &**s)
}
