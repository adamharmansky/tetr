use std::collections::HashMap;
use std::io::prelude::*;
use std::rc::Rc;

#[derive(Clone)]
pub enum Resource {
    Image(Rc<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>),
    Text(Rc<String>),
    Binary(Rc<Vec<u8>>),
}

pub struct ResourceManager {
    #[allow(unused)]
    /// The directory containing all the resources
    directory: String,
    resources: HashMap<String, Resource>,
}

impl ResourceManager {
    pub fn new(directory: String) -> Result<Self, Box<dyn std::error::Error>> {
        let resources = std::fs::read_dir(&directory)?
            .filter(|x| match x {
                Ok(x) => x.file_type().unwrap().is_file(),
                Err(_) => true,
            })
            .map::<Result<(String, Resource), Box<dyn std::error::Error>>, _>(|x| {
                let x = x?;
                let name = x.file_name().into_string().expect("invalid filename UTF-8");
                Ok((
                    name.clone(),
                    Resource::load(format!("{}/{}", directory, name))?,
                ))
            })
            .collect::<Result<HashMap<String, Resource>, _>>()?;
        Ok(Self {
            directory,
            resources,
        })
    }

    pub fn get_resource(&self, name: String) -> Option<Resource> {
        match self.resources.get(&name) {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }

    pub fn get_image(&self, name: &str) -> Rc<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
        match self
            .get_resource(String::from(name))
            .expect(&format!("Unable to locate resource: {}", name))
        {
            Resource::Image(x) => x,
            _ => panic!("wrong resource type for texture"),
        }
    }

    pub fn get_text(&self, name: &str) -> Rc<String> {
        match self
            .get_resource(String::from(name))
            .expect(&format!("Unable to locate resource: {}", name))
        {
            Resource::Text(x) => x,
            _ => panic!("wrong resource type for text"),
        }
    }

    pub fn get_binary(&self, name: &str) -> Rc<Vec<u8>> {
        match self
            .get_resource(String::from(name))
            .expect(&format!("Unable to locate resource: {}", name))
        {
            Resource::Binary(x) => x,
            _ => panic!("wrong resource type for text"),
        }
    }
}

impl Resource {
    pub fn load(file: String) -> Result<Self, Box<dyn std::error::Error>> {
        match file.split('.').last().expect("no file extension") {
            "png" => Ok(Resource::Image(Rc::new(
                image::io::Reader::new(std::io::BufReader::new(std::fs::File::open(file)?))
                    .with_guessed_format()?
                    .decode()?
                    .into_rgba8(),
            ))),
            "vert" | "frag" | "txt" => Ok(Resource::Text({
                let mut file = std::fs::File::open(file)?;
                let mut v = Vec::<u8>::new();
                file.read_to_end(&mut v)?;
                Rc::new(String::from_utf8(v)?)
            })),
            _ => Ok(Resource::Binary({
                let mut file = std::fs::File::open(file)?;
                let mut v = Vec::<u8>::new();
                file.read_to_end(&mut v)?;
                Rc::new(v)
            })),
        }
    }
}
