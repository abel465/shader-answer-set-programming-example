use clingo::*;
use egui_winit::winit::dpi::PhysicalSize;

pub fn solve(size: PhysicalSize<u32>) {
    let arguments = vec![
        "-n".into(),
        "0".into(),
        "-c".into(),
        format!("num_rows={}", size.width),
        "-c".into(),
        format!("num_cols={}", size.height),
    ];
    let mut ctl = control(arguments).unwrap();

    ctl.add("base", &[], include_str!("logic_program.lp"))
        .expect("Failed to add a logic program.");

    ctl.ground(&vec![Part::new("base", vec![]).unwrap()])
        .expect("Failed to ground a logic program.");

    let mut handle = ctl.solve(SolveMode::YIELD, &vec![]).unwrap();
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        match handle.model() {
            Ok(Some(model)) => {
                let type_string = match model.model_type().unwrap() {
                    ModelType::StableModel => "Stable model",
                    ModelType::BraveConsequences => "Brave consequences",
                    ModelType::CautiousConsequences => "Cautious consequences",
                };

                print!("{} {}:", type_string, model.number().unwrap());

                let atoms = model
                    .symbols(ShowType::SHOWN)
                    .expect("Failed to retrieve symbols in the model.");

                for symbol in atoms {
                    print!(" {}", symbol);
                }
                println!();
            }
            Ok(None) => break,
            Err(e) => panic!("Error: {}", e),
        }
    }
}
