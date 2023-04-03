use std::thread;
use std::sync::Arc;
use std::panic::{catch_unwind, AssertUnwindSafe};

const THERESHOLD : usize = 9;
pub fn parallel_compute<T, R, F>(input: Vec<T>, f: F) -> Vec<Option<R>>
where
    T: Send + 'static + std::clone::Clone,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
    {
    let input_len = input.len();

    if input_len <= THERESHOLD {
        return input.into_iter().map(|t| Some(f(t))).collect();
    }

    let num_threads = 4;
    let chunk_size = (input_len + num_threads - 1) / num_threads;
    let f = Arc::new(f);

    let mut handles = Vec::with_capacity(num_threads);

    for chunk in input.chunks(chunk_size) {
        let f = f.clone();
        let chunk = chunk.to_vec();
        let handle = thread::spawn(move || {
            chunk
                .into_iter()
                .map(|t| catch_unwind(AssertUnwindSafe(|| f(t))).ok())
                .collect::<Vec<Option<R>>>()
        });
        handles.push(handle);
    }

    let mut results = Vec::with_capacity(input_len);

    for handle in handles {
        results.extend(handle.join().unwrap());
    }
    results
}



