use std::sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    mpsc::Receiver,
    Arc,
};

use enchant_calc::solver;
use parking_lot::Mutex;

#[derive(Default)]
pub struct SharedData {
    pub current_step: AtomicU32,
    pub total_tries: AtomicU32,
    pub working: AtomicBool,
    pub result: Mutex<Option<solver::SolverResult>>,
}

#[derive(Debug, Clone)]
pub enum BackgroundThreadMessage {
    Calculate(Vec<solver::Enchant>),
}

pub fn background_work(
    shared_data: Arc<SharedData>,
    background_rx: Receiver<BackgroundThreadMessage>,
) {
    while let Ok(message) = background_rx.recv() {
        match message {
            BackgroundThreadMessage::Calculate(enchants) => {
                let solver = solver::Solver::new(&enchants);
                let result = solver.solve(|total_tries| {
                    shared_data.current_step.fetch_add(1, Ordering::Relaxed);
                    shared_data
                        .total_tries
                        .store(total_tries, Ordering::Relaxed);
                });

                *shared_data.result.lock() = Some(result);

                shared_data.working.store(false, Ordering::Release);
            }
        }
    }
}
