
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Zip(zip::result::ZipError);
        DuckDb(duckdb::Error);
    }
}

fn main() -> Result<()> {
    // let url = "https://nfdc.faa.gov/webContent/28DaySub/2023-11-02/class_airspace_shape_files.zip";
    let url = "https://nfdc.faa.gov/webContent/28DaySub/extra/02_Nov_2023_CSV.zip";
    let zipped_name = "./download.zip";
    let unzipped_name = "./download_dir";
    download_file(url, zipped_name)?;
    unzip_file(unzipped_name, zipped_name)?;
    write_shapes_to_duck_db()?;
    let sql = get_csv_names("./download_dir")?
        .iter()
        .filter_map(|csv_name| make_table_sql(csv_name))
        .collect();
    write_tables_to_duck_db(sql)
}

fn download_file(url: &str, zipped_name: &str) -> Result<()> {
    let response = reqwest::blocking::get(url)?;
    println!("isSuccess?: {}", response.status().is_success());

    // TODO: Fix paths to use tempfile instead
    let path = Path::new(zipped_name);
    let mut file = File::create(path)?;
    let content = response.bytes()?;
    file.write_all(&content)?;
    println!("Wrote file");

    Ok(())
}

fn unzip_file(unzipped_name: &str, zipped_name: &str) -> Result<()> {
    let file = File::open(zipped_name)?;
    let unzipped_path = Path::new(unzipped_name);
    std::fs::create_dir(unzipped_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    println!("Created ZipArchive");

    archive.extract(unzipped_path)?;
    println!("Extracted");

    Ok(())
}

/// TODO: Duckdb searches for a cpg file for some reason?
fn write_shapes_to_duck_db() -> Result<()> {
    // TODO Remove hard-coded path
    let sql = "
    INSTALL spatial;
    LOAD spatial;
    CREATE TABLE zones AS
        SELECT
            zone,
            LocationId,
            borough,
            ST_GeomFromWKB(wkb_geometry) AS geom
        FROM ST_Read('./download_dir/Shape_Files/Class_Airspace.shp');";
    let conn = duckdb::Connection::open_in_memory()?;
    conn.execute_batch(sql)?;
    Ok(())
}

fn get_csv_names(dir_path: &str) -> Result<Vec<PathBuf>> {
    Ok(std::fs::read_dir(dir_path)?
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.path().extension().and_then(std::ffi::OsStr::to_str) == Some("csv"))
        .map(|entry| entry.path())
        .collect())
}

fn write_tables_to_duck_db(sql: String) -> Result<()> {
    let path = Path::new("./piflite.db");
    let conn = duckdb::Connection::open(path)?;
    conn.execute_batch(sql.as_str())?;
    Ok(())
}

fn make_table_sql(path: &Path) -> Option<String> {
    path.file_stem()
        .map(|stem| stem.to_ascii_lowercase())
        .map(|stem| {
            format!(
                "CREATE TABLE {:?} AS SELECT * FROM read_csv_auto('{}');",
                stem,
                path.display(), // FIXME
            )
        })
}
