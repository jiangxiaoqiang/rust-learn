#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct synctex_scanner_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct synctex_node_t {
    _unused: [u8; 0],
}

#[allow(non_camel_case_types)]
pub type synctex_scanner_s = synctex_scanner_t;
#[allow(non_camel_case_types)]
pub type synctex_scanner_p = *mut synctex_scanner_s;
#[allow(non_camel_case_types)]
pub type synctex_status_t = ::std::os::raw::c_long;
#[allow(non_camel_case_types)]
pub type synctex_node_p = *mut synctex_node_s;
#[allow(non_camel_case_types)]
pub type synctex_node_s = synctex_node_t;

#[link(name = "synctex_parser")]
extern "C" {
    pub fn synctex_scanner_new_with_output_file(
        output: *const ::std::os::raw::c_char,
        build_directory: *const ::std::os::raw::c_char,
        parse: ::std::os::raw::c_int,
    ) -> synctex_scanner_p;

    pub fn synctex_display_query(
        scanner: synctex_scanner_p,
        name: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
        column: ::std::os::raw::c_int,
        page_hint: ::std::os::raw::c_int,
    ) -> synctex_status_t;

    pub fn synctex_scanner_next_result(scanner: synctex_scanner_p) -> synctex_node_p;

    pub fn synctex_node_page(node: synctex_node_p) -> ::std::os::raw::c_int;

    pub fn synctex_node_visible_h(node: synctex_node_p) -> f32;

    pub fn synctex_node_visible_v(node: synctex_node_p) -> f32;

    pub fn synctex_node_visible_width(node: synctex_node_p) -> f32;

    pub fn synctex_node_visible_height(node: synctex_node_p) -> f32;
}
