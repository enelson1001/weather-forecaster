/// FileStore for reading and writing to SD Card.
/// NOTE: For ESP-IDF v5.2 and later
///
use esp_idf_svc::hal::{
    gpio::{AnyIOPin, AnyInputPin, AnyOutputPin},
    sd::{spi::SdSpiHostDriver, SdCardConfiguration, SdCardDriver},
    spi::{config::DriverConfig, Dma, SpiAnyPins, SpiDriver},
};

use esp_idf_svc::fs::fatfs::Fatfs;
use esp_idf_svc::io::vfs::MountedFatfs;

use anyhow::Result;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub struct FileStore {
    _mounted_fatfs: MountedFatfs<Fatfs<SdCardDriver<SdSpiHostDriver<'static, SpiDriver<'static>>>>>,
}

impl FileStore {
    pub fn init(
        spi: impl SpiAnyPins + 'static,
        sclk: AnyOutputPin<'static>,
        sdo: AnyOutputPin<'static>,
        sdi: AnyInputPin<'static>,
        cs: AnyOutputPin<'static>,
    ) -> Result<Self> {
        let spi_driver = SpiDriver::new(
            spi,
            sclk,
            sdo,
            Some(sdi),
            &DriverConfig::default().dma(Dma::Auto(4096)),
        )?;

        let sd_spi_host_driver = SdSpiHostDriver::new(
            spi_driver,
            Some(cs),
            AnyIOPin::none(), // cd
            AnyIOPin::none(), // wp
            AnyIOPin::none(), // int
            None,             // wp_active_high -  For ESP-IDF v5.2 and later
        )?;

        let sd_card_driver =
            SdCardDriver::new_spi(sd_spi_host_driver, &SdCardConfiguration::new())?;

        let mounted_fatfs =
            MountedFatfs::mount(Fatfs::new_sdcard(0, sd_card_driver)?, "/sdcard", 4)?;

        Ok(Self {
            // Keep it around or else it will be dropped and unmounted
            _mounted_fatfs: mounted_fatfs,
        })
    }

    pub fn read_lines_from_file(&mut self, file_name: &str) -> Result<Vec<String>> {
        let file_path = format!("/sdcard/{}", file_name);
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);
        let mut lines_vec: Vec<String> = Vec::new();

        for line_result in reader.lines() {
            let line = line_result?;
            lines_vec.push(line);
        }

        Ok(lines_vec)
    }

    pub fn write_lines_to_file(&mut self, file_name: &str, lines: &str) -> Result<()> {
        let file_path = format!("/sdcard/{}", file_name);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // overwrite the file
            .open(&file_path)?;
        file.write_all(lines.as_bytes())?;

        Ok(())
    }
}
