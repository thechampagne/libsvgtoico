/*
* Copyright (c) 2022 XXIV
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::CStr;
use std::slice;
use std::path::Path;
use svg_to_ico::svg_to_ico as to_ico;

#[no_mangle]
unsafe extern "C" fn svg_to_ico(
    svg_path: *const c_char,
    svg_dpi: f32,
    ico_path: *const c_char,
    ico_entry_sizes: *const u16,
    ico_entry_sizes_len: usize) -> c_int {

    let svg_path_rs = match CStr::from_ptr(svg_path).to_str() {
	Ok(str) => Path::new(str),
	Err(_) => return -1
    };
    let ico_path_rs = match CStr::from_ptr(ico_path).to_str() {
	Ok(str) => Path::new(str),
	Err(_) => return -1
    };
    let ico_entry_sizes_rs = slice::from_raw_parts(ico_entry_sizes, ico_entry_sizes_len);
    
    if to_ico(svg_path_rs, svg_dpi, ico_path_rs, ico_entry_sizes_rs).is_ok() {
	0
    } else {
	-1
    }
}
