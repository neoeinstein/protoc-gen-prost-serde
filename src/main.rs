use prost::Message;
use prost_types::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use protoc_gen_prost::generators::{Generator, GeneratorPipeline};
use protoc_gen_prost::{CodeGeneratorResult, ModuleRequestSet};
use protoc_gen_prost_serde::generator::PbJsonGenerator;
use protoc_gen_prost_serde::Parameters;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;

    let response = inner(buf.as_slice()).unwrap_into_response();

    buf.clear();
    response.encode(&mut buf).expect("error encoding response");
    io::stdout().write_all(&buf)?;

    Ok(())
}

fn inner(
    raw_request: &[u8],
) -> Result<CodeGeneratorResponse, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let request = CodeGeneratorRequest::decode(raw_request)?;
    let params = request.parameter().parse::<Parameters>()?;

    let mut builder = params.to_pbjson_builder();
    for file in &request.proto_file {
        builder.register_file_descriptor(file.clone());
    }

    let module_request_set = ModuleRequestSet::new(
        request.file_to_generate,
        request.proto_file,
        raw_request,
        params.default_package_filename(),
    )?;

    let mut generator = PbJsonGenerator::new(builder);

    let pipeline: [&mut dyn Generator; 1] = [&mut generator];

    let response = pipeline
        .into_iter()
        .collect_code_generator_response(&module_request_set);

    Ok(response)
}
