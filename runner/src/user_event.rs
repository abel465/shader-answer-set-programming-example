use crate::app::Graphics;
use crate::shader::CompiledShaderModules;

pub enum UserEvent {
    NewModule(CompiledShaderModules),
    SetVSync(bool),
    CreateWindow(Graphics),
}
