pub struct RenderData {
	pub uniform_buffer: wgpu::Buffer,
	pub uniform_bg: wgpu::BindGroup,
	pub instance_buffer: wgpu::Buffer,
	pub instance_bg: wgpu::BindGroup,
	pub instance_len: usize,
	pub instance_cap: usize,
	pub encoder: wgpu::CommandEncoder,
	pub staging_belt: wgpu::util::StagingBelt,
	pub texture_bg: wgpu::BindGroup,
	pub nearest_sampler: wgpu::Sampler,
	pub current_frame: Option<wgpu::SurfaceTexture>,
	pub cached_buffers: fnv::FnvHashMap<&'static str, (usize, wgpu::BindGroup, wgpu::Buffer)>,
}
