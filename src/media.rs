use crate::headers::HttpContentType;

/// Contains an [`HttpContentType`] with its corresponding file extension.
pub struct ContentExtension {
    pub content_type: HttpContentType,
    pub file_extension: &'static str,
}

/// Generates constants for inputted content types.
macro_rules! content_types {
    (
        $(
            $(#[$docs:meta])*
            $content_type_first:literal/$content_type_second:literal $content_extension:literal $konst:ident;
        )+
    ) => {
        $(
            $(#[$docs])*
            pub const $konst: ContentExtension = ContentExtension { content_type: HttpContentType { first:  $content_type_first, second: $content_type_second }, file_extension: $content_extension };
        )+

        /// Converts a file extension into a [`ContentExtension`] if a constant for that
        /// extension exists.
        pub fn ext_to_type(ext: &str) -> Option<ContentExtension> {
            match ext {
                $(
                $content_extension => Some($konst),
                )+
                _ => None,
            }
        }
    };
}

content_types!(
    "text" / "plain" "txt" TXT;
    "text" / "html" "html" HTML;
    "text" / "css" "css" CSS;
    "image" / "png" "png" PNG;
    "image" / "svg+xml" "svg" SVG;
);
