use super::parser;
use std::fs;
use std::io::Read;
use std::path::Path;

pub struct Generator {
    src_path: Box<Path>,
    out_path: Box<Path>,
    tl_files: Vec<Box<Path>>,
}

impl Generator {
    pub fn new(src_path: &str, out_path: &str) -> Generator {
        let mut g = Generator {
            src_path: Box::from(Path::new(src_path)),
            out_path: Box::from(Path::new(out_path)),
            tl_files: vec![],
        };
        g.init();

        return g;
    }

    pub fn init(&mut self) {
        self.scan();
        fs::create_dir_all(self.out_path.as_ref()).unwrap();
    }

    pub fn generate(&self) {
        println!("src_path: {:?}", self.src_path);
        println!("out_path: {:?}", self.out_path);

        if self.tl_files.len() > 0 {
            fs::File::create(self.out_path.join("mod.rs")).expect("file create issue");
        }

        for file in &self.tl_files {
            self.generate_file(file);
        }
    }

    pub fn generate_file(&self, file: &Path) {
        // let file_name = file
        //     .file_stem()
        //     .expect("file stem have issue")
        //     .to_str()
        //     .unwrap()
        //     .to_owned()
        //     + ".rs";
        println!("generating file for: {:?}", file.file_name().unwrap());
        // fs::File::create(self.out_path.join(file_name)).expect("file create issue");

        let mut input = String::new();
        fs::File::open(file)
            .expect(
                format!(
                    "Unable to open file for reading: {}",
                    file.to_string_lossy()
                )
                .as_str(),
            )
            .read_to_string(&mut input)
            .expect(format!("Unable to read file contents: {}", file.to_string_lossy()).as_str());

        let res = parser::parse_string(input.as_str());
        println!("{:?}", res);
    }

    pub fn scan(&mut self) {
        let mut files = fs::read_dir(&self.src_path)
            .expect(format!("Unable to read directory contents: {:?}", self.src_path).as_str())
            .filter_map(Result::ok)
            .map(|d| d.path())
            .filter(|path| path.to_str().unwrap().ends_with(".tl"))
            .map(|path| path.into_boxed_path())
            .collect::<Vec<Box<Path>>>();

        files.sort();

        self.tl_files = files;
    }
}
