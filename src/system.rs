use crate::structs::{
    AudioInfo, BatteryInfo, CpuInfo, DiskInfo, DiskPartition, DisplayInfo, FilesInfo, GpuInfo,
    NetworkInfo, OsInfo, RamInfo,
};
use crate::utils::{
    get_desktop_directory, get_documents_directory, get_downloads_directory,
    get_entries_from_directory, get_variant_string, get_variant_value, paths_to_strings, query_wmi,
};
use cache_size;
use iana_time_zone;
use std::collections::HashMap;
use sysinfo;
use whoami;

pub fn get_cpu_info() -> Vec<CpuInfo> {
    let results = query_wmi("SELECT * FROM Win32_Processor");
    let mut cpu_info_list = Vec::new();

    for result in results {
        let cpu_info = CpuInfo {
            name: get_variant_value(result.get("Name")),
            manufacturer: get_variant_value(result.get("Manufacturer")),
            device_id: get_variant_value(result.get("DeviceID")),
            socket_designation: get_variant_value(result.get("SocketDesignation")),
            cores: get_variant_value(result.get("NumberOfCores")),
            threads: get_variant_value(result.get("NumberOfCores")),
            l1_cache_size: cache_size::l1_cache_size().unwrap_or(0) as u32,
            l2_cache_size: cache_size::l2_cache_size().unwrap_or(0) as u32,
            l3_cache_size: cache_size::l3_cache_size().unwrap_or(0) as u32,
            max_clock_speed: get_variant_value(result.get("MaxClockSpeed")),
            current_clock_speed: get_variant_value(result.get("CurrentClockSpeed")),
        };

        cpu_info_list.push(cpu_info);
    }

    cpu_info_list
}

pub fn get_gpu_info() -> Vec<GpuInfo> {
    let results = query_wmi("SELECT * FROM Win32_VideoController");
    let mut gpu_info_list = Vec::new();

    for result in results {
        let gpu_info: GpuInfo = GpuInfo {
            name: get_variant_value(result.get("Name")),
            adapder_dac_type: get_variant_value(result.get("AdapterDACType")),
            adapter_compatibility: get_variant_value(result.get("AdapterCompatibility")),
            caption: get_variant_value(result.get("Caption")),
            description: get_variant_value(result.get("Description")),
            device_id: get_variant_value(result.get("DeviceID")),
            video_mode_description: get_variant_value(result.get("VideoModeDescription")),
            video_processor: get_variant_value(result.get("VideoProcessor")),
            max_refresh_rate: get_variant_value(result.get("MaxRefreshRate")),
            adapter_ram: get_variant_value(result.get("AdapterRAM")),
            current_bits_per_pixel: get_variant_value(result.get("CurrentBitsPerPixel")),
            current_horizontal_resolution: get_variant_value(
                result.get("CurrentHorizontalResolution"),
            ),
            current_vertical_resolution: get_variant_value(result.get("CurrentVerticalResolution")),
            current_refresh_rate: get_variant_value(result.get("CurrentRefreshRate")),
            driver_version: get_variant_value(result.get("DriverVersion")),
        };

        gpu_info_list.push(gpu_info);
    }

    gpu_info_list
}

pub fn get_ram_info() -> Vec<RamInfo> {
    let results = query_wmi("SELECT * FROM Win32_PhysicalMemory");
    let mut ram_info_list = Vec::new();

    let sys = sysinfo::System::new_all();

    for result in results {
        let ram_info = RamInfo {
            total: sys.total_memory(),
            used: sys.used_memory(),
            configured_clock_speed: get_variant_value(result.get("ConfiguredClockSpeed")),
            speed: get_variant_value(result.get("Speed")),
            manufacturer: get_variant_value(result.get("Manufacturer")),
            name: get_variant_value(result.get("Name")),
            part_number: get_variant_value(result.get("PartNumber")),
        };

        ram_info_list.push(ram_info);
    }

    ram_info_list
}

pub fn get_disk_info() -> Vec<DiskInfo> {
    let results = query_wmi("SELECT * FROM Win32_DiskDrive");
    let mut disk_info_list = Vec::new();

    for result in results {
        let mut disk_info = DiskInfo {
            caption: get_variant_value(result.get("Caption")),
            description: get_variant_value(result.get("Description")),
            device_id: get_variant_value(result.get("DeviceID")),
            interface_type: get_variant_value(result.get("InterfaceType")),
            manufacturer: get_variant_value(result.get("Manufacturer")),
            media_type: get_variant_value(result.get("MediaType")),
            model: get_variant_value(result.get("Model")),
            name: get_variant_value(result.get("Name")),
            pnp_device_id: get_variant_value(result.get("PNPDeviceID")),
            serial_number: get_variant_value(result.get("SerialNumber")),
            size: get_variant_value(result.get("Size")),
            total_heads: get_variant_value(result.get("TotalHeads")),
            total_cylinders: get_variant_value(result.get("TotalCylinders")),
            tracks_per_cylinder: get_variant_value(result.get("TracksPerCylinder")),
            total_tracks: get_variant_value(result.get("TotalTracks")),
            sectors_per_track: get_variant_value(result.get("SectorsPerTrack")),
            total_sectors: get_variant_value(result.get("TotalSectors")),
            bytes_per_sector: get_variant_value(result.get("BytesPerSector")),
            partitions_count: get_variant_value(result.get("Partitions")),
            partitions: Vec::new(),
        };

        let device_id_str = match &disk_info.device_id.value {
            wmi::Variant::String(s) => s.clone(),
            _ => String::new(),
        };

        let partitions_query = format!(
            "ASSOCIATORS OF {{Win32_DiskDrive.DeviceID='{}'}} WHERE AssocClass=Win32_DiskDriveToDiskPartition",
            device_id_str
        );

        let partitions_results = query_wmi(&partitions_query);

        for partition_result in partitions_results {
            let partition = DiskPartition {
                block_size: get_variant_value(partition_result.get("BlockSize")),
                bootable: get_variant_value(partition_result.get("Bootable")),
                boot_partition: get_variant_value(partition_result.get("BootPartition")),
                caption: get_variant_value(partition_result.get("Caption")),
                description: get_variant_value(partition_result.get("Description")),
                device_id: get_variant_value(partition_result.get("DeviceID")),
                disk_index: get_variant_value(partition_result.get("DiskIndex")),
                name: get_variant_value(partition_result.get("Name")),
                primary_partition: get_variant_value(partition_result.get("PrimaryPartition")),
                size: get_variant_value(partition_result.get("Size")),
                starting_offset: get_variant_value(partition_result.get("StartingOffset")),
                partition_type: get_variant_value(partition_result.get("Type")),
            };

            disk_info.partitions.push(partition);
        }

        disk_info_list.push(disk_info);
    }

    disk_info_list
}

pub fn get_display_info() -> Vec<DisplayInfo> {
    let results = query_wmi("SELECT * FROM Win32_DesktopMonitor");
    let mut display_info_list = Vec::new();

    for result in results {
        let display_info = DisplayInfo {
            device_id: get_variant_value(result.get("DeviceID")),
            screen_height: get_variant_value(result.get("ScreenHeight")),
            screen_width: get_variant_value(result.get("ScreenWidth")),
            caption: get_variant_value(result.get("Caption")),
            description: get_variant_value(result.get("Description")),
            display_type: get_variant_value(result.get("DisplayType")),
            monitor_manufacturer: get_variant_value(result.get("MonitorManufacturer")),
            monitor_type: get_variant_value(result.get("MonitorType")),
            name: get_variant_value(result.get("Name")),
            pixels_per_x_logical_inch: get_variant_value(result.get("PixelsPerXLogicalInch")),
            pixels_per_y_logical_inch: get_variant_value(result.get("PixelsPerYLogicalInch")),
            status: get_variant_value(result.get("Status")),
        };

        display_info_list.push(display_info);
    }

    display_info_list
}

pub fn get_network_info() -> Vec<NetworkInfo> {
    let results = query_wmi("SELECT * FROM Win32_NetworkAdapter");
    let mut network_info_list = Vec::new();

    for result in results {
        let network_info = NetworkInfo {
            name: get_variant_value(result.get("Name")),
            adapter_type: get_variant_value(result.get("AdapterType")),
            caption: get_variant_value(result.get("Caption")),
            description: get_variant_value(result.get("Description")),
            mac_address: get_variant_value(result.get("MACAddress")),
            manufacturer: get_variant_value(result.get("Manufacturer")),
            physical_adapter: get_variant_value(result.get("PhysicalAdapter")),
            product_name: get_variant_value(result.get("ProductName")),
            speed: get_variant_value(result.get("Speed")),
        };

        network_info_list.push(network_info);
    }

    network_info_list
}

pub fn get_audio_info() -> Vec<AudioInfo> {
    let results = query_wmi("SELECT * FROM Win32_SoundDevice");
    let mut audio_info_list = Vec::new();

    for result in results {
        let audio_info = AudioInfo {
            name: get_variant_value(result.get("Name")),
            caption: get_variant_value(result.get("Caption")),
            description: get_variant_value(result.get("Description")),
            manufacturer: get_variant_value(result.get("Manufacturer")),
            product_name: get_variant_value(result.get("ProductName")),
        };

        audio_info_list.push(audio_info);
    }

    audio_info_list
}

pub fn get_os_info() -> OsInfo {
    let results = query_wmi("SELECT * FROM Win32_OperatingSystem");
    let default_map = &HashMap::new();
    let result = results.get(0).unwrap_or(default_map);

    let mut os_info = OsInfo {
        name: get_variant_value(result.get("Name")),
        version: get_variant_value(result.get("Version")),
        system_name: get_variant_value(result.get("CSName")),
        serial_number: get_variant_value(result.get("SerialNumber")),
        registerer_user: get_variant_value(result.get("RegisteredUser")),
        architecture: get_variant_value(result.get("OSArchitecture")),
        locale: get_variant_value(result.get("Locale")),
        languages: get_variant_value(result.get("MUILanguages")),
        environment_variables: get_environment_variables(),
        timezone: iana_time_zone::get_timezone().unwrap_or_default(),
        user_real_name: whoami::realname(),
        user_name: whoami::username(),
        battery: Vec::new(),
        is_laptop: false,
    };

    let battery_results = query_wmi("SELECT * FROM Win32_Battery");

    if battery_results.len() > 0 {
        os_info.is_laptop = true;
    }

    for battery_result in battery_results {
        let battery_info = BatteryInfo {
            name: get_variant_value(battery_result.get("Name")),
            status: get_variant_value(battery_result.get("Status")),
            caption: get_variant_value(battery_result.get("Caption")),
            description: get_variant_value(battery_result.get("Description")),
            device_id: get_variant_value(battery_result.get("DeviceID")),
            estimated_charge_remaining: get_variant_value(
                battery_result.get("EstimatedChargeRemaining"),
            ),
        };

        os_info.battery.push(battery_info);
    }

    os_info
}

fn get_environment_variables() -> Vec<HashMap<String, String>> {
    let results = query_wmi("SELECT * FROM Win32_Environment");
    let mut env_vars = Vec::new();

    for result in results {
        let env_var = get_variant_string(result.get("Name"));
        let env_val = get_variant_string(result.get("VariableValue"));

        let mut env_var_map = HashMap::new();
        env_var_map.insert(env_var, env_val);

        env_vars.push(env_var_map);
    }

    env_vars
}

pub fn get_files() -> FilesInfo {
    let desktop_dir = get_desktop_directory();
    let downloads_dir = get_downloads_directory();
    let documents_dir = get_documents_directory();

    let desktop_files = get_entries_from_directory(&desktop_dir);
    let downloads_files = get_entries_from_directory(&downloads_dir);
    let documents_files = get_entries_from_directory(&documents_dir);

    let desktop_files_list = paths_to_strings(desktop_files);
    let downloads_files_list = paths_to_strings(downloads_files);
    let documents_files_list = paths_to_strings(documents_files);

    let files_info = FilesInfo {
        desktop: desktop_files_list,
        downloads: downloads_files_list,
        documents: documents_files_list,
    };

    files_info
}
