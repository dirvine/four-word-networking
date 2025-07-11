use std::time::Instant;
use three_word_networking::dictionary65k;
use three_word_networking::dictionary16k;

fn main() {
    println!("Benchmarking dictionary performance...\n");
    
    // Benchmark 65K dictionary
    benchmark_dictionary_65k();
    
    // Benchmark 16K dictionary
    benchmark_dictionary_16k();
}

fn benchmark_dictionary_65k() {
    println!("=== 65K Dictionary Benchmarks ===");
    
    // Benchmark initialization
    let start = Instant::now();
    let dict = dictionary65k::get_global_dictionary().unwrap();
    let init_time = start.elapsed();
    println!("Initialization time: {:?}", init_time);
    
    // Benchmark word lookups
    let iterations = 1_000_000;
    let start = Instant::now();
    for i in 0..iterations {
        let idx = (i % 65536) as u16;
        let _ = dict.get_word(idx).unwrap();
    }
    let lookup_time = start.elapsed();
    let per_lookup = lookup_time / iterations;
    println!("Word lookup: {:?} per operation ({} ops/sec)", 
             per_lookup, 
             (iterations as f64 / lookup_time.as_secs_f64()) as u64);
    
    // Benchmark index lookups
    let test_words: Vec<_> = (0..100)
        .map(|i| dict.get_word((i * 655) as u16).unwrap().to_string())
        .collect();
    
    let start = Instant::now();
    for _ in 0..iterations/100 {
        for word in &test_words {
            let _ = dict.get_index(word).unwrap();
        }
    }
    let index_time = start.elapsed();
    let per_index = index_time / iterations;
    println!("Index lookup: {:?} per operation ({} ops/sec)", 
             per_index,
             (iterations as f64 / index_time.as_secs_f64()) as u64);
    
    // Benchmark concurrent access
    use std::thread;
    let num_threads = 8;
    let ops_per_thread = 100_000;
    
    let start = Instant::now();
    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            thread::spawn(move || {
                let dict = dictionary65k::get_global_dictionary().unwrap();
                for i in 0..ops_per_thread {
                    let idx = (i % 65536) as u16;
                    let word = dict.get_word(idx).unwrap();
                    let _ = dict.get_index(word).unwrap();
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    let concurrent_time = start.elapsed();
    let total_ops = num_threads * ops_per_thread * 2;
    println!("Concurrent access: {} ops in {:?} ({} ops/sec)\n",
             total_ops,
             concurrent_time,
             (total_ops as f64 / concurrent_time.as_secs_f64()) as u64);
}

fn benchmark_dictionary_16k() {
    println!("=== 16K Dictionary Benchmarks ===");
    
    // Benchmark initialization
    let start = Instant::now();
    let dict = dictionary16k::get_global_dictionary().unwrap();
    let init_time = start.elapsed();
    println!("Initialization time: {:?}", init_time);
    
    // Benchmark word lookups
    let iterations = 1_000_000;
    let start = Instant::now();
    for i in 0..iterations {
        let idx = (i % 16384) as u16;
        let _ = dict.get_word(idx).unwrap();
    }
    let lookup_time = start.elapsed();
    let per_lookup = lookup_time / iterations;
    println!("Word lookup: {:?} per operation ({} ops/sec)", 
             per_lookup, 
             (iterations as f64 / lookup_time.as_secs_f64()) as u64);
    
    // Benchmark index lookups
    let test_words: Vec<_> = (0..100)
        .map(|i| dict.get_word((i * 163) as u16).unwrap().to_string())
        .collect();
    
    let start = Instant::now();
    for _ in 0..iterations/100 {
        for word in &test_words {
            let _ = dict.get_index(word).unwrap();
        }
    }
    let index_time = start.elapsed();
    let per_index = index_time / iterations;
    println!("Index lookup: {:?} per operation ({} ops/sec)", 
             per_index,
             (iterations as f64 / index_time.as_secs_f64()) as u64);
    
    // Benchmark concurrent access
    use std::thread;
    let num_threads = 8;
    let ops_per_thread = 100_000;
    
    let start = Instant::now();
    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            thread::spawn(move || {
                let dict = dictionary16k::get_global_dictionary().unwrap();
                for i in 0..ops_per_thread {
                    let idx = (i % 16384) as u16;
                    let word = dict.get_word(idx).unwrap();
                    let _ = dict.get_index(word).unwrap();
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    let concurrent_time = start.elapsed();
    let total_ops = num_threads * ops_per_thread * 2;
    println!("Concurrent access: {} ops in {:?} ({} ops/sec)",
             total_ops,
             concurrent_time,
             (total_ops as f64 / concurrent_time.as_secs_f64()) as u64);
}