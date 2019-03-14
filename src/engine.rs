//! Core typesetting engine.

use std::error;
use std::fmt;
use crate::syntax::{SyntaxTree, Node};
use crate::doc::{Document, Style, Size, Page, Text, TextCommand};
use crate::font::Font;


/// The core typesetting engine, transforming an abstract syntax tree into a document.
#[derive(Debug, Clone)]
pub struct Engine<'s> {
    // Immutable
    tree: &'s SyntaxTree<'s>,
    style: Style,

    // Mutable
    fonts: Vec<Font>,
    active_font: usize,
    text_commands: Vec<TextCommand>,
    current_line: String,
    current_width: Size,
}

impl<'s> Engine<'s> {
    /// Create a new generator from a syntax tree.
    pub fn new(tree: &'s SyntaxTree<'s>, style: Style) -> Engine<'s> {
        Engine {
            style,
            tree,
            fonts: Vec::new(),
            active_font: 0,
            text_commands: Vec::new(),
            current_line: String::new(),
            current_width: Size::zero(),
        }
    }

    /// Generate the abstract document.
    pub fn typeset(mut self) -> TypeResult<Document> {
        // Load font defined by style
        let font_family = self.style.font_families.first().unwrap();
        let program = std::fs::read(format!("../fonts/{}-Regular.ttf", font_family)).unwrap();
        let font = Font::new(program).unwrap();

        self.fonts.push(font);
        self.active_font = 0;

        // Move cursor to top-left position
        self.text_commands.push(TextCommand::Move(
            self.style.margin_left,
            self.style.height - self.style.margin_top
        ));

        // Set the current font
        self.text_commands.push(TextCommand::SetFont(0, self.style.font_size));

        // Iterate through the documents nodes.
        for node in &self.tree.nodes {
            match node {
                Node::Word(word) => self.write_word(word),

                Node::Space => self.write_space(),
                Node::Newline => (),

                Node::ToggleItalics | Node::ToggleBold | Node::ToggleMath => unimplemented!(),
                Node::Func(_) => unimplemented!(),
            }
        }

        // Create a page from the contents.
        let page = Page {
            width: self.style.width,
            height: self.style.height,
            text: vec![Text {
                commands: self.text_commands,
            }],
        };

        Ok(Document {
            pages: vec![page],
            fonts: self.fonts,
        })
    }

    fn write_word(&mut self, word: &str) {
        let font = &self.fonts[self.active_font];

        let width = self.width(word);
        if self.would_overflow(width) {
            let vertical_move = - self.style.font_size
                * self.style.line_spacing
                * font.metrics.ascender;
            self.text_commands.push(TextCommand::Move(Size::zero(), vertical_move));

            self.current_line.clear();
            self.current_width = Size::zero();
        }

        self.text_commands.push(TextCommand::Text(word.to_owned()));
        self.current_line.push_str(word);
        self.current_width += width;
    }

    fn write_space(&mut self) {
        let space_width = self.width(" ");

        if !self.would_overflow(space_width) && !self.current_line.is_empty() {
            self.text_commands.push(TextCommand::Text(" ".to_owned()));
            self.current_line.push_str(" ");
            self.current_width += space_width;
        }
    }

    fn width(&self, word: &str) -> Size {
        let font = &self.fonts[self.active_font];
        word.chars()
            .map(|c| font.widths[font.map(c) as usize] * self.style.font_size)
            .sum()
    }

    fn would_overflow(&self, width: Size) -> bool {
        let max_width = self.style.width
            - self.style.margin_left
            - self.style.margin_right;

        self.current_width + width > max_width
    }
}

/// Result type used for parsing.
type TypeResult<T> = std::result::Result<T, TypesetError>;

/// The error type for typesetting.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypesetError {
    message: String,
}

impl error::Error for TypesetError {}

impl fmt::Display for TypesetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message)
    }
}