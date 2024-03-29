use std::{rc::Rc, cell::RefCell};

use rltk::GameState;
use super::super::global::global::Global;
pub struct InitProps<'a>
{
    pub ctx: &'a mut rltk::BTerm,
    pub global : Rc<RefCell<Global>>
}

pub struct TickProps<'a>
{
    pub ctx: &'a mut rltk::BTerm,
    pub global : Rc<RefCell<Global>>
}

pub struct QuitProps<'a>
{
    pub ctx: &'a mut rltk::BTerm,
    pub global : Rc<RefCell<Global>>
}

pub trait RgState { 
    // This will be called when ui first pushed
    fn on_init(&mut self,props:&InitProps) -> bool;

    fn on_tick(&mut self,props:&TickProps);

    // This will be called when ui pop
    fn on_quit(&mut self,quit:&QuitProps);

}

