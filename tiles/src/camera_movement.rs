use amethyst::{
    input::{InputHandler, StringBindings},
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Read, System, SystemData, ReadStorage, WriteStorage, Join}, 
    renderer::Camera
};

#[derive(SystemDesc)]
pub struct CameraMovement;

impl<'s> System<'s> for CameraMovement {
    type SystemData = (
        ReadStorage<'s, Camera>, 
        WriteStorage<'s, Transform>, 
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (cameras, mut transforms, input) : Self::SystemData) {
        for (camera, transform) in (&cameras, &mut transforms).join() {
            
            let hor = input.axis_value("horizontal").unwrap_or(0.0);
            let forw = input.axis_value("forward").unwrap_or(0.0);
            let ver = input.axis_value("vertical").unwrap_or(0.0);

            transform.move_up(ver*2.0);
            transform.move_forward(forw*2.0);
            transform.move_right(hor*2.0);

            let rot_x = input.axis_value("rotation_x").unwrap_or(0.0);
            let rot_y = input.axis_value("rotation_y").unwrap_or(0.0);

            transform.append_rotation_x_axis(rot_x*0.05);
            transform.append_rotation_y_axis(rot_y*0.05);
        }
    }
}