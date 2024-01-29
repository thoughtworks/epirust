/*
 * EpiRust
 * Copyright (c) 2024  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use serde_yaml::{from_str, to_string, Value};

fn read_yaml_file(file_path: PathBuf) -> Result<Value, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let value: Value = from_str(&content)?;

    Ok(value)
}

fn write_yaml_file(file: &mut File, data: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let yaml_string = to_string(data)?;

    file.write_all(yaml_string.as_bytes())?;

    Ok(())
}

pub fn get_log4rs_yaml(output_dir_path: String, temp_conf_file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    //Todo: Fix this bcoz it forces you to run epirust from project root folder
    let path = env::current_dir()?;
    let yaml_file_path = path.join("engine-app/config/log4rs.yaml");

    // Read the YAML file
    let mut yaml_data = read_yaml_file(yaml_file_path).unwrap();

    // Modify the "appenders.file.path" field at runtime
    if let Some(appenders) = yaml_data["appenders"].as_mapping_mut() {
        if let Some(file) = appenders.get_mut(&Value::String("file".to_string())).unwrap().as_mapping_mut() {
            if let Some(path) = file.get_mut(&Value::String("path".to_string())) {
                *path = Value::String(output_dir_path);
            }
        }
    }

    // Create a temporary file

    // Write the modified YAML data to the temporary file
    write_yaml_file(temp_conf_file, &yaml_data)?;

    // Get the path of the temporary file

    Ok(())
}
