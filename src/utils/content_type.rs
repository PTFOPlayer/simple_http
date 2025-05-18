#[derive(Default, PartialEq, Eq)]

pub enum ContentType {
    #[default]
    TextPlain,
    TextCss,
    TextHtml,
    TextXml,
    ApplicationJson,
    ApplicationJavascript,
}

impl ContentType {
    pub fn from_file_ext_or<'a>(path: &str, default: ContentType) -> Self {
        match path {
            _ if path.ends_with(".css") => ContentType::TextCss,
            _ if path.ends_with(".html") => ContentType::TextHtml,
            _ if path.ends_with(".xml") => ContentType::TextXml,
            _ if path.ends_with(".json") => ContentType::ApplicationJson,
            _ if path.ends_with(".js") => ContentType::ApplicationJavascript,
            _ if path.ends_with(".txt") => ContentType::TextPlain,
            _ => default,
        }
    }

    pub fn from_file_ext<'a>(path: &str) -> Self {
        Self::from_file_ext_or(path, ContentType::TextPlain)
    }

    pub fn to_string(&self) -> &str {
        match self {
            ContentType::TextCss => "text/css",
            ContentType::TextHtml => "text/html",
            ContentType::TextXml => "text/xml",
            ContentType::ApplicationJson => "application/json",
            ContentType::ApplicationJavascript => "application/javascript",
            ContentType::TextPlain => "text/plain",
        }
    }
}
