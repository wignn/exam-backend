use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub skip: Option<i64>,
    pub page: Option<i64>,
}

impl Pagination {
    pub fn limit_or_default(&self, default: i64) -> i64 {
        self.limit.unwrap_or(default)
    }
    pub fn skip_or_default(&self) -> i64 {
        self.skip.unwrap_or(0)
    }
    pub fn page_or_default(&self) -> i64 {
        self.page.unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_default_values() {
        let pagination = Pagination {
            limit: None,
            skip: None,
            page: None,
        };

        assert_eq!(pagination.limit_or_default(20), 20);
        assert_eq!(pagination.skip_or_default(), 0);
        assert_eq!(pagination.page_or_default(), 1);
    }

    #[test]
    fn test_pagination_with_values() {
        let pagination = Pagination {
            limit: Some(10),
            page: None, // No page specified
            skip: Some(5),
        };

        assert_eq!(pagination.limit_or_default(20), 10);
        assert_eq!(pagination.skip_or_default(), 5);
    }

    #[test]
    fn test_pagination_limit_only() {
        let pagination = Pagination {
            limit: Some(15),
            page: None,
            skip: None,
        };

        assert_eq!(pagination.limit_or_default(20), 15);
        assert_eq!(pagination.skip_or_default(), 0);
    }

    #[test]
    fn test_pagination_skip_only() {
        let pagination = Pagination {
            limit: None,
            page: None,
            skip: Some(10),
        };

        assert_eq!(pagination.limit_or_default(20), 20);
        assert_eq!(pagination.skip_or_default(), 10);
    }

    #[test]
    fn test_pagination_zero_values() {
        let pagination = Pagination {
            limit: Some(0),
            page: Some(0),
            skip: Some(0),
        };

        assert_eq!(pagination.limit_or_default(20), 0);
        assert_eq!(pagination.skip_or_default(), 0);
    }

    #[test]
    fn test_pagination_large_values() {
        let pagination = Pagination {
            limit: Some(1000),
            skip: Some(50000),
            page: None, // No page specified
        };

        assert_eq!(pagination.limit_or_default(20), 1000);
        assert_eq!(pagination.skip_or_default(), 50000);
    }
}
