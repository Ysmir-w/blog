use serde::{Deserialize, Serialize};

/// 分页对象
#[derive(Serialize, Deserialize)]
pub struct Paginate<T> {
    pub page: u32,
    pub page_size: u8,
    pub total_records: i64,
    pub total_pages: i64,
    pub data: T,
}

impl<T> Paginate<T> {
    /// 创建一个新的分页对象
    pub fn new(page: u32, page_size: u8, total_records: i64, data: T) -> Self {
        let total_pages = f64::ceil(total_records as f64 / page_size as f64) as i64;
        Self {
            page,
            page_size,
            total_records,
            total_pages,
            data,
        }
    }

    pub fn has_prev(&self) -> bool {
        self.page > 0
    }

    pub fn last_page(&self) -> i64 {
        self.total_pages - 1
    }

    pub fn has_next(&self) -> bool {
        (self.page as i64) < self.last_page()
    }

    pub fn is_active(&self, page: &i64) -> bool {
        (self.page as i64) == *page
    }
}
