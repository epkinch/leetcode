fn main() {
}

struct MinStack {
    vec: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack {
            vec: Vec::new()
        }
    }
    
    fn push(&mut self, val: i32) {
        self.vec.push(val);
    }
    
    fn pop(&mut self) {
        self.vec.pop();
    }
    
    fn top(&self) -> i32 {
        self.vec[self.vec.len() - 1]
    }
    
    fn get_min(&self) -> i32 {
        let mut min = 0;
        for i in 0..self.vec.len() {
            if self.vec[i] < min { min = self.vec[i]; }
        }

        min
    }
}


