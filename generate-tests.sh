set -e

elementIn() {
    local e
    for e in "${@:2}"; do
        [[ "$e" == "$1" ]] && return 0
    done
    return 1
}

main() {
    local tests_dir=$(pwd)/tests
    local cmsis_dir=$tests_dir/cmsis_tests
    local blacklist=(
        # These SVD files have some registers with a `resetValue` bigger than the register itself
        Toshiba/M365
        Toshiba/M367
        Toshiba/M368
        Toshiba/M369
        Toshiba/M36B
        SiliconLabs/SIM3L1x8_SVD
    )

    rm -rf tests/cmsis_tests
    mkdir -p tests/cmsis_tests

    local svd_source=cmsis-svd-data
    if [ ! -d $svd_source ]
    then
        git clone https://github.com/cmsis-svd/cmsis-svd-data.git
    fi

    local vendor_dir
    for vendor_dir in $(echo $svd_source/data/*); do
        local vendor=$(basename $vendor_dir)
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

            cat >>"$cmsis_dir/$vendor.rs" <<EOF
#[test]
fn $device() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/$device_path"));

    svd::parse(xml).unwrap();
}
EOF
	done
	cat >>"$cmsis_dir/mod.rs" <<EOF
pub mod $vendor;
EOF
    done
    cat >"$tests_dir/cmsis.rs"<<EOF
pub mod cmsis_tests;
EOF
}

main
