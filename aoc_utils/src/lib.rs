use std::path::Path;
use std::fs::{ self, File };
use std::io::Write;

pub fn get_input_for_day(is_test: bool) -> String {
    let file_name = file!();
    let day = file_name
        .split('/')
        .last()
        .unwrap()
        .strip_prefix("day")
        .unwrap()
        .strip_suffix(".rs")
        .unwrap();

    let year = env!("CARGO_PKG_NAME").strip_prefix("year_").unwrap();

    get_input(year, day, is_test)
}

fn get_input(year: &str, day: &str, is_test: bool) -> String {
    let base_path = format!("year_{}/src/data{}", year, if is_test { "/test" } else { "" });
    let filename = format!("{}/day{}{}_input.txt", base_path, day, if is_test {
        "_test"
    } else {
        ""
    });

    fs::read_to_string(&filename).unwrap_or_else(|_|
        panic!("Unable to read input file: {}", filename)
    )
}

pub fn create_day_files(year: &str, day: u32) -> std::io::Result<()> {
    let day = format!("{:02}", day);
    let package_name = format!("year_{}", year);
    let year_project_path = format!("./{}", package_name);

    if !Path::new(&year_project_path).exists() {
        return Err(
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Project for year {} does not exist", year)
            )
        );
    }

    fs::create_dir_all(format!("{}/src/data/test", year_project_path))?;

    let template_path = "templates/day_template.rs";
    let template = fs
        ::read_to_string(&template_path)
        .map_err(|_|
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Template file not found in workspace root/templates/"
            )
        )?;

    // Solution file
    let rs_path = format!("{}/src/bin/day{}.rs", year_project_path, day);
    if !Path::new(&rs_path).exists() {
        let mut file = File::create(&rs_path)?;
        file.write_all(template.as_bytes())?;
        println!("Created {}", rs_path);
    }

    // Input files
    let input_path = format!("{}/src/data/day{}_input.txt", year_project_path, day);
    let test_input_path = format!("{}/src/data/test/day{}_test_input.txt", year_project_path, day);

    if !Path::new(&input_path).exists() {
        File::create(&input_path)?;
        println!("Created {}", input_path);
    }

    if !Path::new(&test_input_path).exists() {
        File::create(&test_input_path)?;
        println!("Created {}", test_input_path);
    }

    Ok(())
}
