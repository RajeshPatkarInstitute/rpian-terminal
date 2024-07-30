use rpian_terminal::{
    clear_screen,
    rbox::{draw_shaded_rectangle, ShadeStyle},
    set_viewport,
};

fn main() -> std::io::Result<()> {
    set_viewport(80, 24);
    clear_screen()?;

    //Draw rectangles with different shade styles
    draw_shaded_rectangle(5, 5, 20, 5, ShadeStyle::Light)?;
    draw_shaded_rectangle(30, 5, 20, 5, ShadeStyle::Medium)?;
    draw_shaded_rectangle(5, 15, 20, 5, ShadeStyle::Dark)?;
    draw_shaded_rectangle(30, 15, 20, 5, ShadeStyle::Solid)?;

    //move_cursor_to(1, 25)?;
    Ok(())
}
