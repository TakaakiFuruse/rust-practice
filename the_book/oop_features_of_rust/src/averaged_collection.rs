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
            Some(v) => {
                self.update_average();
                Some(v)
            }
            None => {
                self.update_average();
                None
            }
        }
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();

        if self.list.len() > 0 {
            self.average = total as f64 / self.list.len() as f64
        } else {
            self.average = 0.0
        }
    }
}

#[cfg(test)]
mod tests_averaged_collection {
    use super::*;

    #[test]
    fn test_fn_add() {
        let mut avg = AveragedCollection {
            list: vec![1, 1],
            average: 0.0,
        };
        avg.add(1);
        assert_eq!(avg.list, vec![1, 1, 1]);
        assert_eq!(avg.average, 1.0)
    }
    #[test]
    fn test_fn_remove() {
        let mut avg = AveragedCollection {
            list: vec![1, 2, 3],
            average: 2.0,
        };
        avg.remove();
        assert_eq!(avg.list, vec![1, 2]);
        assert_eq!(avg.average, 1.5);
    }
    #[test]
    fn test_fn_remove_in_case_of_no_elm() {
        let mut avg = AveragedCollection {
            list: vec![],
            average: 2.0,
        };
        avg.remove();
        assert_eq!(avg.list, vec![]);
        assert_eq!(avg.average, 0.0);
    }

}
