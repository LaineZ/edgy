// Copyright 2019 the SimpleCSS Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use alloc::{vec, vec::Vec};
use core::fmt;

use log::warn;

use crate::stream::Stream;
use crate::Error;

/// An attribute selector operator.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AttributeOperator<'a> {
    /// `[attr]`
    Exists,
    /// `[attr=value]`
    Matches(&'a str),
    /// `[attr~=value]`
    Contains(&'a str),
    /// `[attr|=value]`
    StartsWith(&'a str),
}

pub enum SelectorKindInfo<'a> {
    Tag(&'a str),
    Id(&'a str),
    Class(&'a str),
}

impl AttributeOperator<'_> {
    /// Checks that value is matching the operator.
    pub fn matches(&self, value: &str) -> bool {
        match *self {
            AttributeOperator::Exists => true,
            AttributeOperator::Matches(v) => value == v,
            AttributeOperator::Contains(v) => value.split(' ').any(|s| s == v),
            AttributeOperator::StartsWith(v) => {
                // exactly `v` or beginning with `v` immediately followed by `-`
                if value == v {
                    true
                } else if value.starts_with(v) {
                    value.get(v.len()..v.len() + 1) == Some("-")
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum SimpleSelectorType<'a> {
    Type(&'a str),
    Universal,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum SubSelector<'a> {
    Attribute(&'a str, AttributeOperator<'a>),
    PseudoClass(&'a str),
}

#[derive(Clone, Debug)]
struct SimpleSelector<'a> {
    kind: SimpleSelectorType<'a>,
    subselectors: Vec<SubSelector<'a>>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Combinator {
    None,
    Descendant,
    Child,
    AdjacentSibling,
}

#[derive(Clone, Debug)]
struct Component<'a> {
    /// A combinator that precede the selector.
    combinator: Combinator,
    selector: SimpleSelector<'a>,
}

/// A selector.
#[derive(Clone, Debug)]
pub struct Selector<'a> {
    components: Vec<Component<'a>>,
}

impl<'a> Selector<'a> {
    /// Parses a selector from a string.
    ///
    /// Will log any errors as a warnings.
    ///
    /// Parsing will be stopped at EOF, `,` or `{`.
    pub fn parse(text: &'a str) -> Option<Self> {
        parse(text).0
    }

    pub fn kind(&self) -> Result<SelectorKindInfo<'a>, &'a str> {
         // TODO: More convient error types
        let tag = self.type_name();
        let classes = self.class_list();

        let mut count = 0;
        if tag.is_some() {
            count += 1;
        }
        if !classes.is_empty() {
            count += 1;
        }

        if count > 1 {
            return Err("Multiple rule-selectors are not allowed in this version of edgy");
        }

        if let Some(t) = tag {
            Ok(SelectorKindInfo::Tag(t))
        } else if let Some(c) = classes.get(0) {
            Ok(SelectorKindInfo::Class(c))
        } else {
            Err("Selector must have at least a tag, id, or class".into())
        }
    }

    /// Returns all class names from the selector.
    pub fn class_list(&self) -> Vec<&'a str> {
        let mut classes = Vec::new();
        for component in &self.components {
            for sub in &component.selector.subselectors {
                if let SubSelector::Attribute("class", AttributeOperator::Contains(value)) = sub {
                    classes.push(*value);
                }
            }
        }
        classes
    }

    /// Gets a full selector name
    pub fn type_name(&self) -> Option<&'a str> {
        for component in &self.components {
            if let SimpleSelectorType::Type(ident) = component.selector.kind {
                return Some(ident);
            }
        }
        None
    }

    /// Gets a pseudo classes
    pub fn pseudo_classes(&self) -> Vec<&'a str> {
        let mut pseudos = Vec::new();
        for component in &self.components {
            for sub in &component.selector.subselectors {
                if let SubSelector::PseudoClass(name) = sub {
                    pseudos.push(*name);
                }
            }
        }
        pseudos
    }

    /// Compute the selector's specificity.
    ///
    /// Cf. <https://www.w3.org/TR/selectors/#specificity>.
    pub fn specificity(&self) -> [u8; 3] {
        let mut spec = [0u8; 3];

        for selector in self.components.iter().map(|c| &c.selector) {
            if matches!(selector.kind, SimpleSelectorType::Type(_)) {
                spec[2] = spec[2].saturating_add(1);
            }

            for sub in &selector.subselectors {
                match sub {
                    SubSelector::Attribute("id", _) => spec[0] = spec[0].saturating_add(1),
                    _ => spec[1] = spec[1].saturating_add(1),
                }
            }
        }

        spec
    }
}

pub(crate) fn parse(text: &str) -> (Option<Selector<'_>>, usize) {
    let mut components: Vec<Component<'_>> = Vec::new();
    let mut combinator = Combinator::None;

    let mut tokenizer = SelectorTokenizer::from(text);
    for token in &mut tokenizer {
        let mut add_sub = |sub| {
            if combinator == Combinator::None && !components.is_empty() {
                if let Some(ref mut component) = components.last_mut() {
                    component.selector.subselectors.push(sub);
                }
            } else {
                components.push(Component {
                    selector: SimpleSelector {
                        kind: SimpleSelectorType::Universal,
                        subselectors: vec![sub],
                    },
                    combinator,
                });

                combinator = Combinator::None;
            }
        };

        let token = match token {
            Ok(t) => t,
            Err(e) => {
                warn!("Selector parsing failed cause {}.", e);
                return (None, tokenizer.stream.pos());
            }
        };

        match token {
            SelectorToken::UniversalSelector => {
                components.push(Component {
                    selector: SimpleSelector {
                        kind: SimpleSelectorType::Universal,
                        subselectors: Vec::new(),
                    },
                    combinator,
                });

                combinator = Combinator::None;
            }
            SelectorToken::TypeSelector(ident) => {
                components.push(Component {
                    selector: SimpleSelector {
                        kind: SimpleSelectorType::Type(ident),
                        subselectors: Vec::new(),
                    },
                    combinator,
                });

                combinator = Combinator::None;
            }
            SelectorToken::ClassSelector(ident) => {
                add_sub(SubSelector::Attribute(
                    "class",
                    AttributeOperator::Contains(ident),
                ));
            }
            SelectorToken::IdSelector(id) => {
                add_sub(SubSelector::Attribute("id", AttributeOperator::Matches(id)));
            }
            SelectorToken::AttributeSelector(name, op) => {
                add_sub(SubSelector::Attribute(name, op));
            }
            SelectorToken::PseudoClass(ident) => {
                add_sub(SubSelector::PseudoClass(ident));
            }
            SelectorToken::DescendantCombinator => {
                combinator = Combinator::Descendant;
            }
            SelectorToken::ChildCombinator => {
                combinator = Combinator::Child;
            }
            SelectorToken::AdjacentCombinator => {
                combinator = Combinator::AdjacentSibling;
            }
        }
    }

    if components.is_empty() {
        (None, tokenizer.stream.pos())
    } else if components[0].combinator != Combinator::None {
        debug_assert_eq!(
            components[0].combinator,
            Combinator::None,
            "the first component must not have a combinator"
        );

        (None, tokenizer.stream.pos())
    } else {
        (Some(Selector { components }), tokenizer.stream.pos())
    }
}

impl fmt::Display for Selector<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for component in &self.components {
            match component.combinator {
                Combinator::Descendant => write!(f, " ")?,
                Combinator::Child => write!(f, " > ")?,
                Combinator::AdjacentSibling => write!(f, " + ")?,
                Combinator::None => {}
            }

            match component.selector.kind {
                SimpleSelectorType::Universal => write!(f, "*")?,
                SimpleSelectorType::Type(ident) => write!(f, "{ident}")?,
            };

            for sel in &component.selector.subselectors {
                match sel {
                    SubSelector::Attribute(name, operator) => {
                        match operator {
                            AttributeOperator::Exists => {
                                write!(f, "[{name}]")?;
                            }
                            AttributeOperator::Matches(value) => {
                                write!(f, "[{name}='{value}']")?;
                            }
                            AttributeOperator::Contains(value) => {
                                write!(f, "[{name}~='{value}']")?;
                            }
                            AttributeOperator::StartsWith(value) => {
                                write!(f, "[{name}|='{value}']")?;
                            }
                        };
                    }
                    SubSelector::PseudoClass(class) => write!(f, ":{class}")?,
                }
            }
        }

        Ok(())
    }
}

/// A selector token.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SelectorToken<'a> {
    /// `*`
    UniversalSelector,

    /// `div`
    TypeSelector(&'a str),

    /// `.class`
    ClassSelector(&'a str),

    /// `#id`
    IdSelector(&'a str),

    /// `[color=red]`
    AttributeSelector(&'a str, AttributeOperator<'a>),

    /// `:first-child`
    PseudoClass(&'a str),

    /// `a b`
    DescendantCombinator,

    /// `a > b`
    ChildCombinator,

    /// `a + b`
    AdjacentCombinator,
}

/// A selector tokenizer.
///
/// # Example
///
/// ```
/// use simplecss::{SelectorTokenizer, SelectorToken};
///
/// let mut t = SelectorTokenizer::from("div > p:first-child");
/// assert_eq!(t.next().unwrap().unwrap(), SelectorToken::TypeSelector("div"));
/// assert_eq!(t.next().unwrap().unwrap(), SelectorToken::ChildCombinator);
/// assert_eq!(t.next().unwrap().unwrap(), SelectorToken::TypeSelector("p"));
/// assert_eq!(t.next().unwrap().unwrap(), SelectorToken::PseudoClass("first-child"));
/// assert!(t.next().is_none());
/// ```
pub struct SelectorTokenizer<'a> {
    stream: Stream<'a>,
    after_combinator: bool,
    finished: bool,
}

impl<'a> From<&'a str> for SelectorTokenizer<'a> {
    fn from(text: &'a str) -> Self {
        SelectorTokenizer {
            stream: Stream::from(text),
            after_combinator: true,
            finished: false,
        }
    }
}

impl<'a> Iterator for SelectorTokenizer<'a> {
    type Item = Result<SelectorToken<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished || self.stream.at_end() {
            if self.after_combinator {
                self.after_combinator = false;
                return Some(Err(Error::SelectorMissing));
            }

            return None;
        }

        macro_rules! try2 {
            ($e:expr) => {
                match $e {
                    Ok(v) => v,
                    Err(e) => {
                        self.finished = true;
                        return Some(Err(e));
                    }
                }
            };
        }

        match self.stream.curr_byte_unchecked() {
            b'*' => {
                if !self.after_combinator {
                    self.finished = true;
                    return Some(Err(Error::UnexpectedSelector));
                }

                self.after_combinator = false;
                self.stream.advance(1);
                Some(Ok(SelectorToken::UniversalSelector))
            }
            b'#' => {
                self.after_combinator = false;
                self.stream.advance(1);
                let ident = try2!(self.stream.consume_ident());
                Some(Ok(SelectorToken::IdSelector(ident)))
            }
            b'.' => {
                self.after_combinator = false;
                self.stream.advance(1);
                let ident = try2!(self.stream.consume_ident());
                Some(Ok(SelectorToken::ClassSelector(ident)))
            }
            b'[' => {
                self.after_combinator = false;
                self.stream.advance(1);
                let ident = try2!(self.stream.consume_ident());

                let op = match try2!(self.stream.curr_byte()) {
                    b']' => AttributeOperator::Exists,
                    b'=' => {
                        self.stream.advance(1);
                        let value = try2!(self.stream.consume_string());
                        AttributeOperator::Matches(value)
                    }
                    b'~' => {
                        self.stream.advance(1);
                        try2!(self.stream.consume_byte(b'='));
                        let value = try2!(self.stream.consume_string());
                        AttributeOperator::Contains(value)
                    }
                    b'|' => {
                        self.stream.advance(1);
                        try2!(self.stream.consume_byte(b'='));
                        let value = try2!(self.stream.consume_string());
                        AttributeOperator::StartsWith(value)
                    }
                    _ => {
                        self.finished = true;
                        return Some(Err(Error::InvalidAttributeSelector));
                    }
                };

                try2!(self.stream.consume_byte(b']'));

                Some(Ok(SelectorToken::AttributeSelector(ident, op)))
            }
            b':' => {
                self.after_combinator = false;
                self.stream.advance(1);
                let ident = try2!(self.stream.consume_ident());
                Some(Ok(SelectorToken::PseudoClass(ident)))
            }
            b'>' => {
                if self.after_combinator {
                    self.after_combinator = false;
                    self.finished = true;
                    return Some(Err(Error::UnexpectedCombinator));
                }

                self.stream.advance(1);
                self.after_combinator = true;
                Some(Ok(SelectorToken::ChildCombinator))
            }
            b'+' => {
                if self.after_combinator {
                    self.after_combinator = false;
                    self.finished = true;
                    return Some(Err(Error::UnexpectedCombinator));
                }

                self.stream.advance(1);
                self.after_combinator = true;
                Some(Ok(SelectorToken::AdjacentCombinator))
            }
            b' ' | b'\t' | b'\n' | b'\r' | b'\x0C' => {
                self.stream.skip_spaces();

                if self.after_combinator {
                    return self.next();
                }

                while self.stream.curr_byte() == Ok(b'/') {
                    try2!(self.stream.skip_comment());
                    self.stream.skip_spaces();
                }

                match self.stream.curr_byte() {
                    Ok(b'>') | Ok(b'+') | Ok(b',') | Ok(b'{') | Err(_) => self.next(),
                    _ => {
                        if self.after_combinator {
                            self.after_combinator = false;
                            self.finished = true;
                            return Some(Err(Error::UnexpectedSelector));
                        }

                        self.after_combinator = true;
                        Some(Ok(SelectorToken::DescendantCombinator))
                    }
                }
            }
            b'/' => {
                if self.stream.next_byte() == Ok(b'*') {
                    try2!(self.stream.skip_comment());
                } else {
                    self.finished = true;
                }

                self.next()
            }
            b',' | b'{' => {
                self.finished = true;
                self.next()
            }
            _ => {
                let ident = try2!(self.stream.consume_ident());

                if !self.after_combinator {
                    self.finished = true;
                    return Some(Err(Error::UnexpectedSelector));
                }

                self.after_combinator = false;
                Some(Ok(SelectorToken::TypeSelector(ident)))
            }
        }
    }
}
