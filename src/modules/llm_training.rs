//! Pretend to train an LLM
use async_trait::async_trait;
use humansize::{FormatSizeOptions, format_size};
use rand::RngExt;
use rand::rng;
use rand::seq::IndexedRandom;

use crate::args::AppConfig;
use crate::data::{LLM_GPUS_LIST, LLM_MODELS_LIST};
use crate::io::{csleep, erase_line, newline, print};
use crate::modules::Module;

pub struct Gpu<'a> {
    pub name: &'a str,
    pub vram: u64,
}

pub struct LlmTraining;

/// Formats a time duration in seconds as MM:SS
///
/// Example: 345 seconds -> "05:45"
fn format_secs_as_mm_ss(secs: u64) -> String {
    format!("{:02}:{:02}", secs / 60, secs % 60)
}

/// Runs a progress bar animation with the given parameters.
/// `render` is called for each frame with
/// (frame_index, step, pct, elapsed_secs, remaining_secs, bar)
/// and should return the string to print.
async fn run_progress_bar<F>(
    total: usize,
    bar_steps: u32,
    step_ms: u64,
    simulated_total_secs: u64,
    mut render: F,
    appconfig: &AppConfig,
) where
    F: FnMut(u32, usize, f64, u64, u64, &progress_string::Bar) -> String,
{
    let mut bar = progress_string::BarBuilder::new()
        .total(total)
        .full_char('█')
        .width(10)
        .build();

    for i in 0..=bar_steps {
        erase_line().await;

        let step = (total * i as usize / bar_steps as usize).min(total);
        let pct = step as f64 / total as f64 * 100.0;
        bar.replace(step);

        let elapsed_secs = simulated_total_secs * i as u64 / bar_steps as u64;
        let remaining_secs = simulated_total_secs * (bar_steps - i) as u64 / bar_steps as u64;

        let text = render(i, step, pct, elapsed_secs, remaining_secs, &bar);
        print(text).await;

        if i < bar_steps {
            csleep(step_ms).await;
        }

        if appconfig.should_exit() {
            return;
        }
    }
}

#[async_trait(?Send)]
impl Module for LlmTraining {
    fn name(&self) -> &'static str {
        "llm_training"
    }

    fn signature(&self) -> String {
        "python train.py --model gpt2 --dataset wikitext".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = rng();

        // selects a random GPU and logs the device details
        let gpu = LLM_GPUS_LIST.choose(&mut rng).unwrap();
        // Use format_size to properly display VRAM in human-readable units (e.g., "80 GB")
        let vram_bytes = gpu.vram * 1024 * 1024; // MB -> bytes
        let vram_opts = FormatSizeOptions::from(humansize::BINARY).space_after_value(false);
        print(format!(
            "[INFO] Device found: {} ({})",
            gpu.name,
            format_size(vram_bytes, vram_opts)
        ))
        .await;
        newline().await;
        csleep(300).await;

        // chooses mixed precision training using either "bf16" or "fp16"
        let precision = if rng.random_bool(0.6) { "bf16" } else { "fp16" };
        print(format!(
            "[INFO] Using mixed precision training ({precision})."
        ))
        .await;
        newline().await;

        // configures gradient accumulation steps, micro-batch size, and calculates the total batch size
        let grad_acc_steps = [2u32, 4, 8, 16].choose(&mut rng).unwrap();
        let micro_batch = [8u32, 16, 32].choose(&mut rng).unwrap();
        let total_batch = grad_acc_steps * micro_batch;
        print(format!(
            "[INFO] Gradient accumulation steps: {grad_acc_steps} | Total batch size: {total_batch}"
        ))
        .await;
        newline().await;

        // initializes weights from a randomly selected model
        let model = LLM_MODELS_LIST.choose(&mut rng).unwrap();
        print(format!(
            "[INFO] Initializing model weights from '{model}'... Done."
        ))
        .await;
        newline().await;
        csleep(500).await;

        // downloads model files
        let file_sizes = vec![
            rng.random_range(400_000..3_000_000),
            rng.random_range(10_000..80_000),
            rng.random_range(200_000_000..7_000_000_000),
            rng.random_range(1_000..8_000),
        ];

        let bar_steps = 12u64;
        let step_ms = 100u64;

        for &file_size in &file_sizes {
            let speed = rng.random_range(1_000_000..80_000_000);
            let simulated_total_secs = (file_size / speed).max(1);

            let size_opts = FormatSizeOptions::from(humansize::BINARY).space_after_value(false);
            let speed_opts = FormatSizeOptions::from(humansize::BINARY)
                .space_after_value(false)
                .suffix("/s");

            run_progress_bar(
                file_size as usize,
                bar_steps as u32,
                step_ms,
                simulated_total_secs,
                move |i, downloaded, pct, elapsed_secs, remaining_secs, bar| {
                    if i == 0 {
                        format!(
                            "  0%[          ] 0.00/{} [00:00<?, ?/s]",
                            format_size(file_size, size_opts)
                        )
                    } else {
                        format!(
                            "{pct:>3.0}%{bar} {downloaded}/{total} [{elapsed}<{eta}, {speed}]",
                            downloaded = format_size(downloaded as u64, size_opts),
                            total = format_size(file_size, size_opts),
                            elapsed = format_secs_as_mm_ss(elapsed_secs),
                            eta = format_secs_as_mm_ss(remaining_secs),
                            speed = format_size(speed, speed_opts),
                        )
                    }
                },
                appconfig,
            )
            .await;

            newline().await;
        }

        // logs the number of instances in the training dataset
        let num_instances = rng.random_range(10_000..100_000);
        print(format!("Training set has {num_instances} instances")).await;
        newline().await;
        newline().await;

        let num_epochs = rng.random_range(3..8);
        let steps_per_epoch = rng.random_range(500..2000);
        let mut loss = rng.random_range(3.0..5.0);

        let epoch_bar_steps = 12u32;
        let epoch_step_ms = 80u64;

        let simulated_epoch_secs = rng.random_range(60..300);

        // do the training process over a random number of epochs with progress bars
        for epoch in 1..=num_epochs {
            run_progress_bar(
                steps_per_epoch as usize,
                epoch_bar_steps,
                epoch_step_ms,
                simulated_epoch_secs,
                |i, step, pct, elapsed_secs, remaining_secs, bar| {
                    if i == 0 {
                        format!(
                            "Epoch {epoch}/{num_epochs}:   0%[          ] 0/{steps_per_epoch} [00:00<?, ?it/s]"
                        )
                    } else {
                        let it_per_s = step as f64 / elapsed_secs.max(1) as f64;
                        let step_loss =
                            loss - rng.random_range(0.0..0.2) * i as f64 / epoch_bar_steps as f64;
                        format!(
                            "Epoch {epoch}/{num_epochs}: {pct:>3.0}%{bar} {step}/{steps_per_epoch} [{elapsed}<{eta}, {it:.2}it/s, loss={loss:.4}]",
                            elapsed = format_secs_as_mm_ss(elapsed_secs),
                            eta = format_secs_as_mm_ss(remaining_secs),
                            it = it_per_s,
                            loss = step_loss,
                        )
                    }
                },
                appconfig,
            )
            .await;

            let prev_loss = loss;
            loss -= rng.random_range(0.3..0.8);
            loss = loss.max(0.05);
            newline().await;

            // randomly triggers checkpoint saves to persist the model state
            if rng.random_bool(0.4) {
                let checkpoint_step = epoch * steps_per_epoch;
                print(format!(
                    "[CHECKPOINT] Model improved ({prev_loss:.4} -> {loss:.4}). Saving model to ./output/checkpoint-{checkpoint_step}..."
                ))
                .await;
                newline().await;
                csleep(rng.random_range(1_000..2_000)).await;
                print(format!(
                    "[INFO] Configuration saved in ./output/checkpoint-{checkpoint_step}/config.json"
                ))
                .await;
                newline().await;
            }

            if appconfig.should_exit() {
                return;
            }
        }

        let val_steps = rng.random_range(50..300);
        let val_simulated_secs = rng.random_range(3..15);
        let val_bar_steps = 12u32;
        let val_step_ms = 80u64;

        // we do a validation step after training
        run_progress_bar(
            val_steps as usize,
            val_bar_steps,
            val_step_ms,
            val_simulated_secs,
            |i, step, pct, elapsed_secs, remaining_secs, bar| {
                if i == 0 {
                    format!(
                        "Validating:   0%[          ] 0/{val_steps} [00:00<?, ?it/s]"
                    )
                } else {
                    let it_per_s = step as f64 / elapsed_secs.max(1) as f64;
                    format!(
                        "Validating: {pct:>3.0}%{bar} {step}/{val_steps} [{elapsed}<{eta}, {it:.2}it/s]",
                        elapsed = format_secs_as_mm_ss(elapsed_secs),
                        eta = format_secs_as_mm_ss(remaining_secs),
                        it = it_per_s,
                    )
                }
            },
            appconfig,
        )
        .await;

        newline().await;

        // displays the validation results
        let val_loss = loss + rng.random_range(0.05..0.3);
        let perplexity = val_loss.exp();
        print(format!(
            "> Validation results: loss={val_loss:.4} | perplexity={perplexity:.2}"
        ))
        .await;
        newline().await;

        csleep(500).await;
    }
}
