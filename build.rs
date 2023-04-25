#[cfg(not(feature = "compile-shaders"))]
fn main() {}

#[cfg(feature = "compile-shaders")]
fn main() -> std::io::Result<()> {
    use std::io::*;

    let dir = "src/window/reng/shaders";

    let compiler = shaderc::Compiler::new().unwrap();

    let mut options = shaderc::CompileOptions::new().unwrap();

    options.set_warnings_as_errors();

    options.set_optimization_level(shaderc::OptimizationLevel::Performance);

    options.set_include_callback(|source_req, _, _, _| {
        let full_req = format!("{}/{}", dir, source_req);

        let mut content = String::new();

        std::fs::File::open(std::path::Path::new(&full_req))
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();

        Ok(shaderc::ResolvedInclude {
            resolved_name: full_req,
            content,
        })
    });

    for maybe_file in std::fs::read_dir(dir).unwrap() {
        let file = maybe_file.unwrap();

        let pathbuf = file.path();

        let path = pathbuf.as_path();

        let extension = path.extension().unwrap().to_str().unwrap();

        let kind = {
            if extension == "vert" {
                shaderc::ShaderKind::Vertex
            } else if extension == "frag" {
                shaderc::ShaderKind::Fragment
            } else if extension == "comp" {
                shaderc::ShaderKind::Compute
            } else {
                continue;
            }
        };

        let mut new_shader_name = String::from(path.to_str().unwrap());
        new_shader_name.push_str(".spv");

        let mut new_shader = std::fs::File::create(new_shader_name).unwrap();

        let source = std::fs::read_to_string(path)?;

        let spirv = compiler
            .compile_into_spirv(
                source.as_str(),
                kind,
                path.to_str().unwrap(),
                "main",
                Some(&options),
            )
            .map_err(|e| println!("{}", e))
            .expect("");

        new_shader.write(spirv.as_binary_u8()).unwrap();
    }
    Ok(())
}
