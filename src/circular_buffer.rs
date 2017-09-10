use std::fmt::Display;

pub struct CircularBuffer<T : Display> {
    elements: Vec<T>,
    capacity: usize
}

#[allow(dead_code)]
impl <T> CircularBuffer<T> where T: Display {

    pub fn new(c: usize) -> CircularBuffer<T> {
        CircularBuffer { 
            elements: Vec::with_capacity(c),
            capacity: c
        } 
    }

    /* Returns the size of the buffer */
    fn get_size(&self) -> usize {
        self.elements.len()
    }

    /* Clears all the elements in the buffer*/
    fn clear(&mut self) {
        self.elements.clear();
    }

    /* Checks if the buffer is empty */
    fn is_empty(&mut self) -> bool {
        self.elements.is_empty() 
    }

    /* Checks if the buffer is full */
    fn is_full(&self) -> bool {
        self.get_size() == self.capacity 
    }

    /* Inserts a new element at the end of the buffer 
     * If the buffer is full then the oldest element is overwritten. */
    pub fn write(&mut self, elem: T) {
        if self.is_full() {
            self.delete();
        }

        self.elements.push(elem); 
    }

    /* deletes the oldest element in the buffer */
    fn delete(&mut self) {
        if !self.is_empty() {
            self.elements.remove(0);
        }
    }

    /* Prints out all the elements in the buffer */
    pub fn print_all(&self) {
        for element in self.elements.iter() {
            println!("{}", element);
        }
    }
}
