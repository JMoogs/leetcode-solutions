
struct MyQueue {
    stack: Vec<i32>,
    head: usize,
}
/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self {
            stack: Vec::new(),
            head: 0,
        }
    }
    
    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        self.head += 1;
        self.stack[self.head - 1]
    }
    
    fn peek(&self) -> i32 {
        self.stack[self.head]
    }
    
    fn empty(&self) -> bool {
        self.stack.len() <= self.head
    }
}
/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */