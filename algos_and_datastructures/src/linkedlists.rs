#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }
}

// Figure out a way to remove the copy trait
impl<T: PartialOrd + Copy> LinkedList<T> {
    pub fn insert_sorted(&mut self, data: T) {
        match self.0 {
            Some((val, ref mut child)) => {
                if data > val {
                    child.insert_sorted(data);
                } else {
                    self.push_front(data);
                }
            }
            None => self.push_front(data),
        }
    }
}
