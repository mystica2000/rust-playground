## Ownership and Borrowing

- When it comes to storing variables, it boils down to whether the variable goes to stack or heap.

Stack - fixed (primitive types like i32, &str, char)
Heap - dynamic (vector, string::new, array) whose size can be increased, created using Box<>
