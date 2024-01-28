/*
 * EpiRust
 * Copyright (c) 2023  ThoughtWorks, Inc.
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

use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

use crate::log_file;

pub struct FileLogger;

impl FileLogger {
    pub fn init(engine_id: String, output_dir_path: &Path) -> Result<(), Box<dyn Error>> {
        let temp_yaml_path = format!("/tmp/log4rs-{}-epirust.yaml", engine_id);
        let mut file = File::create(&temp_yaml_path)?;
        log_file::get_log4rs_yaml(
            format!("{}/logs/{}.log", output_dir_path.to_str().unwrap().to_string(), engine_id),
            &mut file,
        )?;
        log4rs::init_file(&temp_yaml_path, Default::default())?;
        fs::remove_file(temp_yaml_path)?;
        Ok(())
    }
}
