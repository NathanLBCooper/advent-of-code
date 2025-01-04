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

    let pair = find_first_bad_pair(global_id, SkipIndex(false, 0u));
    if (!pair.is_some) {
        return 1u;
    }

    // left element to blame
    let next_bad_pair_with_skip_x = find_first_bad_pair(global_id, SkipIndex(true, pair.value.x));
    if (!next_bad_pair_with_skip_x.is_some) {
        return 1u;
    }

    // right element to blame
    let next_bad_pair_with_skip_y = find_first_bad_pair(global_id, SkipIndex(true, pair.value.y));
    if (!next_bad_pair_with_skip_y.is_some) {
        return 1u;
    }

    // maybe the direction (set by the 0th and 1st element) was wrong
    let next_bad_pair_with_skip_first = find_first_bad_pair(global_id, SkipIndex(true, 0u));
    if (!next_bad_pair_with_skip_first.is_some) {
        return 1u;
    }

    return 0u;
}

struct BadLevelPair {
    is_some: bool,
    value: vec2<u32>
}

struct SkipIndex {
    is_some: bool,
    value: u32
}

fn find_first_bad_pair(global_id: vec3<u32>, skip: SkipIndex) -> BadLevelPair {
    let report_metadata = report_metadata_buffer[global_id.x];

    const gentle_max = 3;
    const gentle_min = 1;
    var increasing_gently = true;
    var decreasing_gently = true;

    // todo this skip stuff is awful
    var start = 0u;
    if (skip.is_some && skip.value == 0) {
        start = 1u;
    }

    for (var i: u32 = start + 1; i < report_metadata.length; i = i + 1) {
        var last_i = i - 1;
        if (skip.is_some) {
            if (skip.value == i) {
                continue;
            }
            if (skip.value == i - 1) {
                last_i = last_i - 1;
            }
        }

        let last_level = flattened_level_buffer[report_metadata.offset + last_i];
        let level = flattened_level_buffer[report_metadata.offset + i];

        let diff = i32(level) - i32(last_level);

        if (diff < gentle_min || diff > gentle_max) {
            increasing_gently = false;
        }
        if (diff < -gentle_max || diff > -gentle_min) {
            decreasing_gently = false;
        }

        if (!increasing_gently && !decreasing_gently) {
            return BadLevelPair(true, vec2<u32>(last_i, i));
        }
    }

    return BadLevelPair(false, vec2<u32>(0,0));
}

// Part 1
//fn do_some_compute(global_id: vec3<u32>) -> u32 {
//    let report_metadata = report_metadata_buffer[global_id.x];
//
//    if (report_metadata.length < 2) {
//        return 1u;
//    }
//
//    const gentle_max = 3;
//    const gentle_min = 1;
//    var increasing_gently = true;
//    var decreasing_gently = true;
//
//    for (var i: u32 = 1; i < report_metadata.length; i = i + 1) {
//        let last_level = flattened_level_buffer[report_metadata.offset + i - 1];
//        let level = flattened_level_buffer[report_metadata.offset + i];
//
//        let diff = i32(level) - i32(last_level);
//
//        if (diff < gentle_min || diff > gentle_max) {
//            increasing_gently = false;
//        }
//        if (diff < -gentle_max || diff > -gentle_min) {
//            decreasing_gently = false;
//        }
//
//        if (!increasing_gently && !decreasing_gently) {
//            return 0u;
//        }
//    }
//
//    return 1u;
//}

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    output_buffer[global_id.x] = do_some_compute(global_id);
}