use leptos::{ReadSignal, SignalGet, SignalUpdate, WriteSignal};

pub fn create_signals<T>() -> MainBodySignals<T>
where
    T: for<'a> HasReactivePosition + HidesReactively + HoldsTilePosition + Default,
{
    MainBodySignals::default()
}

pub struct MainBodySignals<T>
where
    T: for<'a> HasReactivePosition + HidesReactively + HoldsTilePosition + Default,
{
    pub ctx_menu: T,
    pub touch_menu: T,
}

impl<T> Default for MainBodySignals<T>
where
    T: for<'a> HasReactivePosition + HidesReactively + HoldsTilePosition + Default,
{
    fn default() -> Self {
        Self {
            ctx_menu: Default::default(),
            touch_menu: Default::default(),
        }
    }
}

pub struct ContextMenuSignals {
    position: (ReadSignal<ScreenCoords>, WriteSignal<ScreenCoords>),
    hidden: (ReadSignal<bool>, WriteSignal<bool>),
    tile: (ReadSignal<GridCoords>, WriteSignal<GridCoords>),
}

impl HasReactivePosition for ContextMenuSignals {
    fn get_position(&self) -> ScreenCoords {
        self.position.0.get()
    }

    fn set_position(&mut self, coords: ScreenCoords) -> &mut Self {
        self.position.1.update(|value| *value = coords);
        self
    }
}

impl HidesReactively for ContextMenuSignals {
    fn is_hidden(&self) -> bool {
        self.hidden.0.get()
    }

    fn hide(&mut self) -> &mut Self {
        self.hidden.1.update(|value| *value = true);
        self
    }

    fn show(&mut self) -> &mut Self {
        self.hidden.1.update(|value| *value = false);
        self
    }
}

impl HoldsTilePosition for ContextMenuSignals {
    fn get_tile(&self) -> Option<GridCoords> {
        Some(self.tile.0.get())
    }

    fn set_tile(&mut self, coords: GridCoords) -> &mut Self {
        self.tile.1.update(|value| *value = coords);
        self
    }
}

impl<'a> ContextMenuSignals {
    fn on_select(&'a mut self) -> impl FnMut(GridCoords, ScreenCoords) + 'a {
        move |(row, col), (x, y): ScreenCoords| {
            self.set_position((x, y)).show().set_tile((row, col));
        }
    }
}

pub struct TouchMenuSignals;

impl HasReactivePosition for TouchMenuSignals {
    fn get_position(&self) -> ScreenCoords {
        self.
    }

    fn set_position(&mut self, coords: ScreenCoords) -> &mut Self {
        todo!()
    }
}

pub trait HasReactivePosition {
    fn get_position(&self) -> ScreenCoords;
    fn set_position(&mut self, coords: ScreenCoords) -> &mut Self;
}

pub trait HidesReactively {
    fn is_hidden(&self) -> bool;
    fn hide(&mut self) -> &mut Self;
    fn show(&mut self) -> &mut Self;
}

pub trait HoldsTilePosition {
    fn get_tile(&self) -> Option<GridCoords>;
    fn set_tile(&mut self, coords: GridCoords) -> &mut Self;
}

pub type ScreenCoords = (i32, i32);
pub type GridCoords = (usize, usize);
