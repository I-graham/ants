pub trait TextureType: Sized + Clone + Copy {
	fn list() -> Vec<Self>;

	fn name(&self) -> &'static str;

	fn frame_count(&self) -> u32 {
		1
	}
}
