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
    local blacklist=(
        # These SVD files have some registers with a `resetValue` bigger than the register itself
        Toshiba/M365
        Toshiba/M367
        Toshiba/M368
        Toshiba/M369
        Toshiba/M36B
        SiliconLabs/SIM3L1x8_SVD
    )

    rm -rf tests
    mkdir -p tests

    local vendor_dir
    for vendor_dir in $(echo cmsis-svd/data/*); do
        local vendor=$(basename $vendor_dir)
        cat >"$tests_dir/$vendor.rs" <<EOF
#![allow(non_snake_case)]

extern crate svd_parser as svd;

EOF

        local device_path
        for device_path in $(find $vendor_dir/* -name '*.svd'); do
            local device=$(basename $device_path)
            device=${device%.svd}

            if elementIn "$vendor/$device" "${blacklist[@]}"; then
                continue
            fi

            device=${device//./_}

            cat >>"$tests_dir/$vendor.rs" <<EOF
#[test]
fn $device() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/$device_path"));

    svd::parse(xml).unwrap();
}
EOF
        done
    done
}

main
