//! Access to the file-analyzing service
//! ([fileinfo](https://retdec.com/api/docs/fileinfo.html)).

use analysis::Analysis;
use analysis::AnalysisArguments;
use connection::APIArguments;
use connection::APIConnectionFactory;
use connection::HyperAPIConnectionFactory;
use error::Result;
use error::ResultExt;
use settings::Settings;

/// File-analyzing service.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
///
/// use retdec::analysis::AnalysisArguments;
/// use retdec::fileinfo::Fileinfo;
/// use retdec::settings::Settings;
///
/// let settings = Settings::new()
///     .with_api_key("MY-API-KEY");
/// let fileinfo = Fileinfo::new(settings);
/// let args = AnalysisArguments::new()
///     .with_input_file(Path::new(&"file.exe").to_path_buf());
/// let mut analysis = fileinfo.start_analysis(&args).unwrap();
/// analysis.wait_until_finished().unwrap();
/// let output = analysis.get_output().unwrap();
/// print!("{}", output);
/// ```
pub struct Fileinfo {
    conn_factory: Box<APIConnectionFactory>,
}

impl Fileinfo {
    /// Creates a new instance of the file-analyzing service.
    pub fn new(settings: Settings) -> Self {
        Fileinfo {
            conn_factory: Box::new(HyperAPIConnectionFactory::new(settings)),
        }
    }

    /// Starts a new file analysis with the given arguments.
    pub fn start_analysis(&self, args: &AnalysisArguments) -> Result<Analysis> {
        let mut conn = self.conn_factory.new_connection();
        let url = format!("{}/fileinfo/analyses", conn.api_url());
        let api_args = self.create_api_args(args)?;
        let response = conn.send_post_request(&url, &api_args)
            .chain_err(|| "failed to start an analysis")?;
        let content = response.body_as_json()?;
        let id = content["id"].as_str().unwrap();
        Ok(Analysis::new(id, conn))
    }

    fn create_api_args(&self, args: &AnalysisArguments) -> Result<APIArguments> {
        let mut api_args = APIArguments::new();
        api_args.add_opt_string_arg("output_format", args.output_format());
        api_args.add_opt_bool_arg("verbose", args.verbose());
        match args.input_file() {
            Some(ref input_file) => {
                api_args.add_file("input", input_file);
            },
            None => {
                bail!("no input file given");
            },
        }
        Ok(api_args)
    }
}
