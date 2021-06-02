pub struct TextSlide {
    text: String
}

impl TextSlide {
    fn new(text: &str) -> TextSlide {
        TextSlide { text: String::from(text) }
    }
}

pub struct ImageSlide {
    image_path: String
}

impl ImageSlide {
    fn new(text: & str) -> ImageSlide {
        let path = &text[1..];

        ImageSlide { image_path: String::from(path) }
    }
}

pub enum Slide {
    TextSlide(TextSlide),
    ImageSlide(ImageSlide),
    EmptySlide
}

pub fn generate_slides(full_text: String) -> Vec<Slide> {
    // Get the text for each slide, with newlines stripped
    let slides_text: Vec<&str> = full_text.split("\n\n").map(|s| s.trim()).collect();

    // Put the slides together in the slide struct
    let mut slides: Vec<Slide> = Vec::new();
    for s in slides_text {
        match s.chars().nth(0).unwrap() {
            // Starting with @ denotes an image slide with a path (e.g. @nyan.png)
            '@' => slides.push(Slide::ImageSlide(ImageSlide::new(s))),
            // Lines beginning with # are comments, so the parser should do nothing
            '#' => {},
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