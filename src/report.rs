use configuration::ReportParameters;
use std::fs::File;

use handlebars::Handlebars;
use input_data::InputData;
use report_data::make_report_data;
use std::error::Error;

pub fn generate_report(
    output_file: &str,
    input_data: &InputData,
    report_params: &ReportParameters,
) -> Result<(), Box<Error>> {
    let data = make_report_data(&input_data, &report_params);

    let mut reg = Handlebars::new();
    reg.register_template_string("main", include_str!("templates/main.hbs"))?;
    reg.register_template_string("area_chart", include_str!("templates/area_chart.hbs"))?;
    reg.register_template_string("table", include_str!("templates/table.hbs"))?;
    reg.render_to_write("main", &data, File::create(output_file)?)?;

    Ok(())
}
