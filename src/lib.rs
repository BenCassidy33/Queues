//! Queue Data Structure
//!
//! ! This crate is a hobby side project and is not intended to be used for production implementations !
//! Source Code on GitHub: <https://github.com/BenCassidy33/Queues>
//!
pub mod queue {

    #[derive(Debug)]
    pub struct Queue<T> {
        pub queue: Vec<T>,
    }

    pub trait QueueActions<T> {
        fn enqueue(&mut self, item: T);
        fn enqueue_many(&mut self, items: Vec<T>);
        fn dequeue(&mut self);
        fn remove_first(&mut self);
        fn remove_at(&mut self, idx: usize);
        fn destroy(&mut self);
    }

    impl<T> QueueActions<T> for Queue<T> {
        /// Will add an item to the end of the queue
        ///
        /// # Examples
        ///
        /// ```
        /// use lineup_rust::queue::{create_queue, QueueActions};
        ///
        /// let mut queue = create_queue(vec![1, 2, 3, 4, 5]);
        /// QueueActions::enqueue(&mut queue, 6);
        /// assert_eq!(queue.queue, [1, 2, 3, 4, 5, 6]);
        /// ````
        fn enqueue(&mut self, item: T) {
            self.queue.push(item);
        }

        /// Will add multiple items to the end of the queue 
        ///
        /// # Examples
        ///
        /// ```
        /// use lineup_rust::queue::{create_queue, QueueActions};
        ///
        /// let mut queue = create_queue(vec![1, 2, 3, 4, 5]);
        /// QueueActions::enqueue_many(&mut queue, vec![6, 7, 8, 9, 10]);
        /// assert_eq!(queue.queue, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        /// ```
        fn enqueue_many(&mut self, items: Vec<T>) {
            for item in items {
                self.queue.push(item);
            }
        }

        /// Will remove items from the end of the queue
        ///
        /// # Examples
        ///
        /// ```
        /// use lineup_rust::queue::{create_queue, QueueActions};
        ///
        /// let mut queue = create_queue(vec![1, 2, 3, 4, 5]);
        /// QueueActions::dequeue(&mut queue);
        /// assert_eq!(queue.queue, [1, 2, 3, 4]);
        /// ```
        fn dequeue(&mut self) {
            self.queue.pop();
        }

        /// Will remove the first item from the queue
        ///
        /// # Examples
        /// ```
        /// use lineup_rust::queue::{create_queue, QueueActions};
        ///
        /// let mut queue = create_queue(vec![1, 2, 3, 4, 5]);
        /// QueueActions::remove_first(&mut queue);
        /// assert_eq!(queue.queue, [2, 3, 4, 5]);
        /// ````
        fn remove_first(&mut self) {
            self.queue.remove(0);
        }

        /// Will remove an item at a given index if the index is valid (keep in mind that the first index is 0)
        /// Will panic if index is out of bounds for the queue
        ///
        /// # Examples
        /// ```
        /// use lineup_rust::queue::{create_queue, QueueActions};
        ///
        /// let mut queue = create_queue(vec![1, 2, 3, 4, 5]);
        /// QueueActions::remove_at(&mut queue, 2);
        /// assert_eq!(queue.queue, [1, 2, 4, 5]);
        /// ```
        fn remove_at(&mut self, idx: usize) {
            if idx <= self.queue.len() {
                self.queue.remove(idx);
            } else {
                panic!("Index out of bounds");
            };
        }

        /// Will remove all items from the queue
        ///
        /// # Examples
        ///
        /// ```
        /// use lineup_rust::queue::{create_queue, QueueActions};
        ///
        /// let mut queue = create_queue(vec![1, 2, 3, 4, 5]);
        /// QueueActions::destroy(&mut queue);
        /// assert_eq!(queue.queue, []);
        /// ````
        fn destroy(&mut self) {
            self.queue.clear();
        }
    }

    /// Will create a new queue with the given items
    ///
    /// # Examples
    ///
    /// ```
    /// use lineup_rust::queue::create_queue;
    ///
    /// let queue = create_queue(vec![1, 2, 3, 4, 5]);
    /// assert_eq!(queue.queue, [1, 2, 3, 4, 5]);
    /// ````
    pub fn create_queue<T>(items: Vec<T>) -> Queue<T> {
        return Queue { queue: items };
    }

    /// Will create a new empty queue
    ///
    /// # Examples
    ///
    /// ```
    /// use lineup_rust::queue::{create_empty, Queue};
    ///
    /// let empty: Queue<i32> = lineup_rust::queue::create_empty();
    /// assert_eq!(empty.queue, []);
    /// ````
    pub fn create_empty<T>() -> Queue<T> {
        return Queue { queue: Vec::new() };
    }
}
