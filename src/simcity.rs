use rand::{thread_rng, Rng};
use yansi::Paint;
use parse_args::AppConfig;
use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

pub fn run(appconfig: &AppConfig) {

    // Thanks https://gist.github.com/erikcox/7e96d031d00d7ecb1a2f
    const SIMCITY: &[&str] = &[
        "Adding Hidden Agendas",
        "Adjusting Bell Curves",
        "Aesthesizing Industrial Areas",
        "Aligning Covariance Matrices",
        "Applying Feng Shui Shaders",
        "Applying Theatre Soda Layer",
        "Asserting Packed Exemplars",
        "Attempting to Lock Back-Buffer",
        "Binding Sapling Root System",
        "Breeding Fauna",
        "Building Data Trees",
        "Bureacritizing Bureaucracies",
        "Calculating Inverse Probability Matrices",
        "Calculating Llama Expectoration Trajectory",
        "Calibrating Blue Skies",
        "Charging Ozone Layer",
        "Coalescing Cloud Formations",
        "Cohorting Exemplars",
        "Collecting Meteor Particles",
        "Compounding Inert Tessellations",
        "Compressing Fish Files",
        "Computing Optimal Bin Packing",
        "Concatenating Sub-Contractors",
        "Containing Existential Buffer",
        "Debarking Ark Ramp",
        "Debunching Unionized Commercial Services",
        "Deciding What Message to Display Next",
        "Decomposing Singular Values",
        "Decrementing Tectonic Plates",
        "Deleting Ferry Routes",
        "Depixelating Inner Mountain Surface Back Faces",
        "Depositing Slush Funds",
        "Destabilizing Economic Indicators",
        "Determining Width of Blast Fronts",
        "Deunionizing Bulldozers",
        "Dicing Models",
        "Diluting Livestock Nutrition Variables",
        "Downloading Satellite Terrain Data",
        "Exposing Flash Variables to Streak System",
        "Extracting Resources",
        "Factoring Pay Scale",
        "Fixing Election Outcome Matrix",
        "Flood-Filling Ground Water",
        "Flushing Pipe Network",
        "Gathering Particle Sources",
        "Generating Jobs",
        "Gesticulating Mimes",
        "Graphing Whale Migration",
        "Hiding Willio Webnet Mask",
        "Implementing Impeachment Routine",
        "Increasing Accuracy of RCI Simulators",
        "Increasing Magmafacation",
        "Initializing My Sim Tracking Mechanism",
        "Initializing Rhinoceros Breeding Timetable",
        "Initializing Robotic Click-Path AI",
        "Inserting Sublimated Messages",
        "Integrating Curves",
        "Integrating Illumination Form Factors",
        "Integrating Population Graphs",
        "Iterating Cellular Automata",
        "Lecturing Errant Subsystems",
        "Mixing Genetic Pool",
        "Modeling Object Components",
        "Mopping Occupant Leaks",
        "Normalizing Power",
        "Obfuscating Quigley Matrix",
        "Overconstraining Dirty Industry Calculations",
        "Partitioning City Grid Singularities",
        "Perturbing Matrices",
        "Pixalating Nude Patch",
        "Polishing Water Highlights",
        "Populating Lot Templates",
        "Preparing Sprites for Random Walks",
        "Prioritizing Landmarks",
        "Projecting Law Enforcement Pastry Intake",
        "Realigning Alternate Time Frames",
        "Reconfiguring User Mental Processes",
        "Relaxing Splines",
        "Removing Road Network Speed Bumps",
        "Removing Texture Gradients",
        "Removing Vehicle Avoidance Behavior",
        "Resolving GUID Conflict",
        "Reticulating Splines",
        "Retracting Phong Shader",
        "Retrieving from Back Store",
        "Reverse Engineering Image Consultant",
        "Routing Neural Network Infanstructure",
        "Scattering Rhino Food Sources",
        "Scrubbing Terrain",
        "Searching for Llamas",
        "Seeding Architecture Simulation Parameters",
        "Sequencing Particles",
        "Setting Advisor Moods",
        "Setting Inner Deity Indicators",
        "Setting Universal Physical Constants",
        "Sonically Enhancing Occupant-Free Timber",
        "Speculating Stock Market Indices",
        "Splatting Transforms",
        "Stratifying Ground Layers",
        "Sub-Sampling Water Data",
        "Synthesizing Gravity",
        "Synthesizing Wavelets",
        "Time-Compressing Simulator Clock",
        "Unable to Reveal Current Activity",
        "Weathering Buildings",
        "Zeroing Crime Network"
    ];

    let mut rng = thread_rng();

    for _ in 0..500 {
        let sleep_length = rng.gen_range(200, 10000);

        // A number between 1 and 66 (inclusive)
        let mut spinner_id: u8 = ((rng.next_u32() % 66) as u8) + 1;
        let spinner_enum = match spinner_id {
            1 => Spinners::Dots,
            2 => Spinners::Dots2,
            3 => Spinners::Dots3,
            4 => Spinners::Dots4,
            5 => Spinners::Dots5,
            6 => Spinners::Dots6,
            7 => Spinners::Dots7,
            8 => Spinners::Dots8,
            9 => Spinners::Dots9,
            10 => Spinners::Dots10,
            11 => Spinners::Dots11,
            12 => Spinners::Dots12,
            13 => Spinners::Line,
            14 => Spinners::Line2,
            15 => Spinners::Pipe,
            16 => Spinners::SimpleDots,
            18 => Spinners::Star,
            19 => Spinners::Star2,
            20 => Spinners::Flip,
            21 => Spinners::Hamburger,
            22 => Spinners::GrowVertical,
            23 => Spinners::GrowHorizontal,
            24 => Spinners::Balloon,
            25 => Spinners::Balloon2,
            26 => Spinners::Noise,
            27 => Spinners::Bounce,
            28 => Spinners::BoxBounce,
            29 => Spinners::BoxBounce2,
            30 => Spinners::Triangle,
            31 => Spinners::Arc,
            32 => Spinners::Circle,
            33 => Spinners::SquareCorners,
            34 => Spinners::CircleQuarters,
            35 => Spinners::CircleHalves,
            36 => Spinners::Squish,
            37 => Spinners::Toggle,
            38 => Spinners::Toggle2,
            39 => Spinners::Toggle3,
            40 => Spinners::Toggle4,
            42 => Spinners::Toggle6,
            43 => Spinners::Toggle7,
            44 => Spinners::Toggle8,
            45 => Spinners::Toggle9,
            48 => Spinners::Toggle12,
            49 => Spinners::Toggle13,
            50 => Spinners::Arrow,
            51 => Spinners::Arrow2,
            52 => Spinners::Arrow3,
            54 => Spinners::BouncingBall,
            55 => Spinners::Smiley,
            58 => Spinners::Clock,
            59 => Spinners::Earth,
            60 => Spinners::Moon,
            64 => Spinners::Dqpb,
            65 => Spinners::Weather,
            _ => Spinners::Dots9,
        };

        // A number between 1 and 200 (inclusive)
        let mut color_id: u8 = ((rng.next_u32() % 200) as u8) + 1;
        let color_enum = match color_id {
            1 ... 10 => Paint::blue,
            10 ... 14 => Paint::green,
            14 ... 16 => Paint::yellow,
            16 ... 17 => Paint::red,
            _ => Paint::white,
        };

        // A message, chosen from SIMCITY above
        let simcity_msg = rng.choose(SIMCITY).unwrap_or(&"").to_string();

        // Create a spinner with a (sometimes) colored message)
        let sp = Spinner::new(spinner_enum, color_enum(simcity_msg + "...").to_string());
        sleep(Duration::from_millis(sleep_length));
        sp.stop();

        if appconfig.should_exit() {
            return;
        }
        println!();
    }
}
