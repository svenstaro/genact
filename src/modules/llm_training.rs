//! Pretend to train an LLM
use async_trait::async_trait;
use humansize::{FormatSizeOptions, format_size};
use rand::seq::IndexedRandom;
use rand::{RngExt, rng};

use crate::args::AppConfig;
use crate::data::{LLM_GPUS_LIST, LLM_MODELS_LIST};
use crate::io::{csleep, erase_line, newline, print};
use crate::modules::Module;

pub struct Gpu<'a> {
    pub name: &'a str,
    pub vram: u64,
}

pub struct LlmTraining;

fn fmt_time(secs: u64) -> String {
    format!("{:02}:{:02}", secs / 60, secs % 60)
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

        let gpu = LLM_GPUS_LIST.choose(&mut rng).unwrap();
        let (gpu_name, gpu_vram) = (gpu.name, gpu.vram);
        print(format!("[INFO] Device found: {gpu_name} ({gpu_vram}MB)")).await;
        newline().await;
        csleep(300).await;

        let precision = if rng.random_bool(0.6) { "bf16" } else { "fp16" };
        print(format!(
            "[INFO] Using mixed precision training ({precision})."
        ))
        .await;
        newline().await;

        let grad_acc_steps = [2u32, 4, 8, 16].choose(&mut rng).unwrap();
        let micro_batch = [8u32, 16, 32].choose(&mut rng).unwrap();
        let total_batch = grad_acc_steps * micro_batch;
        print(format!(
            "[INFO] Gradient accumulation steps: {grad_acc_steps} | Total batch size: {total_batch}"
        ))
        .await;
        newline().await;

        let model = LLM_MODELS_LIST.choose(&mut rng).unwrap();
        print(format!(
            "[INFO] Initializing model weights from '{model}'... Done."
        ))
        .await;
        newline().await;
        csleep(500).await;

        let file_sizes: Vec<u64> = vec![
            rng.random_range(400_000..3_000_000),
            rng.random_range(10_000..80_000),
            rng.random_range(200_000_000..7_000_000_000),
            rng.random_range(1_000..8_000),
        ];

        let bar_steps = 12u64;
        let step_ms = 100u64;

        for &file_size in &file_sizes {
            let speed: u64 = rng.random_range(1_000_000..80_000_000);
            let simulated_total_secs = file_size / speed;

            let mut bar = progress_string::BarBuilder::new()
                .total(file_size as usize)
                .full_char('█')
                .width(10)
                .build();

            for step in 0..=bar_steps {
                erase_line().await;

                let downloaded = (file_size * step / bar_steps).min(file_size);
                let pct = downloaded as f64 / file_size as f64 * 100.0;
                bar.replace(downloaded as usize);

                let size_opts = FormatSizeOptions::from(humansize::BINARY).space_after_value(false);
                let speed_opts = FormatSizeOptions::from(humansize::BINARY)
                    .space_after_value(false)
                    .suffix("/s");

                if step == 0 {
                    print(format!(
                        "  0%|          | 0.00/{} [00:00<?, ?/s]",
                        format_size(file_size, size_opts)
                    ))
                    .await;
                } else {
                    let elapsed_secs = simulated_total_secs * step / bar_steps;
                    let remaining_secs = simulated_total_secs * (bar_steps - step) / bar_steps;
                    print(format!(
                        "{pct:>3.0}%|{bar}| {downloaded}/{total} [{elapsed}<{eta}, {speed}]",
                        bar = bar,
                        downloaded = format_size(downloaded, size_opts),
                        total = format_size(file_size, size_opts),
                        elapsed = fmt_time(elapsed_secs),
                        eta = fmt_time(remaining_secs),
                        speed = format_size(speed, speed_opts),
                    ))
                    .await;
                }

                if step < bar_steps {
                    csleep(step_ms).await;
                }

                if appconfig.should_exit() {
                    return;
                }
            }

            newline().await;
            newline().await;
        }

        let num_instances: u32 = rng.random_range(10_000..100_000);
        print(format!("Training set has {num_instances} instances")).await;
        newline().await;
        newline().await;

        let num_epochs: u32 = rng.random_range(3..8);
        let steps_per_epoch: u32 = rng.random_range(500..2000);
        let mut loss: f64 = rng.random_range(3.0..5.0);

        let epoch_bar_steps = 12u32;
        let epoch_step_ms = 80u64;

        let simulated_epoch_secs: u64 = rng.random_range(60..300);

        for epoch in 1..=num_epochs {
            let mut epoch_bar = progress_string::BarBuilder::new()
                .total(steps_per_epoch as usize)
                .full_char('█')
                .width(10)
                .build();

            for i in 0..=epoch_bar_steps {
                erase_line().await;

                let step = (steps_per_epoch * i / epoch_bar_steps).min(steps_per_epoch);
                let pct = step as f64 / steps_per_epoch as f64 * 100.0;
                let step_loss =
                    loss - rng.random_range(0.0_f64..0.2) * i as f64 / epoch_bar_steps as f64;
                epoch_bar.replace(step as usize);

                if i == 0 {
                    print(format!(
                        "Epoch {epoch}/{num_epochs}:   0%|          | 0/{steps_per_epoch} [00:00<?, ?it/s]"
                    ))
                    .await;
                } else {
                    let elapsed_secs = simulated_epoch_secs * i as u64 / epoch_bar_steps as u64;
                    let remaining_secs = simulated_epoch_secs * (epoch_bar_steps - i) as u64
                        / epoch_bar_steps as u64;
                    let it_per_s = step as f64 / elapsed_secs.max(1) as f64;
                    print(format!(
                        "Epoch {epoch}/{num_epochs}: {pct:>3.0}%|{bar}| {step}/{steps_per_epoch} [{elapsed}<{eta}, {it:.2}it/s, loss={loss:.4}]",
                        bar = epoch_bar,
                        elapsed = fmt_time(elapsed_secs),
                        eta = fmt_time(remaining_secs),
                        it = it_per_s,
                        loss = step_loss,
                    ))
                    .await;
                }

                if i < epoch_bar_steps {
                    csleep(epoch_step_ms).await;
                }

                if appconfig.should_exit() {
                    return;
                }
            }

            let prev_loss = loss;
            loss -= rng.random_range(0.3_f64..0.8);
            loss = loss.max(0.05);
            newline().await;

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

        let val_steps: u32 = rng.random_range(50..300);
        let val_simulated_secs: u64 = rng.random_range(3..15);
        let val_bar_steps = 12u32;
        let val_step_ms = 80u64;

        let mut val_bar = progress_string::BarBuilder::new()
            .total(val_steps as usize)
            .full_char('█')
            .width(10)
            .build();

        for i in 0..=val_bar_steps {
            erase_line().await;

            let step = (val_steps * i / val_bar_steps).min(val_steps);
            let pct = step as f64 / val_steps as f64 * 100.0;
            val_bar.replace(step as usize);

            if i == 0 {
                print(format!(
                    "Validating:   0%|          | 0/{val_steps} [00:00<?, ?it/s]"
                ))
                .await;
            } else {
                let elapsed_secs = val_simulated_secs * i as u64 / val_bar_steps as u64;
                let remaining_secs =
                    val_simulated_secs * (val_bar_steps - i) as u64 / val_bar_steps as u64;
                let it_per_s = step as f64 / elapsed_secs.max(1) as f64;
                print(format!(
                    "Validating: {pct:>3.0}%|{bar}| {step}/{val_steps} [{elapsed}<{eta}, {it:.2}it/s]",
                    bar = val_bar,
                    elapsed = fmt_time(elapsed_secs),
                    eta = fmt_time(remaining_secs),
                    it = it_per_s,
                ))
                .await;
            }

            if i < val_bar_steps {
                csleep(val_step_ms).await;
            }

            if appconfig.should_exit() {
                return;
            }
        }

        newline().await;

        let val_loss = loss + rng.random_range(0.05_f64..0.3);
        let perplexity = val_loss.exp();
        print(format!(
            "> Validation results: loss={val_loss:.4} | perplexity={perplexity:.2}"
        ))
        .await;
        newline().await;

        csleep(500).await;
    }
}
