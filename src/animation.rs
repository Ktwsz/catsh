use std::time::Duration;
use std::ops::{Add, Sub, Mul};

pub struct Interpolator<Item> 
    where Item: Add<Output=Item> + Sub<Output=Item> + Mul<f64, Output=Item> + Copy + Clone
{
    interpolator: fn(f64) -> f64,
    initial_value: Item,
    final_value: Item,
}

impl<Item> Interpolator<Item> where
    Item: Add<Output=Item> + Sub<Output=Item> + Mul<f64, Output=Item> + Copy + Clone
{
    fn get(&self, dt: f64) -> Item {
        self.initial_value + (self.final_value - self.initial_value) * (self.interpolator)(dt)
    }

    pub fn linear(initial_value: Item, final_value: Item) -> Interpolator<Item> {
        Interpolator {
            initial_value, final_value,
            interpolator: linear_interpolation,
        }
    }
}

pub fn linear_interpolation(dt: f64) -> f64 {
    dt
}

pub struct Animation<Item>
    where Item: Add<Output=Item> + Sub<Output=Item> + Mul<f64, Output=Item> + Copy + Clone
{
    duration: f64,
    current_dt: f64,
    interpolator: Interpolator<Item>,
}

impl<Item> Animation<Item>
    where Item: Add<Output=Item> + Sub<Output=Item> + Mul<f64, Output=Item> + Copy + Clone
{
    pub fn new(duration: Duration, interpolator: Interpolator<Item>) -> Animation<Item> {
        Animation {
            duration: duration.as_millis() as f64 / 1000.0,
            current_dt: 0.0,
            interpolator,
        }
    }
}


pub struct AnimateProperty <Item>
    where Item: Add<Output=Item> + Sub<Output=Item> + Mul<f64, Output=Item> + Copy + Clone
{
    pub property: Item,
    animation: Option<Animation<Item>>,
}

impl<Item> AnimateProperty<Item>
    where Item: Add<Output=Item> + Sub<Output=Item> + Mul<f64, Output=Item> + Copy + Clone
{
    pub fn new(property: Item) -> AnimateProperty<Item> {
        AnimateProperty {
            property,
            animation: None,
        }
    }

    pub fn set_animation(&mut self, duration: Duration, interpolator: Interpolator<Item>) {
        self.animation = Some(Animation::new(duration, interpolator));
    }


    pub fn update(&mut self, dt: Duration) {
        match &mut self.animation {
            Some(animation) => {
                animation.current_dt += dt.as_millis() as f64 / 1000.0;
                self.property = self.property + animation.interpolator.get(animation.current_dt / animation.duration);
            },
            None => (),
        };
    }
}

