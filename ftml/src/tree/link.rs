/*
 * tree/link.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2022 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use super::clone::{option_string_to_owned, string_to_owned};
use crate::data::{PageRef, PageCallbacks};
use crate::settings::WikitextSettings;
use crate::url::is_url;
use std::borrow::Cow;
use std::rc::Rc;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum LinkLocation<'a> {
    /// This link points to a particular page on a wiki.
    Page(PageRef<'a>, Option<Cow<'a, str>>),

    /// This link is to a specific URL.
    Url(Cow<'a, str>),
}

impl<'a> LinkLocation<'a> {
    pub fn parse_interwiki(
        link: Cow<'a, str>,
        settings: &WikitextSettings,
        page_callbacks: Rc<dyn PageCallbacks>,
    ) -> Option<(Self, LinkType)> {
        // Handle interwiki (starts with "!", like "!wp:Apple")
        match link.as_ref().strip_prefix('!') {
            // Not interwiki, parse as normal
            None => {
                let interwiki = Self::parse(link, page_callbacks);
                let ltype = interwiki.link_type();
                Some((interwiki, ltype))
            }

            // Try to interpret as interwiki
            Some(link) => settings
                .interwiki
                .build(link)
                .map(|url| (LinkLocation::Url(Cow::Owned(url)), LinkType::Interwiki)),
        }
    }

    pub fn parse(link: Cow<'a, str>, page_callbacks: Rc<dyn PageCallbacks>) -> Self {
        let mut link_str = link.to_string();

        // Check for direct URLs or anchor links
        if is_url(&link_str) || link_str.starts_with('#') || link_str.starts_with('/') {
            return LinkLocation::Url(link);
        }

        let anchor = match link_str.clone().split_once('#') {
            Some((left, right)) => {
                link_str = String::from(left);
                Some(Cow::Owned(right.to_string()))
            }
            None => None
        };

        link_str = page_callbacks.normalize_page_name(Cow::from(link_str)).to_string();

        match PageRef::parse(&link_str) {
            Err(_) => LinkLocation::Url(link),
            Ok(page_ref) => LinkLocation::Page(page_ref.to_owned(), anchor),
        }
    }

    pub fn to_owned(&self) -> LinkLocation<'static> {
        match self {
            LinkLocation::Page(page, _) => LinkLocation::Page(page.to_owned(), None),
            LinkLocation::Url(url) => LinkLocation::Url(string_to_owned(url)),
        }
    }

    pub fn link_type(&self) -> LinkType {
        match self {
            LinkLocation::Page(_, _) => LinkType::Page,
            LinkLocation::Url(_) => LinkType::Direct,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum LinkLabel<'a> {
    /// Custom text link label.
    ///
    /// Can be set to any arbitrary value of the input text's choosing.
    Text(Cow<'a, str>),

    /// URL-mirroring link label.
    ///
    /// If `None`, then the label for this link is the same as the URL.
    /// If `Some(_)`, then the label is a subslice of the URL it targets.
    Url(Option<Cow<'a, str>>),

    /// Article title-based link label.
    ///
    /// The label for this link is whatever the page's title is.
    Page,
}

impl LinkLabel<'_> {
    pub fn to_owned(&self) -> LinkLabel<'static> {
        match self {
            LinkLabel::Text(text) => LinkLabel::Text(string_to_owned(text)),
            LinkLabel::Url(url) => LinkLabel::Url(option_string_to_owned(url)),
            LinkLabel::Page => LinkLabel::Page,
        }
    }
}

#[derive(EnumIter, Serialize, Deserialize, Debug, Hash, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum LinkType {
    /// This URL was specified directly.
    ///
    /// For instance, as a raw URL, or a single-bracket link.
    Direct,

    /// This URL was specified by specifying a particular Wikijump page.
    ///
    /// This variant comes from triple-bracket links.
    Page,

    /// This URL was generated via interwiki substitution.
    Interwiki,

    /// This URL points to an anchor elsewhere on this page.
    Anchor,

    /// This URL points to entries on a page in a table of contents.
    TableOfContents,
}

impl LinkType {
    pub fn name(self) -> &'static str {
        match self {
            LinkType::Direct => "direct",
            LinkType::Page => "page",
            LinkType::Interwiki => "interwiki",
            LinkType::Anchor => "anchor",
            LinkType::TableOfContents => "table-of-contents",
        }
    }
}

impl<'a> TryFrom<&'a str> for LinkType {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<LinkType, &'a str> {
        match value {
            "direct" => Ok(LinkType::Direct),
            "page" => Ok(LinkType::Page),
            "interwiki" => Ok(LinkType::Interwiki),
            "anchor" => Ok(LinkType::Anchor),
            "table-of-contents" => Ok(LinkType::TableOfContents),
            _ => Err(value),
        }
    }
}