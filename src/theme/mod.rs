use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tera::Tera;
use utils::write_file;
use errors::{Error, Result};

static SIMPLE_FAVICON: &'static [u8] = include_bytes!("simple/static/favicon.png");
static SIMPLE_LOGO: &'static [u8] = include_bytes!("simple/static/logo.png");
static SIMPLE_MAIN_CSS: &'static [u8] = include_bytes!("simple/static/main.css");
static SIMPLE_MAIN_JS: &'static [u8] = include_bytes!("simple/static/main.js");
static SIMPLE_BASE: &'static [u8] = include_bytes!("simple/templates/base.tpl");
static SIMPLE_INDEX: &'static [u8] = include_bytes!("simple/templates/index.tpl");
static SIMPLE_POST: &'static [u8] = include_bytes!("simple/templates/post.tpl");
static SIMPLE_TAG: &'static [u8] = include_bytes!("simple/templates/tag.tpl");

/// theme object
#[derive(Default)]
pub struct Theme {
    /// theme root directory
    pub root: PathBuf,
    /// theme name
    pub name: String,
    /// theme renderer
    pub renderer: Tera,
    favicon: Vec<u8>,
    logo: Vec<u8>,
    main_css: Vec<u8>,
    main_js: Vec<u8>,
    base: Vec<u8>,
    index: Vec<u8>,
    post: Vec<u8>,
    tag: Vec<u8>,
}

impl Theme {
    pub fn new<P: AsRef<Path>>(root: P, name: &str) -> Result<Theme> {
        debug!("loading theme: {}", &name);
        let root = root.as_ref();
        let mut theme = Theme {
            root: root.to_owned(),
            name: name.to_string(),
            renderer: Tera::default(),
            ..Default::default()
        };
        let src_dir = root.join(name);
        if !src_dir.exists() {
            if name != "simple" {
                return Err(Error::ThemeNotFound(name.to_string()));
            }
            theme.favicon.extend_from_slice(&SIMPLE_FAVICON);
            theme.logo.extend_from_slice(&SIMPLE_LOGO);
            theme.main_css.extend_from_slice(&SIMPLE_MAIN_CSS);
            theme.main_js.extend_from_slice(&SIMPLE_MAIN_JS);
            theme.base.extend_from_slice(&SIMPLE_BASE);
            theme.index.extend_from_slice(&SIMPLE_INDEX);
            theme.post.extend_from_slice(&SIMPLE_POST);
            theme.tag.extend_from_slice(&SIMPLE_TAG);
            theme.init_template()?;
            return Ok(theme);
        }

        let mut favicon_file = File::open(src_dir.join("static/favicon.png"))?;
        let mut logo_file = File::open(src_dir.join("static/logo.png"))?;
        let mut main_css_file = File::open(src_dir.join("static/main.css"))?;
        let mut main_js_file = File::open(src_dir.join("static/main.js"))?;
        let mut base_file = File::open(src_dir.join("templates/base.tpl"))?;
        let mut index_file = File::open(src_dir.join("templates/index.tpl"))?;
        let mut post_file = File::open(src_dir.join("templates/post.tpl"))?;
        let mut tag_file = File::open(src_dir.join("templates/tag.tpl"))?;
        favicon_file.read_to_end(&mut theme.favicon)?;
        logo_file.read_to_end(&mut theme.logo)?;
        main_css_file.read_to_end(&mut theme.main_css)?;
        main_js_file.read_to_end(&mut theme.main_js)?;
        base_file.read_to_end(&mut theme.base)?;
        index_file.read_to_end(&mut theme.index)?;
        post_file.read_to_end(&mut theme.post)?;
        tag_file.read_to_end(&mut theme.tag)?;
        theme.init_template()?;
        return Ok(theme);
    }

    fn init_template(&mut self) -> Result<()> {
        self.renderer.add_raw_template("base.tpl", ::std::str::from_utf8(&self.base)?)?;
        self.renderer.add_raw_template("index.tpl", ::std::str::from_utf8(&self.index)?)?;
        self.renderer.add_raw_template("post.tpl", ::std::str::from_utf8(&self.post)?)?;
        self.renderer.add_raw_template("tag.tpl", ::std::str::from_utf8(&self.tag)?)?;
        Ok(())
    }

    pub fn init_dir(&self, name: &str) -> Result<()> {
        let dest_dir = self.root.join(name);
        if dest_dir.exists() {
            info!("theme({}) already existed", name);
            return Ok(());
        }
        debug!("init theme({}) ...", name);
        write_file(&dest_dir.join("static/favicon.png"), &self.favicon)?;
        write_file(&dest_dir.join("static/logo.png"), &self.logo)?;
        write_file(&dest_dir.join("static/main.css"), &self.main_css)?;
        write_file(&dest_dir.join("static/main.js"), &self.main_js)?;
        write_file(&dest_dir.join("templates/base.tpl"), &self.base)?;
        write_file(&dest_dir.join("templates/index.tpl"), &self.index)?;
        write_file(&dest_dir.join("templates/post.tpl"), &self.post)?;
        write_file(&dest_dir.join("templates/tag.tpl"), &self.tag)?;
        Ok(())
    }

    pub fn export_static<P: AsRef<Path>>(&self, root: P) -> Result<()> {
        debug!("exporting theme({}) static ...", self.name);
        let dest_dir = root.as_ref();
        write_file(&dest_dir.join("static/favicon.png"), &self.favicon)?;
        write_file(&dest_dir.join("static/logo.png"), &self.logo)?;
        write_file(&dest_dir.join("static/main.css"), &self.main_css)?;
        write_file(&dest_dir.join("static/main.js"), &self.main_js)?;
        Ok(())
    }
}
