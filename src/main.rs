pub mod engine;

fn main() -> Result<(), engine::EngineError>
{
    let colors = engine::color::Colors::detect_set_colors();
    engine::cli::print_logo(&colors);
    engine::cli::print_categories(&colors);
    let mut scenarios = engine::scenario::get_scenarios(engine::cli::get_list_type(), &colors);

    for (i,scenario) in scenarios.iter().enumerate() {
        engine::rendering::cli::print_cli_select(i, &scenario.name, &colors);
    }
    let scenario_index = engine::cli::get_scenario_index(scenarios.len(), &colors);
    engine::runtime::run(&mut scenarios[scenario_index])?;
    Ok(())
}
