mod solutions;

fn main() {
    let got = solutions::task_scheduler::heap_impl::least_interval(
        vec![
            'A', 'A', 'A', 'B', 'B', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
        ],
        7,
    );
    println!("GOT: {got}");
}
