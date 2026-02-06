use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct Paginator<T> {
    pub current_page: usize,
    pub total_pages: usize,
    pub items: Vec<T>,
}

pub fn paginate<T: Clone>(items: &[T], per_page: usize) -> Vec<Paginator<T>> {
    if items.is_empty() {
        return vec![];
    }
    
    let total_pages = (items.len() as f64 / per_page as f64).ceil() as usize;
    let mut pages = Vec::new();

    for i in 0..total_pages {
        let start = i * per_page;
        let end = std::cmp::min(start + per_page, items.len());
        pages.push(Paginator {
            current_page: i + 1,
            total_pages,
            items: items[start..end].to_vec(),
        });
    }

    pages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paginate() {
        let items = vec![1, 2, 3, 4, 5];
        let pages = paginate(&items, 2);

        assert_eq!(pages.len(), 3);
        assert_eq!(pages[0].items, vec![1, 2]);
        assert_eq!(pages[0].current_page, 1);
        assert_eq!(pages[0].total_pages, 3);
        
        assert_eq!(pages[2].items, vec![5]);
        assert_eq!(pages[2].current_page, 3);
    }
}