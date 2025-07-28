
use std::thread;

pub fn simulate_multi_thread_logging<L>(mut logger: L)
where
    L: 'static + Send + std::sync::MutexGuard<'static> + super::logger::traits::Logger,
{
    let handle = thread::spawn(move || {
        for i in 0..5 {
            logger.log(&format!("Thread log message {}", i));
        }
    });

    handle.join().unwrap();
}
