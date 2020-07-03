use crate::error::Result;
use crate::lafi;
use crate::laheader::LazParams;
use crate::lareader;
use crate::lawriter;

/// Pipeline to process laz data from input to filtered output
pub struct LazPipeline {
    pub reader: Option<lareader::LazReader>,
    pub writer: Option<lawriter::LazWriter>,
    pub num_points: usize,
    filters: Vec<Box<dyn lafi::LazFilter>>,
}

impl LazPipeline {
    /// Creates pipeline for file processing
    ///
    /// * `input_file`: File with input laz data
    /// * `output_file`: File to write processed data to
    /// * `compress`: True for compressed output (laz), false for uncompressed (las)
    pub fn from_file(
        input_file: String,
        output_file: String,
        compress: bool,
    ) -> Result<LazPipeline> {
        crate::lautil::load_laszip_library();

        let reader = lareader::LazReader::from_file(&input_file)?;
        let header = reader.get_header()?;
        let (scale, offset) = (header.scale(), header.offset());
        let writer = Some(lawriter::LazWriter::from_file(
            &output_file,
            compress,
            &scale,
            &offset,
        )?);

        let num_points = if (*header).number_of_point_records == 0 {
            (*header).extended_number_of_point_records
        } else {
            (*header).number_of_point_records as u64
        };

        Ok(LazPipeline {
            reader: Some(reader),
            writer,
            num_points: num_points as usize,
            filters: Vec::default(),
        })
    }

    /// Creates pipeline for in-memory processing
    ///
    /// * `buffer`: Input laz data
    /// * `compress`: True for compressed output (laz), false for uncompressed (las)
    pub fn from_vec(buffer: Vec<u8>, compress: bool) -> Result<LazPipeline> {
        crate::lautil::load_laszip_library();

        let reader = lareader::LazReader::from_vec(buffer)?;
        let header = reader.get_header()?;
        let writer = Some(lawriter::LazWriter::from_vec(
            4096usize,
            compress,
            &header.scale(),
            &header.offset(),
        )?);

        let num_points = if (*header).number_of_point_records == 0 {
            (*header).extended_number_of_point_records
        } else {
            (*header).number_of_point_records as u64
        };

        Ok(LazPipeline {
            reader: Some(reader),
            writer,
            num_points: num_points as usize,
            filters: Vec::default(),
        })
    }

    pub fn add_filter(&mut self, filter: Box<dyn lafi::LazFilter>) {
        self.filters.push(filter);
    }

    pub fn run(&mut self) -> Result<()> {
        let filters = &self.filters;
        if self.reader.is_some() && self.writer.is_some() {
            let points = self.reader.as_ref().unwrap().iter_points()?;
            for p in points.filter(|p| filters.iter().all(|f| f.apply(p.as_ref().unwrap()))) {
                if let Ok(p) = p {
                    self.writer.as_mut().unwrap().push(p)?;
                }
            }
        }
        Ok(())
    }
}
