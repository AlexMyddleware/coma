use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use markup5ever::local_name;
use scraper::Html;
use url::Url;

use crate::topology;

// TODO: here hashset but other vector, should harmonized that
pub fn extract_links(url: &Url, page: &Html) -> HashSet<Url> {
    HashSet::from_iter(page.tree.values().filter_map(|v| match v {
        scraper::Node::Element(element) => {
            let element = element.to_owned();

            // Ensure this is a link
            if !matches!(element.name.local, local_name!("a")) {
                return None;
            }

            // We want the attribute "href"
            for (key, value) in &element.attrs {
                if matches!(key.local, local_name!("href")) {
                    return Url::join(url, value).ok();
                }
            }
            None
        }
        _ => None,
    }))
}

pub fn extract_comments(node: &Arc<Mutex<topology::Node>>, page: &Html) {
    node.lock().unwrap().comments = page
        .tree
        .values()
        .filter_map(|v| match v {
            scraper::Node::Comment(comment) => Some(comment.to_string()),
            _ => None,
        })
        .collect();
}

pub fn extract_texts(node: &Arc<Mutex<topology::Node>>, page: &Html) {
    node.lock().unwrap().texts = page
        .tree
        .values()
        .filter_map(|v| match v {
            scraper::Node::Text(text) => Some(text.to_string()),
            _ => None,
        })
        .collect();
}

pub fn extract_images(node: &Arc<Mutex<topology::Node>>, page: &Html) {
    node.lock().unwrap().images = page
        .tree
        .values()
        .filter_map(|v| match v {
            scraper::Node::Element(element) => {
                let element = element.to_owned();
                if !matches!(element.name.local, local_name!("img")) {
                    return None;
                }
                for (key, value) in &element.attrs {
                    if matches!(key.local, local_name!("src")) {
                        // TODO: add errors
                        // If the url is absolute, the value will replace the base url
                        return Url::join(&node.lock().unwrap().url, value).ok();
                    }
                }
                None
            }
            _ => None,
        })
        .collect();
}
