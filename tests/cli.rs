#[cfg(test)]
#[cfg(not(mov_cross_compile))] // Cross-compilation does not allow to spawn threads but `command.assert()` would do.

mod cli {
    use anyhow::Result;
    use assert_cmd::Command;
    use std::fs;
    use std::path::Path;

    fn mov() -> Command {
        Command::cargo_bin("mov").expect("Error invoking mov")
    }

    #[test]
    fn multiple_preview() -> Result<()> {
        let input = fs::read_to_string("tests/data/multiple/start.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/multiple/patch.patch").expect("Error reading input");
        mov()
            .current_dir("tests/data/multiple")
            .write_stdin(input)
            .args(&["changes", "altered"])
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn missing_preview() -> Result<()> {
        let input = fs::read_to_string("tests/data/missing/start.txt").expect("Error reading input");
        mov()
            .current_dir("tests/data/missing")
            .write_stdin(input)
            .args(&["missing", "replaced"])
            .assert()
            .success()
            .stdout("");
        Ok(())
    }

    #[test]
    fn simple_preview() -> Result<()> {
        let input = fs::read_to_string("tests/data/simple/find.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/simple/patch.patch").expect("Error reading input");
        mov()
            .current_dir("tests/data/simple")
            .write_stdin(input)
            .args(&["changes", "altered"])
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn nested_preview() -> Result<()> {
        let input = fs::read_to_string("tests/data/nested/find.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/nested/patch.patch").expect("Error reading input");
        mov()
            .current_dir("tests/data/nested")
            .write_stdin(input)
            .args(&["changes", "altered"])
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn dirs_preview() -> Result<()> {
        let input = fs::read_to_string("tests/data/dirs/find.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/dirs/patch.patch").expect("Error reading input");
        mov()
            .current_dir("tests/data/dirs")
            .write_stdin(input)
            .args(&["changes", "altered"])
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn simple_move() -> Result<()> {
        let input = fs::read_to_string("tests/data/simple/find.txt").expect("Error reading input");
        let file_path_component = "changes";
        let file_path = Path::new("tests/data/simple").join(file_path_component);
        let tmp_dir = tempfile::tempdir()?;
        let tmp_dir_path = tmp_dir.path();
        let file_path_dst = tmp_dir_path.join(file_path_component);
        let prefix = file_path_dst.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        fs::copy(file_path, &file_path_dst).expect("Error copying file");
        mov()
            .current_dir(tmp_dir_path)
            .write_stdin(input)
            .args(&["changes", "altered", "-w"])
            .assert()
            .success();
        assert!(!Path::exists(&file_path_dst));
        let file_path_component_moved = "altered";
        let file_path_moved = tmp_dir_path.join(file_path_component_moved);
        assert!(Path::exists(&file_path_moved));
        Ok(())
    }

    #[test]
    fn nested_move() -> Result<()> {
        let input = fs::read_to_string("tests/data/nested/find.txt").expect("Error reading input");
        let file_path_component = "change dir with spaces/change dir with spaces two/change file with spaces";
        let file_path = Path::new("tests/data/nested").join(file_path_component);
        let tmp_dir = tempfile::tempdir()?;
        let tmp_dir_path = tmp_dir.path();
        let file_path_dst = tmp_dir_path.join(file_path_component);
        let prefix = file_path_dst.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        fs::copy(file_path, &file_path_dst).expect("Error copying file");
        assert!(Path::exists(&file_path_dst)); // FIXME: Remember to delete this
        let command = mov()
            .current_dir(tmp_dir_path)
            .write_stdin(input)
            .args(&["change", "altered", "-w"])
            .assert()
            .success();
        let output = command.get_output();
        println!("stdout = {:?}", String::from_utf8_lossy(&output.stdout));
        println!("stderr = {:?}", String::from_utf8_lossy(&output.stderr));
        assert!(!Path::exists(&file_path_dst));
        let file_path_component_moved = "tests/data/mov/altered dir with spaces/altered dir with spaces two/altered file with spaces";
        let file_path_moved = tmp_dir_path.join(file_path_component_moved);
        assert!(Path::exists(&file_path_moved));
        Ok(())
    }
}
