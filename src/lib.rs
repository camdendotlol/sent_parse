/// TextSlide represents a standard, non-image slide.
/// It contains a ``text`` field with the text of the slide.
pub struct TextSlide {
    text: String
}

impl TextSlide {
    fn new(text: &str) -> TextSlide {
        TextSlide { text: String::from(text) }
    }
}


/// ImageSlide contains an ``image_path`` property that represents the location of the image on disk.
pub struct ImageSlide {
    image_path: String
}

impl ImageSlide {
    fn new(text: & str) -> ImageSlide {
        let path = &text[1..];

        ImageSlide { image_path: String::from(path) }
    }
}

/// There are three types of slides. ``TextSlide``, ``ImageSlide``, and ``EmptySlide``.
/// Empty slides are meant to be blank, with no text or images.
pub enum Slide {
    TextSlide(TextSlide),
    ImageSlide(ImageSlide),
    EmptySlide
}

/// This function accepts a string and returns a ``Vec`` containing ``Slide`` instances.
/// ``generate_slides`` is very un-opinionated, meaning you can use it in a lot more ways than
/// the classic Suckless sent program. Currently, programs using this crate must implement
/// display functionality for ``TextSlide``, ``ImageSlide``, and ``EmptySlide``.
pub fn generate_slides(full_text: String) -> Vec<Slide> {
    // Lines starting with # are comments, so the parser ignores them
    let text_minus_comments: String = full_text
        .lines()
        .filter(|&line| line.len() == 0 || line.chars().nth(0).unwrap() != '#')
        .map(|line | format!("{}\n", line))
        .collect();

    // Get the text for each slide, with newlines stripped
    let slides_text: Vec<&str> = text_minus_comments
        .split("\n\n")
        .map(|s| s.trim())
        .collect();

    // Put the slides together in the slide struct
    let mut slides: Vec<Slide> = Vec::new();
    for s in slides_text {
        match s.chars().nth(0).unwrap() {
            // Starting with @ denotes an image slide with a path (e.g. @nyan.png)
            '@' => slides.push(Slide::ImageSlide(ImageSlide::new(s))),
            // Lines beginning with / are empty slides, used e.g. for pacing or section divisions
            '/' => slides.push(Slide::EmptySlide),
            // Everything else is just regular text
            _ => slides.push(Slide::TextSlide(TextSlide::new(s)))
        }
    };

    slides
}

#[cfg(test)]
mod tests {
    use crate::generate_slides;

    #[test]
    fn buncha_slides() {
        let presentation = String::from("
* test
* test
* test

This is another slide.

Another one!

@image.png
");
        let slides = generate_slides(presentation);
        assert_eq!(slides.len(), 4)
    }

    #[test]
    fn comments_work() {
        let presentation = String::from("
* test
* test
* test

# This is a comment.

Another one!

@image.png
");
    let slides = generate_slides(presentation);
    assert_eq!(slides.len(), 3)
    }
}