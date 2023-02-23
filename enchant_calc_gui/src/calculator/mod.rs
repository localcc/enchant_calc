#![allow(non_upper_case_globals)]

use self::error::CalculatorError;
use enchant_calc::solver::{self, SolverResult};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Receiver;

pub mod error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CalculatorResponse {
    Progress(u32),
    Done(SolverResult),
    Failed(String),
}

#[cfg(target = "wasm32")]
pub fn spawn_future<F>(future: F)
where
    F: std::future::Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

fn run_solver(enchants: Vec<solver::Enchant>, progress: impl FnMut(u32)) -> SolverResult {
    let solver = solver::Solver::new(&enchants);
    solver.solve(progress)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn calculate(
    enchants: Vec<solver::Enchant>,
) -> Result<Receiver<CalculatorResponse>, CalculatorError> {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let mut last_update = std::time::Instant::now();
        let mut tried_since_last_update = 0;

        let result = run_solver(enchants, |paths_tried| {
            // buffering updates to not create a lot of thread communication slowing down the application
            tried_since_last_update += paths_tried;
            if (std::time::Instant::now() - last_update).as_secs() >= 1 {
                let _ = tx.send(CalculatorResponse::Progress(tried_since_last_update));

                tried_since_last_update = 0;
                last_update = std::time::Instant::now();
            }
        });
        let _ = tx.send(CalculatorResponse::Done(result));
    });

    Ok(rx)
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
fn encode<T>(value: &T) -> Result<js_sys::Uint8Array, bincode::Error>
where
    T: Serialize,
{
    Ok(js_sys::Uint8Array::from(
        bincode::serialize(value)?.as_slice(),
    ))
}
#[cfg(target_arch = "wasm32")]
fn decode<T>(array: js_sys::Uint8Array) -> Result<T, bincode::Error>
where
    T: for<'de> Deserialize<'de>,
{
    let array = array.to_vec();
    bincode::deserialize(array.as_slice())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    #[used]
    static performance: web_sys::Performance;
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn calculate_worker(message: js_sys::Uint8Array) {
    console_error_panic_hook::set_once();

    let calculate = move || -> Result<(), CalculatorError> {
        let enchants: Vec<solver::Enchant> = decode(message)?;

        let mut last_update = performance.now();
        let mut tried_since_last_update = 0;
        let result = run_solver(enchants, |paths_tried| {
            // buffering updates to not create a lot of thread communication slowing down the application
            tried_since_last_update += paths_tried;

            if (performance.now() - last_update) >= 1000f64 {
                let encoded =
                    encode(&CalculatorResponse::Progress(tried_since_last_update)).unwrap();
                js_sys::global()
                    .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
                    .unwrap()
                    .post_message(&encoded.into())
                    .unwrap();

                tried_since_last_update = 0;
                last_update = performance.now();
            }
        });

        let encoded = encode(&CalculatorResponse::Done(result))?;
        js_sys::global()
            .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
            .unwrap()
            .post_message(&encoded.into())
            .unwrap();

        Ok(())
    };

    if let Err(e) = calculate() {
        let encoded = encode(&CalculatorResponse::Failed(e.to_string())).unwrap();
        js_sys::global()
            .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
            .unwrap()
            .post_message(&encoded.into())
            .unwrap();
    }
}

#[cfg(target_arch = "wasm32")]
pub fn calculate(
    enchants: Vec<solver::Enchant>,
) -> Result<Receiver<CalculatorResponse>, error::CalculatorError> {
    use js_sys::Uint8Array;

    let (tx, rx) = std::sync::mpsc::channel();

    let encoded: JsValue = encode(&enchants)?.into();

    let worker = web_sys::Worker::new_with_options(
        "./worker.mjs",
        web_sys::WorkerOptions::new().type_(web_sys::WorkerType::Module),
    )?;

    let callback = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::MessageEvent)>::new(
        move |msg: web_sys::MessageEvent| {
            let decoded: CalculatorResponse = decode(Uint8Array::from(msg.data())).unwrap();
            let _ = tx.send(decoded);
        },
    );

    worker.set_onmessage(Some(callback.as_ref().unchecked_ref()));
    worker.post_message(&encoded)?;

    // forgetting because we don't need the closure destroyed before worker finishes
    std::mem::forget(callback);

    Ok(rx)
}
