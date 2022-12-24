use std::{fmt::Display, time::Instant};

pub struct Timer<T1: Display, T2: Display> {
    day: i32,
    start: Instant,
    result1: Option<T1>,
    result2: Option<T2>,
}

impl<T1: Display, T2: Display> Timer<T1, T2> {
    pub fn new(day: i32) -> Self {
        let start = Instant::now();
        Self {
            day,
            start,
            result1: None,
            result2: None,
        }
    }

    pub fn set1(&mut self, result: T1) {
        self.result1 = Some(result);
    }

    pub fn set1_once(&mut self, result: T1) {
        if self.result1.is_none() {
            self.result1 = Some(result);
        }
    }

    pub fn set2(&mut self, result: T2) {
        self.result2 = Some(result);
    }

    pub fn set2_once(&mut self, result: T2) {
        if self.result2.is_none() {
            self.result2 = Some(result);
        }
    }
}

impl<T1: Display, T2: Display> Drop for Timer<T1, T2> {
    fn drop(&mut self) {
        let millis = self.start.elapsed().as_millis();

        if let Some(result) = self.result1.take() {
            println!("[day {:02}] task 1: {} ({} ms)", self.day, result, millis);
        } else {
            println!("[day {:02}] task 1: --- ({} ms)", self.day, millis);
        }

        if let Some(result) = self.result2.take() {
            println!("[day {:02}] task 2: {} ({} ms)", self.day, result, millis);
        } else {
            println!("[day {:02}] task 2: --- ({} ms)", self.day, millis);
        }
    }
}
