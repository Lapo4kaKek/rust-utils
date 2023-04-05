use std::thread;
use std::sync::Arc;
use std::panic::{catch_unwind, AssertUnwindSafe};

const THRESHOLD : usize = 9;
fn process_chunk<T, F>(chunk: &[T], f: F) -> Vec<Option<T>>
where
    T: Send + 'static,
    F: Fn(&T) -> T + Send + Sync + 'static,
{
    chunk
    .into_iter()
    .map(|t| catch_unwind(AssertUnwindSafe(|| f(t))).ok())
    .collect::<Vec<Option<T>>>()
}

pub fn parallel_compute<T, F>(input: Vec<T>, f: F) -> Vec<Option<T>>
where
    T: Send + Sync + 'static + Clone,
    F: Fn(&T) -> T + Send + Sync + 'static + Clone,
{
    if input.len() < THRESHOLD {
        return input.iter().map(|x| Some(f(x))).collect();
    }
    let num_threads = 4;
    let chunk_size = (input.len() + num_threads - 1) / num_threads;
    
    let input = Arc::new(input);
    let mut handles = vec![];

    input.chunks(chunk_size).for_each(|chunk| {
        let chunk = chunk.to_vec();
        let f = f.clone();
        let handle = thread::spawn(move || {
            process_chunk(&chunk, move |x| f(x))
        });
        handles.push(handle);
    });
    let mut results = vec![];
    for handle in handles {
        results.extend(handle.join().unwrap());
    }
    results
}
