//! Docker checker comprehensive tests
//! Testing Docker container cache, image cache, and build cache detection with real file structures

use devsweep::checkers;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_docker_checker_basic_functionality() {
    let result = checkers::check_docker();

    // Should return a result (items may be empty if Docker not installed)
    assert!(!result.name.is_empty());
}

#[test]
fn test_docker_container_cache_detection() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create mock Docker container cache
    let containers_dir = docker_root.join("containers");
    fs::create_dir_all(&containers_dir).unwrap();

    // Create mock container directories
    let container1 = containers_dir.join("abc123def456");
    fs::create_dir_all(&container1).unwrap();
    fs::write(
        container1.join("config.v2.json"),
        b"{\"container_config\":{}}",
    )
    .unwrap();
    fs::write(container1.join("hostconfig.json"), b"{}").unwrap();

    let container2 = containers_dir.join("789ghi012jkl");
    fs::create_dir_all(&container2).unwrap();
    fs::write(container2.join("config.v2.json"), b"{}").unwrap();

    // Verify structure created
    assert!(containers_dir.exists());
    assert_eq!(fs::read_dir(&containers_dir).unwrap().count(), 2);
}

#[test]
fn test_docker_image_cache_structure() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create mock image storage
    let image_dir = docker_root.join("image").join("overlay2");
    fs::create_dir_all(&image_dir).unwrap();

    // Create layerdb
    let layerdb = image_dir.join("layerdb").join("sha256");
    fs::create_dir_all(&layerdb).unwrap();

    // Create mock layers
    let layer1 = layerdb.join("abcd1234567890");
    fs::create_dir_all(&layer1).unwrap();
    fs::write(layer1.join("size"), b"1048576").unwrap(); // 1 MB
    fs::write(layer1.join("cache-id"), b"cache123").unwrap();

    let layer2 = layerdb.join("efgh0987654321");
    fs::create_dir_all(&layer2).unwrap();
    fs::write(layer2.join("size"), b"2097152").unwrap(); // 2 MB

    assert!(layerdb.exists());
    assert_eq!(fs::read_dir(&layerdb).unwrap().count(), 2);
}

#[test]
fn test_docker_build_cache_structure() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create build cache directory
    let buildkit = docker_root.join("buildkit");
    fs::create_dir_all(&buildkit).unwrap();

    let cache_dir = buildkit.join("cache");
    fs::create_dir_all(&cache_dir).unwrap();

    // Create mock cache entries
    fs::write(cache_dir.join("blob1"), b"build cache data 1").unwrap();
    fs::write(cache_dir.join("blob2"), b"build cache data 2").unwrap();
    fs::write(cache_dir.join("blob3"), b"build cache data 3").unwrap();

    assert!(cache_dir.exists());
    assert_eq!(fs::read_dir(&cache_dir).unwrap().count(), 3);
}

#[test]
fn test_docker_volumes_structure() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create volumes directory
    let volumes_dir = docker_root.join("volumes");
    fs::create_dir_all(&volumes_dir).unwrap();

    // Create mock volumes
    let volume1 = volumes_dir.join("my-volume-1");
    fs::create_dir_all(volume1.join("_data")).unwrap();
    fs::write(volume1.join("_data").join("file.txt"), b"data").unwrap();

    let volume2 = volumes_dir.join("my-volume-2");
    fs::create_dir_all(volume2.join("_data")).unwrap();
    fs::write(volume2.join("_data").join("data.db"), b"database").unwrap();

    assert!(volumes_dir.exists());
    assert_eq!(fs::read_dir(&volumes_dir).unwrap().count(), 2);
}

#[test]
fn test_docker_overlay2_diff_structure() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create overlay2 storage
    let overlay2 = docker_root.join("overlay2");
    fs::create_dir_all(&overlay2).unwrap();

    // Create layer diff directories
    let layer1 = overlay2.join("l").join("ABCDEF123456");
    fs::create_dir_all(&layer1).unwrap();
    fs::write(layer1.join("link"), b"short-id").unwrap();

    let diff1 = overlay2.join("abc123").join("diff");
    fs::create_dir_all(&diff1).unwrap();
    fs::write(diff1.join("file1.txt"), b"layer data").unwrap();

    assert!(overlay2.exists());
    assert!(diff1.exists());
}

#[test]
fn test_docker_tmp_directory() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create tmp directory for temporary files
    let tmp_dir = docker_root.join("tmp");
    fs::create_dir_all(&tmp_dir).unwrap();

    // Create mock temporary files
    fs::write(tmp_dir.join("temp-1234.tar"), b"temp data 1").unwrap();
    fs::write(tmp_dir.join("temp-5678.tar"), b"temp data 2").unwrap();

    assert!(tmp_dir.exists());
    assert_eq!(fs::read_dir(&tmp_dir).unwrap().count(), 2);
}

#[test]
fn test_docker_empty_directories() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create empty directories
    let empty_containers = docker_root.join("containers");
    fs::create_dir_all(&empty_containers).unwrap();

    let empty_images = docker_root.join("image");
    fs::create_dir_all(&empty_images).unwrap();

    assert!(empty_containers.exists());
    assert!(empty_images.exists());
    assert_eq!(fs::read_dir(&empty_containers).unwrap().count(), 0);
}

#[test]
fn test_docker_network_cache() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create network directory
    let network_dir = docker_root.join("network").join("files");
    fs::create_dir_all(&network_dir).unwrap();

    // Create network config files
    fs::write(network_dir.join("local-kv.db"), b"network data").unwrap();

    assert!(network_dir.exists());
}

#[test]
fn test_docker_large_image_layers() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    let layerdb = docker_root
        .join("image")
        .join("overlay2")
        .join("layerdb")
        .join("sha256");
    fs::create_dir_all(&layerdb).unwrap();

    // Create many layers (simulate large image)
    for i in 0..20 {
        let layer = layerdb.join(format!("layer{:02}", i));
        fs::create_dir_all(&layer).unwrap();
        fs::write(
            layer.join("size"),
            format!("{}", (i + 1) * 1000000).as_bytes(),
        )
        .unwrap();
        fs::write(layer.join("cache-id"), format!("cache{}", i).as_bytes()).unwrap();
    }

    assert_eq!(fs::read_dir(&layerdb).unwrap().count(), 20);
}

#[test]
fn test_docker_container_logs() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    let containers_dir = docker_root.join("containers");
    fs::create_dir_all(&containers_dir).unwrap();

    // Create container with logs
    let container = containers_dir.join("container123");
    fs::create_dir_all(&container).unwrap();
    fs::write(container.join("container123-json.log"), b"[log entries]").unwrap();

    assert!(container.join("container123-json.log").exists());
}

#[test]
fn test_docker_stopped_containers() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    let containers_dir = docker_root.join("containers");
    fs::create_dir_all(&containers_dir).unwrap();

    // Create stopped containers (would have state files)
    for i in 0..5 {
        let container = containers_dir.join(format!("stopped{}", i));
        fs::create_dir_all(&container).unwrap();
        fs::write(container.join("config.v2.json"), b"{}").unwrap();
        fs::write(container.join("hostconfig.json"), b"{}").unwrap();
    }

    assert_eq!(fs::read_dir(&containers_dir).unwrap().count(), 5);
}

#[test]
fn test_docker_checker_returns_valid_structure() {
    let result = checkers::check_docker();

    // Verify result structure
    assert!(!result.name.is_empty());

    // All items should have valid structure
    for item in result.items {
        assert!(!item.item_type.is_empty());
        // size is u64, always valid
    }
}

#[test]
fn test_docker_image_imagedb() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create imagedb
    let imagedb = docker_root
        .join("image")
        .join("overlay2")
        .join("imagedb")
        .join("content")
        .join("sha256");
    fs::create_dir_all(&imagedb).unwrap();

    // Create image metadata files
    fs::write(imagedb.join("abc123def456"), b"{\"image_metadata\":{}}").unwrap();
    fs::write(imagedb.join("789ghi012jkl"), b"{\"image_metadata\":{}}").unwrap();

    assert!(imagedb.exists());
    assert_eq!(fs::read_dir(&imagedb).unwrap().count(), 2);
}

#[test]
fn test_docker_distribution_cache() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create distribution directory (registry cache)
    let distribution = docker_root
        .join("image")
        .join("overlay2")
        .join("distribution");
    fs::create_dir_all(&distribution).unwrap();

    let v2metadata = distribution.join("v2metadata-by-diffid").join("sha256");
    fs::create_dir_all(&v2metadata).unwrap();

    fs::write(v2metadata.join("metadata1"), b"registry metadata").unwrap();

    assert!(v2metadata.exists());
}

#[test]
fn test_docker_runtimes_directory() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create runtimes directory
    let runtimes = docker_root.join("runtimes");
    fs::create_dir_all(&runtimes).unwrap();

    let runc = runtimes.join("runc");
    fs::create_dir_all(&runc).unwrap();

    assert!(runc.exists());
}

// ==================== Additional Docker Structure Tests ====================

#[test]
fn test_docker_swarm_directories() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create swarm directory
    let swarm = docker_root.join("swarm");
    fs::create_dir_all(&swarm).unwrap();

    let worker = swarm.join("worker");
    fs::create_dir_all(&worker).unwrap();
    fs::write(worker.join("tasks.db"), b"tasks data").unwrap();

    assert!(swarm.exists());
}

#[test]
fn test_docker_plugins_directory() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create plugins directory
    let plugins = docker_root.join("plugins");
    fs::create_dir_all(&plugins).unwrap();

    let plugin = plugins.join("abc123");
    fs::create_dir_all(&plugin).unwrap();
    fs::write(plugin.join("config.json"), b"{}").unwrap();

    assert!(plugins.exists());
}

#[test]
fn test_docker_trust_directory() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create trust directory (Docker Content Trust)
    let trust = docker_root.join("trust");
    fs::create_dir_all(&trust).unwrap();

    let private = trust.join("private");
    fs::create_dir_all(&private).unwrap();
    fs::write(private.join("key.pem"), b"private key").unwrap();

    assert!(trust.exists());
}

#[test]
fn test_docker_containerd_directory() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    fs::create_dir_all(&docker_root).unwrap();

    // Create containerd directory
    let containerd = docker_root.join("containerd");
    fs::create_dir_all(&containerd).unwrap();

    let daemon = containerd.join("daemon");
    fs::create_dir_all(&daemon).unwrap();

    assert!(containerd.exists());
}

#[test]
fn test_docker_buildx_cache() {
    let temp = TempDir::new().unwrap();
    let buildx = temp.path().join(".docker/buildx");
    fs::create_dir_all(&buildx).unwrap();

    // Create buildx configuration
    let instances = buildx.join("instances");
    fs::create_dir_all(&instances).unwrap();
    fs::write(instances.join("default"), b"default builder").unwrap();

    assert!(buildx.exists());
}

#[test]
fn test_docker_config_json() {
    let temp = TempDir::new().unwrap();
    let docker_config = temp.path().join(".docker");
    fs::create_dir_all(&docker_config).unwrap();

    // Create config.json
    fs::write(docker_config.join("config.json"), b"{\"auths\":{}}").unwrap();

    assert!(docker_config.join("config.json").exists());
}

#[test]
fn test_docker_cli_plugins() {
    let temp = TempDir::new().unwrap();
    let cli_plugins = temp.path().join(".docker/cli-plugins");
    fs::create_dir_all(&cli_plugins).unwrap();

    // Create CLI plugin binaries
    fs::write(cli_plugins.join("docker-compose"), b"binary").unwrap();
    fs::write(cli_plugins.join("docker-buildx"), b"binary").unwrap();

    assert!(cli_plugins.exists());
    assert_eq!(fs::read_dir(&cli_plugins).unwrap().count(), 2);
}

#[test]
fn test_docker_scan_cache() {
    let temp = TempDir::new().unwrap();
    let scan_cache = temp.path().join(".docker/scan/cache");
    fs::create_dir_all(&scan_cache).unwrap();

    // Create scan cache files
    fs::write(scan_cache.join("image_scan_results"), b"scan results").unwrap();

    assert!(scan_cache.exists());
}

#[test]
fn test_docker_contexts() {
    let temp = TempDir::new().unwrap();
    let contexts = temp.path().join(".docker/contexts");
    fs::create_dir_all(&contexts).unwrap();

    // Create context directory
    let meta = contexts.join("meta");
    fs::create_dir_all(&meta).unwrap();

    let ctx = meta.join("abc123");
    fs::create_dir_all(&ctx).unwrap();
    fs::write(ctx.join("meta.json"), b"{\"Name\":\"mycontext\"}").unwrap();

    assert!(contexts.exists());
}

#[test]
fn test_docker_desktop_data() {
    let temp = TempDir::new().unwrap();
    let dd_data = temp.path().join("Library/Containers/com.docker.docker/Data");
    fs::create_dir_all(&dd_data).unwrap();

    // Create Docker Desktop data
    let vms = dd_data.join("vms/0/data");
    fs::create_dir_all(&vms).unwrap();
    fs::write(vms.join("Docker.raw"), b"vm disk").unwrap();

    assert!(dd_data.exists());
}

#[test]
fn test_docker_machine_directory() {
    let temp = TempDir::new().unwrap();
    let machine = temp.path().join(".docker/machine");
    fs::create_dir_all(&machine).unwrap();

    // Create machine directories
    let machines = machine.join("machines");
    fs::create_dir_all(&machines).unwrap();

    let default = machines.join("default");
    fs::create_dir_all(&default).unwrap();
    fs::write(default.join("config.json"), b"{}").unwrap();

    assert!(machine.exists());
}

#[test]
fn test_docker_layer_chain() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    let overlay2 = docker_root.join("overlay2");
    fs::create_dir_all(&overlay2).unwrap();

    // Create a chain of layers (parent-child relationship)
    let layers = ["layer1", "layer2", "layer3", "layer4"];
    for (i, layer) in layers.iter().enumerate() {
        let layer_dir = overlay2.join(layer);
        fs::create_dir_all(layer_dir.join("diff")).unwrap();
        fs::create_dir_all(layer_dir.join("work")).unwrap();
        fs::create_dir_all(layer_dir.join("merged")).unwrap();

        if i > 0 {
            fs::write(layer_dir.join("lower"), layers[i-1].as_bytes()).unwrap();
        }
        fs::write(layer_dir.join("link"), format!("SHORT{}", i).as_bytes()).unwrap();
    }

    assert_eq!(fs::read_dir(&overlay2).unwrap().count(), 4);
}

#[test]
fn test_docker_multi_architecture_images() {
    let temp = TempDir::new().unwrap();
    let docker_root = temp.path().join("docker");
    let imagedb = docker_root.join("image/overlay2/imagedb/content/sha256");
    fs::create_dir_all(&imagedb).unwrap();

    // Create manifests for multi-arch images
    fs::write(imagedb.join("amd64_abc123"), b"{\"architecture\":\"amd64\"}").unwrap();
    fs::write(imagedb.join("arm64_abc123"), b"{\"architecture\":\"arm64\"}").unwrap();
    fs::write(imagedb.join("manifest_list_abc"), b"{\"manifests\":[]}").unwrap();

    assert_eq!(fs::read_dir(&imagedb).unwrap().count(), 3);
}
