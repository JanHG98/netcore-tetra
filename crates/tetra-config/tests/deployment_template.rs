use std::path::Path;
use std::process::Command;
use tetra_config::bluestation::from_toml_str;

#[test]
fn deploy_created_templates_pass_the_actual_tbs_parser_and_rf_validation() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let script = r#"
import sys
sys.path.insert(0, 'system-backend/deployment-core')
from common import toml_dump
from deploy import new_tbs_template, tbs_config
profile = dict(name='TBS-CREATOR', mcc=901, mnc=1510, issi=4010007, la=3, cc=7)
for settings in ({}, {'secondary_carrier': None},
        {'freq_offset': -6250, 'main_carrier': 719, 'secondary_carrier': 720,
         'custom_duplex_spacing': 7600000, 'reverse_operation': True}):
    print(tbs_config(toml_dump(new_tbs_template(settings)), profile), end='\0')
"#;
    let output = Command::new("python3").args(["-c", script]).current_dir(root)
        .output().expect("run actual Deploy template generator");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let text = String::from_utf8(output.stdout).unwrap();
    let configs: Vec<_> = text.split('\0').filter(|text| !text.is_empty()).collect();
    assert_eq!(configs.len(), 3);
    for text in configs {
        let config = from_toml_str(text).expect("generated template must parse in the TBS");
        config.validate().expect("generated SDR and advertised carrier frequencies must agree");
        assert_eq!(config.net.mcc, 901);
        assert_eq!(config.cell.location_area, 3);
        assert_eq!(config.cell.colour_code, 7);
        assert!(config.brew.is_none());
        assert!(!config.asterisk.enabled);
        assert_eq!(config.dashboard.as_ref().unwrap().port, 8080);
        assert!(config.dashboard.as_ref().unwrap().password.is_none());
    }
}
