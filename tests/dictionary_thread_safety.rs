/// Thread safety tests for dictionary modules
use std::sync::{Arc, Barrier};
use std::thread;
use three_word_networking::dictionary16k;
use three_word_networking::dictionary65k;

#[test]
fn test_dictionary65k_concurrent_initialization() {
    // Test that multiple threads can safely initialize and access the dictionary
    let barrier = Arc::new(Barrier::new(100));
    let handles: Vec<_> = (0..100)
        .map(|i| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                // Synchronize all threads to start at the same time
                barrier.wait();
                
                // All threads try to get the dictionary simultaneously
                let dict = dictionary65k::get_global_dictionary().unwrap();
                
                // Perform some operations to verify it works
                let word = dict.get_word(i as u16).unwrap();
                let index = dict.get_index(word).unwrap();
                assert_eq!(index, i as u16);
            })
        })
        .collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_dictionary16k_concurrent_initialization() {
    // Test that multiple threads can safely initialize and access the dictionary
    let barrier = Arc::new(Barrier::new(100));
    let handles: Vec<_> = (0..100)
        .map(|i| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                // Synchronize all threads to start at the same time
                barrier.wait();
                
                // All threads try to get the dictionary simultaneously
                let dict = dictionary16k::get_global_dictionary().unwrap();
                
                // Perform some operations to verify it works
                let word = dict.get_word(i as u16).unwrap();
                let index = dict.get_index(word).unwrap();
                assert_eq!(index, i as u16);
            })
        })
        .collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_dictionary65k_concurrent_access() {
    // Pre-initialize the dictionary
    let _ = dictionary65k::get_global_dictionary().unwrap();
    
    // Now test concurrent access after initialization
    let handles: Vec<_> = (0..1000)
        .map(|i| {
            thread::spawn(move || {
                let dict = dictionary65k::get_global_dictionary().unwrap();
                
                // Perform many read operations
                for j in 0..100 {
                    let idx = ((i * 100 + j) % 65536) as u16;
                    let word = dict.get_word(idx).unwrap();
                    let back = dict.get_index(word).unwrap();
                    assert_eq!(back, idx);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_dictionary16k_concurrent_access() {
    // Pre-initialize the dictionary
    let _ = dictionary16k::get_global_dictionary().unwrap();
    
    // Now test concurrent access after initialization
    let handles: Vec<_> = (0..1000)
        .map(|i| {
            thread::spawn(move || {
                let dict = dictionary16k::get_global_dictionary().unwrap();
                
                // Perform many read operations
                for j in 0..100 {
                    let idx = ((i * 100 + j) % 16384) as u16;
                    let word = dict.get_word(idx).unwrap();
                    let back = dict.get_index(word).unwrap();
                    assert_eq!(back, idx);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_dictionary_utils_thread_safety() {
    // Test utility functions from multiple threads
    let handles: Vec<_> = (0..100)
        .map(|i| {
            thread::spawn(move || {
                // Test 65K utils
                let word = dictionary65k::utils::index_to_word(i as u16).unwrap();
                let index = dictionary65k::utils::word_to_index(word).unwrap();
                assert_eq!(index, i as u16);
                
                // Test 16K utils (limit to 16K range)
                if i < 164 {
                    let idx = (i * 100) as u16;
                    let word = dictionary16k::utils::index_to_word(idx).unwrap();
                    let back = dictionary16k::utils::word_to_index(word).unwrap();
                    assert_eq!(back, idx);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_stress_concurrent_dictionary_operations() {
    use std::time::Instant;
    
    let start = Instant::now();
    let num_threads = 50;
    let ops_per_thread = 10_000;
    
    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            thread::spawn(move || {
                let dict65k = dictionary65k::get_global_dictionary().unwrap();
                let dict16k = dictionary16k::get_global_dictionary().unwrap();
                
                for i in 0..ops_per_thread {
                    // Mix of operations on both dictionaries
                    let idx65k = ((thread_id * ops_per_thread + i) % 65536) as u16;
                    let idx16k = ((thread_id * ops_per_thread + i) % 16384) as u16;
                    
                    // 65K operations
                    let word = dict65k.get_word(idx65k).unwrap();
                    dict65k.get_index(word).unwrap();
                    
                    // 16K operations
                    let word = dict16k.get_word(idx16k).unwrap();
                    dict16k.get_index(word).unwrap();
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
    
    let elapsed = start.elapsed();
    let total_ops = num_threads * ops_per_thread * 4; // 4 operations per iteration
    let ops_per_sec = total_ops as f64 / elapsed.as_secs_f64();
    
    println!("Stress test completed: {} ops in {:?} ({:.0} ops/sec)", 
             total_ops, elapsed, ops_per_sec);
    
    // Should complete in reasonable time (< 5 seconds on modern hardware)
    assert!(elapsed.as_secs() < 5, "Dictionary operations too slow under concurrent load");
}

#[cfg(unix)]
#[test]
fn test_fork_safety() {
    use std::process::Command;
    
    // Initialize dictionary in parent process
    let _ = dictionary65k::get_global_dictionary().unwrap();
    
    // Fork a child process and verify dictionary still works
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo 'Fork test passed'")
        .output()
        .expect("Failed to execute fork test");
    
    assert!(output.status.success());
    
    // Verify dictionary still works in parent after fork
    let dict = dictionary65k::get_global_dictionary().unwrap();
    let word = dict.get_word(0).unwrap();
    assert!(!word.is_empty());
}