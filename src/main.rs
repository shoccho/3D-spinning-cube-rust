const CUBE_WIDTH: usize = 20;

//screen
const WIDTH: usize = 160;
const HEIGHT: usize = 44;

const HORIZONTAL_OFFSET: isize = -2 * CUBE_WIDTH as isize;
use cube::cube_renderer::CubeRenderer;
use cube::renderer::Renderer;

fn main() {
    let renderer = Renderer::new(HEIGHT, WIDTH, HORIZONTAL_OFFSET);
    let mut cube_renderer = CubeRenderer::new(CUBE_WIDTH, renderer, 10);
    cube_renderer.render();
}
