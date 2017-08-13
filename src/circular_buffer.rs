const BUFFER_SIZE: usize = 10;

// Circular buffer that stores the 10 most recently entered items
pub struct CircularBuffer {
    elements: [String; BUFFER_SIZE],
    front: i32,
    back: i32,
    buffer_length: i32
}

#[allow(dead_code)]
impl CircularBuffer {

    pub fn new() -> CircularBuffer {
        CircularBuffer { 
            elements: Default::default(),
            front: 0,
            back: -1,
            buffer_length: 0
        } 
    }

    /* Returns the size of the buffer */
    fn get_size(&self) -> i32 {
        self.buffer_length
    }

    /* Clears all the elements in the buffer*/
    fn clear(&mut self) {
        self.front = 0;
        self.back = 0;
        self.buffer_length = 0;
    }

    /* Checks if the buffer is empty */
    fn is_empty(&mut self) -> bool {
        self.buffer_length == 0
    }

    /* Checks if the buffer is full */
    fn is_full(&self) -> bool {
        self.buffer_length == BUFFER_SIZE as i32
    }

    /* Inserts a new element at the end of the buffer 
     * If the buffer is full then the oldest element is overwritten. */
    pub fn write(&mut self, elem: String) {
        if self.is_full() {
            self.delete();
        }

        self.buffer_length = self.buffer_length + 1;
        self.back = (self.back + 1) % BUFFER_SIZE as i32;
        self.elements[self.back as usize] = elem;
    }

    /* Effectively deletes the oldest element in the buffer */
    fn delete(&mut self) {
        if !self.is_empty() {
            self.buffer_length = self.buffer_length - 1;
            self.front = (self.front + 1) % BUFFER_SIZE as i32;
        }
    }

    /* Prints out all the elements in the buffer */
    pub fn print_all(&self) {
        let len = self.buffer_length;

        for i in 0..len {
            let idx = (self.front + i) as usize;
            println!("{}", self.elements[idx % BUFFER_SIZE]);
        }
    }
}
