struct ReportMetaData {
    offset: u32,
    length: u32
}

@group(0) @binding(0)
var<storage, read> flattened_level_buffer: array<u32>;
@group(0) @binding(1)
var<storage, read> report_metadata_buffer: array<ReportMetaData>;
@group(0) @binding(2)
var<storage, read_write> output_buffer: array<u32>;

fn do_some_compute(global_id: vec3<u32>) -> u32 {
    let report_metadata = report_metadata_buffer[global_id.x];

    if (report_metadata.length < 2) {
        return 1u;
    }

    const gentle_max = 3;
    const gentle_min = 1;
    var increasing_gently = true;
    var decreasing_gently = true;

    for (var i: u32 = 1; i < report_metadata.length; i = i + 1) {
        let last_level = flattened_level_buffer[report_metadata.offset + i - 1];
        let level = flattened_level_buffer[report_metadata.offset + i];

        let diff = i32(level) - i32(last_level);

        if (diff < gentle_min || diff > gentle_max) {
            increasing_gently = false;
        }
        if (diff < -gentle_max || diff > -gentle_min) {
            decreasing_gently = false;
        }

        if (!increasing_gently && !decreasing_gently) {
            return 0u;
        }
    }

    return 1u;
}

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    output_buffer[global_id.x] = do_some_compute(global_id);
}