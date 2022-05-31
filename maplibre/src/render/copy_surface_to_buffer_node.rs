use crate::render::graph::{Node, NodeRunError, RenderContext, RenderGraphContext, SlotInfo};
use crate::render::render_commands::{DrawMasks, DrawTiles};
use crate::render::render_phase::{PhaseItem, RenderCommand};
use crate::render::resource::{Head, TrackedRenderPass};
use crate::render::util::FloatOrd;
use crate::render::Eventually::Initialized;
use crate::render::RenderState;
use std::ops::{Deref, Range};

#[derive(Default)]
pub struct CopySurfaceBufferNode {}

impl CopySurfaceBufferNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Node for CopySurfaceBufferNode {
    fn input(&self) -> Vec<SlotInfo> {
        vec![]
    }

    fn update(&mut self, _state: &mut RenderState) {}

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        RenderContext {
            command_encoder, ..
        }: &mut RenderContext,
        state: &RenderState,
    ) -> Result<(), NodeRunError> {
        match state.surface.head() {
            Head::Headed(_) => {}
            Head::Headless(buffered_texture) => {
                let size = state.surface.size();
                command_encoder.copy_texture_to_buffer(
                    buffered_texture.texture.as_image_copy(),
                    wgpu::ImageCopyBuffer {
                        buffer: &buffered_texture.output_buffer,
                        layout: wgpu::ImageDataLayout {
                            offset: 0,
                            bytes_per_row: Some(
                                std::num::NonZeroU32::new(
                                    buffered_texture.buffer_dimensions.padded_bytes_per_row as u32,
                                )
                                .unwrap(),
                            ),
                            rows_per_image: None,
                        },
                    },
                    wgpu::Extent3d {
                        width: size.width() as u32,
                        height: size.height() as u32,
                        depth_or_array_layers: 1,
                    },
                );
            }
        }

        Ok(())
    }
}
