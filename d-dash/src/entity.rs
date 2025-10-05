#[derive(Debug)]
pub enum Type {
    File,
    Directory,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = match self {
            Type::File => "File",
            Type::Directory => "Dir",
        };
        write!(f, "{}", type_str)
    }
}

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub entity_type: Type,
    pub size: u64,
    pub extension: Option<String>,
}

impl Entity {
    pub fn new(name: String, entity_type: Type, size: u64, extension: Option<String>) -> Self {
        Self {
            name,
            entity_type,
            size,
            extension,
        }
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entity_type = match self.entity_type {
            Type::File => "File",
            Type::Directory => "Dir",
        };
        let extension = self.extension.as_ref().map_or("", |ext| ext.as_str());
        write!(
            f,
            "{:<25}\t{:<5}\t{:<20}\t{:>10} bytes",
            self.name,
            entity_type,
            match self.entity_type {
                Type::File => get_extension_means(extension).unwrap_or("Unknown"),
                Type::Directory => "",
            },
            self.size
        )
    }
}

pub fn get_extension_means(extension: &str) -> Option<&'static str> {
    match extension {
        "txt" => Some("Text File"),
        "rs" => Some("Rust Source File"),
        "py" => Some("Python Source File"),
        "js" => Some("JavaScript File"),
        "html" => Some("HTML File"),
        "css" => Some("CSS File"),
        "json" => Some("JSON File"),
        "xml" => Some("XML File"),
        "jpg" | "jpeg" => Some("JPEG Image"),
        "png" => Some("PNG Image"),
        "gif" => Some("GIF Image"),
        "bmp" => Some("Bitmap Image"),
        "mp3" => Some("MP3 Audio File"),
        "wav" => Some("WAV Audio File"),
        "mp4" => Some("MP4 Video File"),
        "mkv" => Some("MKV Video File"),
        "pdf" => Some("PDF Document"),
        "doc" | "docx" => Some("Word Document"),
        "xls" | "xlsx" => Some("Excel Spreadsheet"),
        "ppt" | "pptx" => Some("PowerPoint Presentation"),
        "md" => Some("Markdown File"),
        "zip" => Some("ZIP Archive"),
        "toml" => Some("TOML File"),
        "yaml" | "yml" => Some("YAML File"),
        "csv" => Some("CSV File"),
        "exe" => Some("Windows Executable"),
        "lock" => Some("Lock File"),
        "bin" => Some("Binary File"),
        _ => None,
    }
}
