pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = if self.list.is_empty() {
            0.0
        } else {
            total as f64 / self.list.len() as f64
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut collection = AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        };

        collection.add(10);
        collection.add(20);
        collection.add(30);

        assert_eq!(collection.list, vec![10, 20, 30]);
        assert_eq!(collection.average(), 20.0);
    }

    #[test]
    fn test_remove() {
        let mut collection = AveragedCollection {
            list: vec![10, 20, 30],
            average: 20.0,
        };

        let removed_value = collection.remove();
        assert_eq!(removed_value, Some(30));
        assert_eq!(collection.list, vec![10, 20]);
        assert_eq!(collection.average(), 15.0);

        let removed_value = collection.remove();
        assert_eq!(removed_value, Some(20));
        assert_eq!(collection.list, vec![10]);
        assert_eq!(collection.average(), 10.0);

        let removed_value = collection.remove();
        assert_eq!(removed_value, Some(10));
        assert_eq!(collection.list, vec![]);
        assert_eq!(collection.average(), 0.0);

        let removed_value = collection.remove();
        assert_eq!(removed_value, None);
    }

    #[test]
    fn test_average_empty() {
        let collection = AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        };

        assert_eq!(collection.average(), 0.0);
    }

    #[test]
    fn test_update_average() {
        let mut collection = AveragedCollection {
            list: vec![10, 20, 30],
            average: 0.0,
        };

        collection.update_average();
        assert_eq!(collection.average(), 20.0);
    }
}
