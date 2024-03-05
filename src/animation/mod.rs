mod gof;
mod mir;

pub trait Animation {
    fn new() -> Self;

    fn update(&mut self) {
        // for each frame count the number of characters that moved
        // by divising by the total number of characters we have the percentage of characters that moved
        // if this percentage is inferior to a certain threshold, automatically stop the simulation
    }

    fn compute_next_frame(&mut self);
    fn render_frame(&self) {}
}
