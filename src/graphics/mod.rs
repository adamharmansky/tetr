pub use gl33::GlFns;
use glam::*;

use std::rc::Rc;

pub mod handle;
pub mod model;
pub mod shader;
pub mod texture;

pub use handle::GraphicsHandle;
pub use model::Model;
pub use shader::Shader;
pub use texture::Texture;
