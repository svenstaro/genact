use rand::{thread_rng, Rng};
use parse_args::AppConfig;
use utils::{dprint, csleep};
use yansi::Paint;

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

    const SPINNERS: &[&str] = &["/", "-", "\\", "|"];
    const SPINNER_SLEEP: u64 = 50;
    const TEXT_SLEEP: u64 = 15;
    const MAX_SPINNER_LOOPS: u8 = 20;

    let mut rng = thread_rng();
    let mut simcity = "";

    for _ in 0..500 {
        let spinner_loops = rng.gen_range(1, MAX_SPINNER_LOOPS);

        // Message chosen from SIMCITY above
        let last_simcity = simcity;
        simcity = rng.choose(SIMCITY).unwrap_or(&"");

        // Don't choose the same message twice in a row
        while simcity == last_simcity {
            // Select another message
            simcity = rng.choose(SIMCITY).unwrap_or(&"");
        }

        // Choose a status/resolution per "task"
        let resolution_id: u8 = ((rng.next_u32() % 100) as u8) + 1;
        let resolution = match resolution_id {
            1 ... 4 => "FAIL",
            5 ... 9 => "YES",
            10 ... 14 => "SUCCESS",
            _ => "OK",
        };

        // Select a color
        let mut color_id: u8 = ((rng.next_u32() % 20) as u8) + 1;
        if resolution_id < 5 {
            // Use red for FAIL
            color_id = 1;
        } else if resolution_id > 50 {
            // Use white most of the time
            color_id = 15;
        }
        let color_func = match color_id {
            1 ... 2 => Paint::red,
            3 ... 4 => Paint::green,
            5 ... 6 => Paint::cyan,
            7 ... 10 => Paint::blue,
            _ => Paint::white,
        };

        // Prepare and color the messages
        let simcity_msg = color_func(simcity.to_string());
        let resolution_msg = color_func(resolution.to_string());
        let dots = color_func("... ".to_string());
        let unchecked_checkbox = "[ ] ";
        let checked_checkbox = "[o] ";

        // Keep track of when the message is first printed
        let mut first = true;
        for i in 0..spinner_loops {
            for spinner in SPINNERS {
                // Output a message, with a checkbox in front and spinner behind
                let msg = simcity_msg.to_string() + "... " + spinner;
                if first {
                    dprint(unchecked_checkbox.to_string(), 0);
                    dprint(msg, TEXT_SLEEP);
                    first = false;
                } else {
                    dprint(unchecked_checkbox.to_string(), 0);
                    dprint(msg, 0);
                }
                // Wait a bit, then erase the line
                csleep(SPINNER_SLEEP);
                dprint("\r", 0);
            }
            if i == (spinner_loops - 1) {
                // End of loop, the line has been removed, conclude the status
                dprint(checked_checkbox.to_string(), 10);
                dprint(simcity_msg.to_string(), 0);
                dprint(dots.to_string(), 0);
                dprint(resolution_msg.to_string(), 0);
            }
        }

        if appconfig.should_exit() {
            dprint("\nALL DONE\n", 0);
            return;
        }

        println!();
    }
}
