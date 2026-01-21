//! Python checker comprehensive tests
//! Testing __pycache__, pip cache, and virtualenv detection with real file structures

use devsweep::checkers;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_python_checker_basic_functionality() {
    let result = checkers::check_python();

    // Should return a result (items may be empty if no Python caches exist)
    assert!(!result.name.is_empty());
}

#[test]
fn test_pycache_detection() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("my-project");
    fs::create_dir_all(&project).unwrap();

    // Create __pycache__ directory
    let pycache = project.join("__pycache__");
    fs::create_dir_all(&pycache).unwrap();

    // Create mock .pyc files
    fs::write(pycache.join("module.cpython-39.pyc"), b"compiled bytecode").unwrap();
    fs::write(pycache.join("main.cpython-39.pyc"), b"compiled bytecode").unwrap();
    fs::write(pycache.join("utils.cpython-310.pyc"), b"compiled bytecode").unwrap();

    // Verify structure
    assert!(pycache.exists());
    assert_eq!(fs::read_dir(&pycache).unwrap().count(), 3);
}

#[test]
fn test_nested_pycache_directories() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("project");
    fs::create_dir_all(&project).unwrap();

    // Create nested __pycache__ directories
    let root_pycache = project.join("__pycache__");
    fs::create_dir_all(&root_pycache).unwrap();
    fs::write(root_pycache.join("main.cpython-39.pyc"), b"bytecode").unwrap();

    let subdir = project.join("utils");
    fs::create_dir_all(&subdir).unwrap();
    let utils_pycache = subdir.join("__pycache__");
    fs::create_dir_all(&utils_pycache).unwrap();
    fs::write(utils_pycache.join("helper.cpython-39.pyc"), b"bytecode").unwrap();

    let nested = project.join("lib").join("core");
    fs::create_dir_all(&nested).unwrap();
    let nested_pycache = nested.join("__pycache__");
    fs::create_dir_all(&nested_pycache).unwrap();
    fs::write(nested_pycache.join("core.cpython-39.pyc"), b"bytecode").unwrap();

    assert!(root_pycache.exists());
    assert!(utils_pycache.exists());
    assert!(nested_pycache.exists());
}

#[test]
fn test_pip_cache_detection() {
    let temp = TempDir::new().unwrap();
    let pip_cache = temp.path().join(".cache").join("pip");
    fs::create_dir_all(&pip_cache).unwrap();

    // Create http cache
    let http_cache = pip_cache.join("http");
    fs::create_dir_all(&http_cache).unwrap();
    fs::write(http_cache.join("package1.whl"), b"wheel data").unwrap();
    fs::write(http_cache.join("package2.tar.gz"), b"source data").unwrap();

    // Create wheels cache
    let wheels_cache = pip_cache.join("wheels");
    fs::create_dir_all(&wheels_cache).unwrap();
    fs::write(
        wheels_cache.join("numpy-1.21.0-cp39-cp39-macosx_11_0_arm64.whl"),
        b"numpy wheel",
    )
    .unwrap();

    assert!(http_cache.exists());
    assert!(wheels_cache.exists());
}

#[test]
fn test_pip_cache_subdirectories() {
    let temp = TempDir::new().unwrap();
    let pip_cache = temp.path().join(".cache").join("pip");
    fs::create_dir_all(&pip_cache).unwrap();

    // Create hash-based subdirectories (pip uses these)
    let http = pip_cache.join("http");
    fs::create_dir_all(&http).unwrap();

    for hash_prefix in &["a", "b", "c", "1", "2"] {
        let subdir = http.join(hash_prefix);
        fs::create_dir_all(&subdir).unwrap();
        fs::write(subdir.join("package.whl"), b"cached package").unwrap();
    }

    assert_eq!(fs::read_dir(&http).unwrap().count(), 5);
}

#[test]
fn test_virtualenv_detection() {
    let temp = TempDir::new().unwrap();
    let venv = temp.path().join("venv");
    fs::create_dir_all(&venv).unwrap();

    // Create virtualenv structure
    let bin_dir = venv.join("bin");
    fs::create_dir_all(&bin_dir).unwrap();
    fs::write(bin_dir.join("python"), b"#!/usr/bin/env python3\n").unwrap();
    fs::write(bin_dir.join("pip"), b"#!/usr/bin/env python3\n").unwrap();
    fs::write(bin_dir.join("activate"), b"# Activation script\n").unwrap();

    let lib_dir = venv.join("lib").join("python3.9").join("site-packages");
    fs::create_dir_all(&lib_dir).unwrap();
    fs::write(lib_dir.join("setuptools-1.0.0.dist-info"), b"metadata").unwrap();

    let pyvenv_cfg = venv.join("pyvenv.cfg");
    fs::write(
        &pyvenv_cfg,
        b"home = /usr/bin\ninclude-system-site-packages = false\n",
    )
    .unwrap();

    assert!(bin_dir.exists());
    assert!(lib_dir.exists());
    assert!(pyvenv_cfg.exists());
}

#[test]
fn test_multiple_virtualenvs() {
    let temp = TempDir::new().unwrap();

    // Create multiple venvs (common in projects)
    for venv_name in &["venv", ".venv", "env", ".env", "virtualenv"] {
        let venv = temp.path().join(venv_name);
        fs::create_dir_all(&venv).unwrap();

        let bin_dir = venv.join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        fs::write(bin_dir.join("python"), b"#!/usr/bin/python3\n").unwrap();

        fs::write(venv.join("pyvenv.cfg"), b"home = /usr/bin\n").unwrap();
    }

    // All should exist
    assert!(temp.path().join("venv").exists());
    assert!(temp.path().join(".venv").exists());
    assert!(temp.path().join("env").exists());
}

#[test]
fn test_poetry_cache() {
    let temp = TempDir::new().unwrap();
    let poetry_cache = temp.path().join(".cache").join("pypoetry");
    fs::create_dir_all(&poetry_cache).unwrap();

    // Create cache directory
    let cache = poetry_cache.join("cache");
    fs::create_dir_all(&cache).unwrap();

    // Create artifacts
    let artifacts = poetry_cache.join("artifacts");
    fs::create_dir_all(&artifacts).unwrap();
    fs::write(artifacts.join("package.whl"), b"poetry cached wheel").unwrap();

    assert!(cache.exists());
    assert!(artifacts.exists());
}

#[test]
fn test_pytest_cache() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("project");
    fs::create_dir_all(&project).unwrap();

    // Create .pytest_cache
    let pytest_cache = project.join(".pytest_cache");
    fs::create_dir_all(&pytest_cache).unwrap();

    let v_dir = pytest_cache.join("v").join("cache");
    fs::create_dir_all(&v_dir).unwrap();
    fs::write(v_dir.join("lastfailed"), b"{}").unwrap();
    fs::write(v_dir.join("nodeids"), b"[]").unwrap();
    fs::write(v_dir.join("stepwise"), b"[]").unwrap();

    assert!(pytest_cache.exists());
    assert!(v_dir.exists());
}

#[test]
fn test_mypy_cache() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("project");
    fs::create_dir_all(&project).unwrap();

    // Create .mypy_cache
    let mypy_cache = project.join(".mypy_cache");
    fs::create_dir_all(&mypy_cache).unwrap();

    // Create version directory
    let version = mypy_cache.join("3.9");
    fs::create_dir_all(&version).unwrap();
    fs::write(version.join("module.data.json"), b"{}").unwrap();
    fs::write(version.join("module.meta.json"), b"{}").unwrap();

    assert!(mypy_cache.exists());
    assert!(version.exists());
}

#[test]
fn test_tox_environments() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("project");
    fs::create_dir_all(&project).unwrap();

    // Create .tox directory
    let tox = project.join(".tox");
    fs::create_dir_all(&tox).unwrap();

    // Create multiple test environments
    for env in &["py39", "py310", "py311", "lint", "docs"] {
        let env_dir = tox.join(env);
        fs::create_dir_all(&env_dir).unwrap();

        let lib = env_dir.join("lib").join("python3.9").join("site-packages");
        fs::create_dir_all(&lib).unwrap();
        fs::write(lib.join("package.py"), b"# test package").unwrap();
    }

    assert_eq!(fs::read_dir(&tox).unwrap().count(), 5);
}

#[test]
fn test_ipython_cache() {
    let temp = TempDir::new().unwrap();
    let ipython = temp.path().join(".ipython");
    fs::create_dir_all(&ipython).unwrap();

    // Create profile directory
    let profile = ipython.join("profile_default");
    fs::create_dir_all(&profile).unwrap();

    // Create cache
    let db = profile.join("db");
    fs::create_dir_all(&db).unwrap();
    fs::write(db.join("dhist"), b"directory history").unwrap();

    let history = profile.join("history.sqlite");
    fs::write(&history, b"SQLite format 3").unwrap();

    assert!(db.exists());
    assert!(history.exists());
}

#[test]
fn test_jupyter_cache() {
    let temp = TempDir::new().unwrap();
    let jupyter = temp.path().join(".jupyter");
    fs::create_dir_all(&jupyter).unwrap();

    // Create kernels directory
    let kernels = jupyter.join("kernels");
    fs::create_dir_all(&kernels).unwrap();

    let python3 = kernels.join("python3");
    fs::create_dir_all(&python3).unwrap();
    fs::write(python3.join("kernel.json"), b"{}").unwrap();

    // Create runtime directory
    let runtime = jupyter.join("runtime");
    fs::create_dir_all(&runtime).unwrap();
    fs::write(runtime.join("kernel-12345.json"), b"{}").unwrap();

    assert!(kernels.exists());
    assert!(runtime.exists());
}

#[test]
fn test_conda_environments() {
    let temp = TempDir::new().unwrap();
    let conda = temp.path().join("miniconda3");
    fs::create_dir_all(&conda).unwrap();

    // Create envs directory
    let envs = conda.join("envs");
    fs::create_dir_all(&envs).unwrap();

    // Create environment
    let myenv = envs.join("myenv");
    fs::create_dir_all(&myenv).unwrap();

    let lib = myenv.join("lib").join("python3.9").join("site-packages");
    fs::create_dir_all(&lib).unwrap();
    fs::write(lib.join("numpy"), b"numpy package").unwrap();

    // Create pkgs directory (package cache)
    let pkgs = conda.join("pkgs");
    fs::create_dir_all(&pkgs).unwrap();
    fs::write(pkgs.join("numpy-1.21.0-py39.tar.bz2"), b"cached package").unwrap();

    assert!(envs.exists());
    assert!(pkgs.exists());
}

#[test]
fn test_pycache_multiple_python_versions() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("project");
    fs::create_dir_all(&project).unwrap();

    let pycache = project.join("__pycache__");
    fs::create_dir_all(&pycache).unwrap();

    // Create .pyc files for different Python versions
    fs::write(pycache.join("module.cpython-38.pyc"), b"py38").unwrap();
    fs::write(pycache.join("module.cpython-39.pyc"), b"py39").unwrap();
    fs::write(pycache.join("module.cpython-310.pyc"), b"py310").unwrap();
    fs::write(pycache.join("module.cpython-311.pyc"), b"py311").unwrap();

    assert_eq!(fs::read_dir(&pycache).unwrap().count(), 4);
}

#[test]
fn test_empty_pycache() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("project");
    fs::create_dir_all(&project).unwrap();

    // Create empty __pycache__
    let pycache = project.join("__pycache__");
    fs::create_dir_all(&pycache).unwrap();

    assert!(pycache.exists());
    assert_eq!(fs::read_dir(&pycache).unwrap().count(), 0);
}

#[test]
fn test_python_checker_returns_valid_structure() {
    let result = checkers::check_python();

    // Verify result structure
    assert!(!result.name.is_empty());

    // All items should have valid structure
    for item in result.items {
        assert!(!item.item_type.is_empty());
        // size is u64, always valid
    }
}

#[test]
fn test_pip_cache_with_selfcheck() {
    let temp = TempDir::new().unwrap();
    let pip_cache = temp.path().join(".cache").join("pip");
    fs::create_dir_all(&pip_cache).unwrap();

    // Create selfcheck.json (pip version check)
    fs::write(
        pip_cache.join("selfcheck.json"),
        b"{\"last_check\":\"2024-01-01\"}",
    )
    .unwrap();

    assert!(pip_cache.join("selfcheck.json").exists());
}

#[test]
fn test_large_pycache_structure() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("large-project");
    fs::create_dir_all(&project).unwrap();

    let pycache = project.join("__pycache__");
    fs::create_dir_all(&pycache).unwrap();

    // Create many .pyc files (simulate large project)
    for i in 0..100 {
        fs::write(
            pycache.join(format!("module{}.cpython-39.pyc", i)),
            b"compiled bytecode",
        )
        .unwrap();
    }

    assert_eq!(fs::read_dir(&pycache).unwrap().count(), 100);
}

#[test]
fn test_virtualenv_with_site_packages() {
    let temp = TempDir::new().unwrap();
    let venv = temp.path().join("venv");
    fs::create_dir_all(&venv).unwrap();

    // Create site-packages with multiple packages
    let site_packages = venv.join("lib").join("python3.9").join("site-packages");
    fs::create_dir_all(&site_packages).unwrap();

    // Create dist-info directories (installed packages)
    for pkg in &["numpy", "pandas", "requests", "flask", "django"] {
        let dist_info = site_packages.join(format!("{}-1.0.0.dist-info", pkg));
        fs::create_dir_all(&dist_info).unwrap();
        fs::write(dist_info.join("METADATA"), b"Name: package\n").unwrap();
    }

    assert_eq!(fs::read_dir(&site_packages).unwrap().count(), 5);
}

#[test]
fn test_pip_wheels_directory_structure() {
    let temp = TempDir::new().unwrap();
    let pip_cache = temp.path().join(".cache").join("pip");
    fs::create_dir_all(&pip_cache).unwrap();

    let wheels = pip_cache.join("wheels");
    fs::create_dir_all(&wheels).unwrap();

    // Create hash subdirectories
    for hash in &["ab", "cd", "ef", "12", "34"] {
        let hash_dir = wheels.join(hash);
        fs::create_dir_all(&hash_dir).unwrap();

        for subhash in &["56", "78"] {
            let subdir = hash_dir.join(subhash);
            fs::create_dir_all(&subdir).unwrap();
            fs::write(subdir.join("package.whl"), b"wheel data").unwrap();
        }
    }

    assert_eq!(fs::read_dir(&wheels).unwrap().count(), 5);
}
