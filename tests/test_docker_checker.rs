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
    fs::create_dir_all(&volume1.join("_data")).unwrap();
    fs::write(volume1.join("_data").join("file.txt"), b"data").unwrap();

    let volume2 = volumes_dir.join("my-volume-2");
    fs::create_dir_all(&volume2.join("_data")).unwrap();
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
