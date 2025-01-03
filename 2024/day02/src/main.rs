use std::num::ParseIntError;
use pollster::FutureExt;
use wgpu::util::DeviceExt;
use common::core::AdventError;
use common::file::read_lines;

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct ReportMetaData {
    offset: u32,
    length: u32,
}

#[derive(Debug)]
struct Report {
    levels: Vec<u32>,
}

fn main() {
    let reports = match parse_file() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e.message());
            std::process::exit(1)
        }
    };

    let (flattened_levels, report_metadata) = flatten_reports(reports);

    use pollster::FutureExt as _;
    // todo should be result, not option
    let result = match execute_gpu(&flattened_levels, &report_metadata).block_on() {
        Some(s) => s,
        None => {
            eprintln!("Failed to execute GPU computation.");
            std::process::exit(1)
        }
    };

    let sum_of_safe_reports = result.iter().sum::<u32>();

    println!("Part 1 Sum: {:?}", sum_of_safe_reports);
}

fn flatten_reports(reports: Vec<Report>) -> (Vec<u32>, Vec<ReportMetaData>) {
    let mut levels = Vec::new();
    let mut metadata = Vec::new();

    let mut offset = 0;
    for report in reports {
        let length = report.levels.len();
        metadata.push(ReportMetaData { offset: offset as u32, length: length as u32 });

        levels.extend(report.levels.iter());
        offset += length;
    }

    (levels, metadata)
}

fn parse_file() -> Result<Vec<Report>, AdventError> {
    let lines = read_lines("./day02/input.txt")?;

    match lines.iter().map(|l| parse_line(l)).collect::<Result<Vec<Report>, ParseIntError>>() {
        Ok(s) => Ok(s),
        Err(e) => Err(AdventError::ParseIntError { inner: e }),
    }
}

fn parse_line(line: &str) -> Result<Report, ParseIntError> {
    let number_strs: Vec<&str> = line.split_whitespace().collect();

    let numbers = number_strs
        .iter()
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()?;

    Ok(Report { levels: numbers })
}

async fn execute_gpu(
    flattened_levels: &[u32],
    report_metadata: &[ReportMetaData],
) -> Option<Vec<u32>> {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await?;
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
            },
            None,
        )
        .await
        .ok()?;

    let module = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

    let flattened_levels_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Flattened Levels Buffer"),
        contents: bytemuck::cast_slice(flattened_levels),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });

    let report_metadata_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Report Metadata Buffer"),
        contents: bytemuck::cast_slice(report_metadata),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });

    let output_buffer_size =
        (report_metadata.len() * std::mem::size_of::<u32>()) as wgpu::BufferAddress;
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: output_buffer_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging Buffer"),
        size: output_buffer_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &module,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });

    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: flattened_levels_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: report_metadata_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: output_buffer.as_entire_binding(),
            },
        ],
    });

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let workgroups_x = report_metadata.len() as u32;

        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("Run my compute shader");
        cpass.dispatch_workgroups(workgroups_x, 1, 1);
    }

    encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_buffer, 0, output_buffer_size);

    queue.submit(Some(encoder.finish()));

    let buffer_slice = staging_buffer.slice(..);

    let (sender, receiver) = flume::bounded(1);
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    device.poll(wgpu::Maintain::wait()).panic_on_timeout();

    if let Ok(Ok(())) = receiver.recv_async().await {
        let data = buffer_slice.get_mapped_range();
        let result = bytemuck::cast_slice(&data).to_vec();

        drop(data);
        staging_buffer.unmap();

        Some(result)
    } else {
        panic!("Failed to run compute on GPU!");
    }
}
