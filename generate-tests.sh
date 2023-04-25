set -e

elementIn() {
    local e
    for e in "${@:2}"; do
        [[ "$e" == "$1" ]] && return 0
    done
    return 1
}

main() {
    git clone https://github.com/posborne/cmsis-svd || true

    local tests_dir=$(pwd)/tests
    local cmsis_dir=$tests_dir/src/cmsis_tests
    local blacklist=(
        # These SVD files have some registers with a `resetValue` bigger than the register itself
        Toshiba/M365
        Toshiba/M367
        Toshiba/M368
        Toshiba/M369
        Toshiba/M36B
        SiliconLabs/SIM3L1x8_SVD
    )

    rm -rf $cmsis_dir
    mkdir -p $cmsis_dir
    >"$cmsis_dir/../cmsis_tests.rs"

    local vendor_dir
    for vendor_dir in $(echo cmsis-svd/data/*); do
        local vendor=$(basename $vendor_dir)
        vendor=${vendor//-/_}
        cat >"$cmsis_dir/$vendor.rs" <<EOF
#![allow(non_snake_case)]

use svd_parser as svd;

EOF

        local device_path

        for device_path in $(find $vendor_dir/* -name '*.svd'); do
            local device=$(basename $device_path)
            device=${device%.svd}

            if elementIn "$vendor/$device" "${blacklist[@]}"; then
                continue
            fi

            device=${device//./_}
            device=${device//-/_}

            cat >>"$cmsis_dir/$vendor.rs" <<EOF
#[test]
fn $device() {
    use std::io::Read;
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../$device_path");
    let mut file = std::fs::File::open(path).unwrap();
    let mut xml = String::new();
    file.read_to_string(&mut xml).unwrap();

    svd::parse(&xml).unwrap();
}
EOF
	done
	cat >>"$cmsis_dir/../cmsis_tests.rs" <<EOF
pub mod $vendor;
EOF
    done
}

main
