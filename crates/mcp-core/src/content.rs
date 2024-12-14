use super::role::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Vec<Role>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextContent {
    pub r#type: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageContent {
    pub r#type: String,
    pub data: String,
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
/// Content passed to or from an LLM
pub enum Content {
    Text(TextContent),
    Image(ImageContent),
}

impl Content {
    pub fn text<S: Into<String>>(text: S) -> Self {
        Content::Text(TextContent {
            r#type: "text".to_string(),
            text: text.into(),
            annotations: None,
        })
    }

    pub fn image<S: Into<String>, T: Into<String>>(data: S, mime_type: T) -> Self {
        Content::Image(ImageContent {
            r#type: "image".to_string(),
            data: data.into(),
            mime_type: mime_type.into(),
            annotations: None,
        })
    }

    /// Get the text content if this is a TextContent variant
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Content::Text(text) => Some(&text.text),
            _ => None,
        }
    }

    /// Get the image content if this is an ImageContent variant
    pub fn as_image(&self) -> Option<(&str, &str)> {
        match self {
            Content::Image(image) => Some((&image.data, &image.mime_type)),
            _ => None,
        }
    }

    /// Set the audience for the content
    pub fn with_audience(mut self, audience: Vec<Role>) -> Self {
        let annotations = match &mut self {
            Content::Text(text) => &mut text.annotations,
            Content::Image(image) => &mut image.annotations,
        };
        *annotations = Some(match annotations.take() {
            Some(mut a) => {
                a.audience = Some(audience);
                a
            }
            None => Annotations {
                audience: Some(audience),
                priority: None,
            },
        });
        self
    }

    /// Set the priority for the content
    /// # Panics
    /// Panics if priority is not between 0.0 and 1.0 inclusive
    pub fn with_priority(mut self, priority: f32) -> Self {
        if !(0.0..=1.0).contains(&priority) {
            panic!("Priority must be between 0.0 and 1.0");
        }
        let annotations = match &mut self {
            Content::Text(text) => &mut text.annotations,
            Content::Image(image) => &mut image.annotations,
        };
        *annotations = Some(match annotations.take() {
            Some(mut a) => {
                a.priority = Some(priority);
                a
            }
            None => Annotations {
                audience: None,
                priority: Some(priority),
            },
        });
        self
    }

    /// Get the audience if set
    pub fn audience(&self) -> Option<&Vec<Role>> {
        match self {
            Content::Text(text) => text.annotations.as_ref().and_then(|a| a.audience.as_ref()),
            Content::Image(image) => image.annotations.as_ref().and_then(|a| a.audience.as_ref()),
        }
    }

    /// Get the priority if set
    pub fn priority(&self) -> Option<f32> {
        match self {
            Content::Text(text) => text.annotations.as_ref().and_then(|a| a.priority),
            Content::Image(image) => image.annotations.as_ref().and_then(|a| a.priority),
        }
    }

    pub fn unannotated(&self) -> Self {
        match self {
            Content::Text(text) => Content::text(text.text.clone()),
            Content::Image(image) => Content::image(image.data.clone(), image.mime_type.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_text() {
        let content = Content::text("hello");
        assert_eq!(content.as_text(), Some("hello"));
        assert_eq!(content.as_image(), None);
    }

    #[test]
    fn test_content_image() {
        let content = Content::image("data", "image/png");
        assert_eq!(content.as_text(), None);
        assert_eq!(content.as_image(), Some(("data", "image/png")));
    }

    #[test]
    fn test_content_annotations() {
        let content = Content::text("hello")
            .with_audience(vec![Role::User])
            .with_priority(0.5);
        assert_eq!(content.audience(), Some(&vec![Role::User]));
        assert_eq!(content.priority(), Some(0.5));
    }

    #[test]
    #[should_panic(expected = "Priority must be between 0.0 and 1.0")]
    fn test_invalid_priority() {
        Content::text("hello").with_priority(1.5);
    }

    #[test]
    fn test_unannotated() {
        let content = Content::text("hello")
            .with_audience(vec![Role::User])
            .with_priority(0.5);
        let unannotated = content.unannotated();
        assert_eq!(unannotated.audience(), None);
        assert_eq!(unannotated.priority(), None);
    }
}