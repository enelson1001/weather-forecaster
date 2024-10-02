use anyhow::Result;

use esp_idf_svc::hal::{
    delay::Ets,
    gpio::{self, AnyOutputPin, Output, PinDriver},
    peripheral,
    spi::{SpiAnyPins, SpiConfig, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
    units::FromValueType,
};

use embedded_sdmmc::{Mode, VolumeIdx, VolumeManager};
use embedded_sdmmc::{SdCard, TimeSource, Timestamp};

pub type SDcard = SdCard<
    SpiDeviceDriver<'static, SpiDriver<'static>>,
    PinDriver<'static, AnyOutputPin, Output>,
    Ets,
>;

pub struct SdMmcClock;

impl TimeSource for SdMmcClock {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

pub struct FileStore {
    pub volume_mgr: VolumeManager<SDcard, SdMmcClock>,
}

impl FileStore {
    pub fn init(
        spi: impl peripheral::Peripheral<P = impl SpiAnyPins> + 'static,
        sclk: gpio::AnyOutputPin,
        sdo: gpio::AnyOutputPin,
        sdi: gpio::AnyInputPin,
        cs: gpio::AnyOutputPin,
    ) -> Result<Self> {
        let spi = SpiDeviceDriver::new_single(
            spi,
            sclk,
            sdo,
            Some(sdi),
            Option::<gpio::AnyIOPin>::None, // don't use chip select here
            &SpiDriverConfig::new(),
            &SpiConfig::new().baudrate(24.MHz().into()),
            //&SpiDriverConfig::new().dma(Dma::Auto(4096)),
            //&SpiConfig::new()
            //    .duplex(Duplex::Full)
            //    .baudrate(24.MHz().into()),
        )?;

        let sdcard_cs = PinDriver::output(cs)?;
        let sdcard = SdCard::new(spi, sdcard_cs, Ets);

        sdcard
            .num_bytes()
            .map_err(|e| anyhow::anyhow!("SdCard error: {:?}", e))?;

        let volume_mgr = VolumeManager::new(sdcard, SdMmcClock);

        Ok(Self { volume_mgr })
    }

    pub fn read_lines_from_file(&mut self, file_name: &str) -> Result<Vec<String>> {
        let mut volume = self
            .volume_mgr
            .open_volume(VolumeIdx(0))
            .map_err(|e| anyhow::anyhow!("SdCard error: {:?}", e))?;

        let mut root_dir = volume
            .open_root_dir()
            .map_err(|e| anyhow::anyhow!("SdCard error: {:?}", e))?;

        // Open the file located in the root directory
        let mut file = root_dir
            .open_file_in_dir(file_name, Mode::ReadOnly)
            .map_err(|e| anyhow::anyhow!("SdCard error: {:?}", e))?;

        let mut buffer: Vec<u8> = vec![0; 1024];
        let mut lines: Vec<String> = Vec::new();

        // read file
        while !file.is_eof() {
            let bytes_read = file
                .read(buffer.as_mut_slice())
                .map_err(|e| anyhow::anyhow!("SdCard error: {:?}", e))?;

            let buffer_str = std::str::from_utf8(&buffer.as_mut_slice()[0..bytes_read])?;
            lines = buffer_str.lines().map(String::from).collect();
        }

        Ok(lines)
    }

    pub fn write_lines_to_file(&mut self, file_name: &str, buffer: &[u8]) -> Result<()> {
        let mut volume = self
            .volume_mgr
            .open_volume(VolumeIdx(0))
            .map_err(|e| anyhow::anyhow!("SdCard error: {:?}", e))?;

        let mut root_dir = volume
            .open_root_dir()
            .map_err(|e| anyhow::anyhow!("SdCard error: {:?}", e))?;

        // First delete the existing file
        root_dir
            .delete_file_in_dir(file_name)
            .map_err(|e| anyhow::anyhow!("SdCard error: {:?}", e))?;

        // Next create a new file with same name
        let mut file = root_dir
            .open_file_in_dir(file_name, Mode::ReadWriteCreate)
            .map_err(|e| anyhow::anyhow!("SdCard error: {:?}", e))?;

        file.write(buffer)
            .map_err(|e| anyhow::anyhow!("SdCard error: {:?}", e))?;

        Ok(())
    }

    pub fn is_file_empty(&mut self, file_name: &str) -> Result<bool> {
        let mut volume = self
            .volume_mgr
            .open_volume(VolumeIdx(0))
            .map_err(|e| anyhow::anyhow!("SdCard error: {:?}", e))?;

        let mut root_dir = volume
            .open_root_dir()
            .map_err(|e| anyhow::anyhow!("SdCard error: {:?}", e))?;

        // Open the file located in the root directory
        let mut file = root_dir
            .open_file_in_dir(file_name, Mode::ReadOnly)
            .map_err(|e| anyhow::anyhow!("SdCard error: {:?}", e))?;

        let mut buffer: Vec<u8> = vec![0; 1024];
        let mut num_read: usize = 0;

        // read file
        while !file.is_eof() {
            num_read = file
                .read(buffer.as_mut_slice())
                .map_err(|e| anyhow::anyhow!("SdCard error: {:?}", e))?;
        }

        Ok(num_read == 0)
    }
}
