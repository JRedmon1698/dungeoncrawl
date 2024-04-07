use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            let offset = Point::new(camera.left_x, camera.top_y);

            renderables
                .iter(ecs)
                .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
                .for_each(|(pos, _)| {});
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });
    draw_batch.submit(5000).expect("Batch error: entity_render");
}
