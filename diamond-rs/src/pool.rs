use libc;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type iv_tbl;
    pub type RClass;
    pub type symbol_name;
    pub type RProc;
    pub type REnv;
    pub type mrb_jmpbuf;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_malloc_simple(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_free(_: *mut mrb_state, _: *mut libc::c_void);
}
pub type __darwin_size_t = libc::c_ulong;
pub type int64_t = libc::c_longlong;
pub type size_t = __darwin_size_t;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
/*
** mruby/value.h - mruby value definitions
**
** See Copyright Notice in mruby.h
*/
/* *
 * MRuby Value definition functions and macros.
 */
pub type mrb_sym = uint32_t;
pub type mrb_bool = uint8_t;
/*
** mruby/gc.h - garbage collector for mruby
**
** See Copyright Notice in mruby.h
*/
/* *
 * Uncommon memory management stuffs.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_state {
    pub jmp: *mut mrb_jmpbuf,
    pub allocf: mrb_allocf,
    pub allocf_ud: *mut libc::c_void,
    pub c: *mut mrb_context,
    pub root_c: *mut mrb_context,
    pub globals: *mut iv_tbl,
    pub exc: *mut RObject,
    pub top_self: *mut RObject,
    pub object_class: *mut RClass,
    pub class_class: *mut RClass,
    pub module_class: *mut RClass,
    pub proc_class: *mut RClass,
    pub string_class: *mut RClass,
    pub array_class: *mut RClass,
    pub hash_class: *mut RClass,
    pub range_class: *mut RClass,
    pub float_class: *mut RClass,
    pub fixnum_class: *mut RClass,
    pub true_class: *mut RClass,
    pub false_class: *mut RClass,
    pub nil_class: *mut RClass,
    pub symbol_class: *mut RClass,
    pub kernel_module: *mut RClass,
    pub gc: mrb_gc,
    pub symidx: mrb_sym,
    pub symtbl: *mut symbol_name,
    pub symhash: [mrb_sym; 256],
    pub symcapa: size_t,
    pub symbuf: [libc::c_char; 8],
    pub eException_class: *mut RClass,
    pub eStandardError_class: *mut RClass,
    pub nomem_err: *mut RObject,
    pub stack_err: *mut RObject,
    pub ud: *mut libc::c_void,
    pub atexit_stack: *mut mrb_atexit_func,
    pub atexit_stack_len: uint16_t,
    pub ecall_nest: uint16_t,
}
pub type mrb_atexit_func
    =
    Option<unsafe extern "C" fn(_: *mut mrb_state) -> ()>;
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RObject {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub iv: *mut iv_tbl,
}
/*
** mruby/object.h - mruby object definition
**
** See Copyright Notice in mruby.h
*/
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RBasic {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
}
pub type mrb_vtype = libc::c_uint;
/*  25 */
pub const MRB_TT_MAXDEFINE: mrb_vtype = 25;
/*  24 */
pub const MRB_TT_BREAK: mrb_vtype = 24;
/*  23 */
pub const MRB_TT_ISTRUCT: mrb_vtype = 23;
/*  22 */
pub const MRB_TT_FIBER: mrb_vtype = 22;
/*  21 */
pub const MRB_TT_DATA: mrb_vtype = 21;
/*  20 */
pub const MRB_TT_ENV: mrb_vtype = 20;
/*  19 */
pub const MRB_TT_FILE: mrb_vtype = 19;
/*  18 */
pub const MRB_TT_EXCEPTION: mrb_vtype = 18;
/*  17 */
pub const MRB_TT_RANGE: mrb_vtype = 17;
/*  16 */
pub const MRB_TT_STRING: mrb_vtype = 16;
/*  15 */
pub const MRB_TT_HASH: mrb_vtype = 15;
/*  14 */
pub const MRB_TT_ARRAY: mrb_vtype = 14;
/*  13 */
pub const MRB_TT_PROC: mrb_vtype = 13;
/*  12 */
pub const MRB_TT_SCLASS: mrb_vtype = 12;
/*  11 */
pub const MRB_TT_ICLASS: mrb_vtype = 11;
/*  10 */
pub const MRB_TT_MODULE: mrb_vtype = 10;
/*   9 */
pub const MRB_TT_CLASS: mrb_vtype = 9;
/*   8 */
pub const MRB_TT_OBJECT: mrb_vtype = 8;
/*   7 */
pub const MRB_TT_CPTR: mrb_vtype = 7;
/*   6 */
pub const MRB_TT_FLOAT: mrb_vtype = 6;
/*   5 */
pub const MRB_TT_UNDEF: mrb_vtype = 5;
/*   4 */
pub const MRB_TT_SYMBOL: mrb_vtype = 4;
/*   3 */
pub const MRB_TT_FIXNUM: mrb_vtype = 3;
/*   2 */
pub const MRB_TT_TRUE: mrb_vtype = 2;
/*   1 */
pub const MRB_TT_FREE: mrb_vtype = 1;
/*   0 */
pub const MRB_TT_FALSE: mrb_vtype = 0;
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct mrb_gc {
    pub heaps: *mut mrb_heap_page,
    pub sweeps: *mut mrb_heap_page,
    pub free_heaps: *mut mrb_heap_page,
    pub live: size_t,
    pub arena: *mut *mut RBasic,
    pub arena_capa: libc::c_int,
    pub arena_idx: libc::c_int,
    pub state: mrb_gc_state,
    pub current_white_part: libc::c_int,
    pub gray_list: *mut RBasic,
    pub atomic_gray_list: *mut RBasic,
    pub live_after_mark: size_t,
    pub threshold: size_t,
    pub interval_ratio: libc::c_int,
    pub step_ratio: libc::c_int,
    #[bitfield(name = "iterating", ty = "mrb_bool", bits = "0..=0")]
    #[bitfield(name = "disabled", ty = "mrb_bool", bits = "1..=1")]
    #[bitfield(name = "full", ty = "mrb_bool", bits = "2..=2")]
    #[bitfield(name = "generational", ty = "mrb_bool", bits = "3..=3")]
    #[bitfield(name = "out_of_memory", ty = "mrb_bool", bits = "4..=4")]
    pub iterating_disabled_full_generational_out_of_memory: [u8; 1],
    #[bitfield(padding)]
    pub _pad: [u8; 7],
    pub majorgc_old_threshold: size_t,
}
pub type mrb_gc_state = libc::c_uint;
pub const MRB_GC_STATE_SWEEP: mrb_gc_state = 2;
pub const MRB_GC_STATE_MARK: mrb_gc_state = 1;
pub const MRB_GC_STATE_ROOT: mrb_gc_state = 0;
/* Disable MSVC warning "C4200: nonstandard extension used: zero-sized array
 * in struct/union" when in C++ mode */
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct mrb_heap_page {
    pub freelist: *mut RBasic,
    pub prev: *mut mrb_heap_page,
    pub next: *mut mrb_heap_page,
    pub free_next: *mut mrb_heap_page,
    pub free_prev: *mut mrb_heap_page,
    #[bitfield(name = "old", ty = "mrb_bool", bits = "0..=0")]
    pub old: [u8; 1],
    #[bitfield(padding)]
    pub _pad: [u8; 7],
    pub objects: [*mut libc::c_void; 0],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_context {
    pub prev: *mut mrb_context,
    pub stack: *mut mrb_value,
    pub stbase: *mut mrb_value,
    pub stend: *mut mrb_value,
    pub ci: *mut mrb_callinfo,
    pub cibase: *mut mrb_callinfo,
    pub ciend: *mut mrb_callinfo,
    pub rescue: *mut uint16_t,
    pub rsize: uint16_t,
    pub ensure: *mut *mut RProc,
    pub esize: uint16_t,
    pub eidx: uint16_t,
    pub status: mrb_fiber_state,
    pub vmexec: mrb_bool,
    pub fib: *mut RFiber,
}
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RFiber {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub cxt: *mut mrb_context,
}
pub type mrb_fiber_state = libc::c_uint;
pub const MRB_FIBER_TERMINATED: mrb_fiber_state = 5;
pub const MRB_FIBER_TRANSFERRED: mrb_fiber_state = 4;
pub const MRB_FIBER_SUSPENDED: mrb_fiber_state = 3;
pub const MRB_FIBER_RESUMED: mrb_fiber_state = 2;
pub const MRB_FIBER_RUNNING: mrb_fiber_state = 1;
pub const MRB_FIBER_CREATED: mrb_fiber_state = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_callinfo {
    pub mid: mrb_sym,
    pub proc_0: *mut RProc,
    pub stackent: *mut mrb_value,
    pub ridx: uint16_t,
    pub epos: uint16_t,
    pub env: *mut REnv,
    pub pc: *mut mrb_code,
    pub err: *mut mrb_code,
    pub argc: libc::c_int,
    pub acc: libc::c_int,
    pub target_class: *mut RClass,
}
/*
** mruby - An embeddable Ruby implementation
**
** Copyright (c) mruby developers 2010-2019
**
** Permission is hereby granted, free of charge, to any person obtaining
** a copy of this software and associated documentation files (the
** "Software"), to deal in the Software without restriction, including
** without limitation the rights to use, copy, modify, merge, publish,
** distribute, sublicense, and/or sell copies of the Software, and to
** permit persons to whom the Software is furnished to do so, subject to
** the following conditions:
**
** The above copyright notice and this permission notice shall be
** included in all copies or substantial portions of the Software.
**
** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
** EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
** MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
** IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
** CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
** TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
** SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
**
** [ MIT license: http://www.opensource.org/licenses/mit-license.php ]
*/
/* *
 * MRuby C API entry point
 */
pub type mrb_code = uint8_t;
/*
** mruby/boxing_no.h - unboxed mrb_value definition
**
** See Copyright Notice in mruby.h
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_value {
    pub value: C2RustUnnamed,
    pub tt: mrb_vtype,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed {
    pub f: mrb_float,
    pub p: *mut libc::c_void,
    pub i: mrb_int,
    pub sym: mrb_sym,
}
pub type mrb_int = int64_t;
pub type mrb_float = libc::c_double;
/* *
 * Function pointer type of custom allocator used in @see mrb_open_allocf.
 *
 * The function pointing it must behave similarly as realloc except:
 * - If ptr is NULL it must allocate new space.
 * - If s is NULL, ptr must be freed.
 *
 * See @see mrb_default_allocf for the default implementation.
 */
pub type mrb_allocf
    =
    Option<unsafe extern "C" fn(_: *mut mrb_state, _: *mut libc::c_void,
                                _: size_t, _: *mut libc::c_void)
               -> *mut libc::c_void>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_pool {
    pub mrb: *mut mrb_state,
    pub pages: *mut mrb_pool_page,
}
/* end of configuration section */
/* Disable MSVC warning "C4200: nonstandard extension used: zero-sized array
 * in struct/union" when in C++ mode */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_pool_page {
    pub next: *mut mrb_pool_page,
    pub offset: size_t,
    pub len: size_t,
    pub last: *mut libc::c_void,
    pub page: [libc::c_char; 0],
}
#[no_mangle]
pub unsafe extern "C" fn mrb_pool_open(mut mrb: *mut mrb_state)
 -> *mut mrb_pool {
    let mut pool: *mut mrb_pool =
        mrb_malloc_simple(mrb,
                          ::std::mem::size_of::<mrb_pool>() as libc::c_ulong)
            as *mut mrb_pool;
    if !pool.is_null() {
        (*pool).mrb = mrb;
        (*pool).pages = 0 as *mut mrb_pool_page
    }
    return pool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_pool_close(mut pool: *mut mrb_pool) {
    let mut page: *mut mrb_pool_page = 0 as *mut mrb_pool_page;
    let mut tmp: *mut mrb_pool_page = 0 as *mut mrb_pool_page;
    if pool.is_null() { return }
    page = (*pool).pages;
    while !page.is_null() {
        tmp = page;
        page = (*page).next;
        mrb_free((*pool).mrb, tmp as *mut libc::c_void);
    }
    mrb_free((*pool).mrb, pool as *mut libc::c_void);
}
unsafe extern "C" fn page_alloc(mut pool: *mut mrb_pool, mut len: size_t)
 -> *mut mrb_pool_page {
    let mut page: *mut mrb_pool_page = 0 as *mut mrb_pool_page;
    if len < 16000i32 as libc::c_ulong { len = 16000i32 as size_t }
    page =
        mrb_malloc_simple((*pool).mrb,
                          (::std::mem::size_of::<mrb_pool_page>() as
                               libc::c_ulong).wrapping_add(len)) as
            *mut mrb_pool_page;
    if !page.is_null() { (*page).offset = 0i32 as size_t; (*page).len = len }
    return page;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_pool_alloc(mut pool: *mut mrb_pool,
                                        mut len: size_t)
 -> *mut libc::c_void {
    let mut page: *mut mrb_pool_page = 0 as *mut mrb_pool_page;
    let mut n: size_t = 0;
    if pool.is_null() { return 0 as *mut libc::c_void }
    len =
        (len as
             libc::c_ulong).wrapping_add(18446744073709551615u64.wrapping_sub(len).wrapping_add(1i32
                                                                                                    as
                                                                                                    libc::c_ulong)
                                             & (8i32 - 1i32) as libc::c_ulong)
            as size_t as size_t;
    page = (*pool).pages;
    while !page.is_null() {
        if (*page).offset.wrapping_add(len) <= (*page).len {
            n = (*page).offset;
            (*page).offset =
                ((*page).offset as libc::c_ulong).wrapping_add(len) as size_t
                    as size_t;
            (*page).last =
                (*page).page.as_mut_ptr().offset(n as isize) as
                    *mut libc::c_void;
            return (*page).last
        }
        page = (*page).next
    }
    page = page_alloc(pool, len);
    if page.is_null() { return 0 as *mut libc::c_void }
    (*page).offset = len;
    (*page).next = (*pool).pages;
    (*pool).pages = page;
    (*page).last = (*page).page.as_mut_ptr() as *mut libc::c_void;
    return (*page).last;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_pool_can_realloc(mut pool: *mut mrb_pool,
                                              mut p: *mut libc::c_void,
                                              mut len: size_t) -> mrb_bool {
    let mut page: *mut mrb_pool_page = 0 as *mut mrb_pool_page;
    if pool.is_null() { return 0i32 as mrb_bool }
    len =
        (len as
             libc::c_ulong).wrapping_add(18446744073709551615u64.wrapping_sub(len).wrapping_add(1i32
                                                                                                    as
                                                                                                    libc::c_ulong)
                                             & (8i32 - 1i32) as libc::c_ulong)
            as size_t as size_t;
    page = (*pool).pages;
    while !page.is_null() {
        if (*page).last == p {
            let mut beg: size_t = 0;
            beg =
                (p as
                     *mut libc::c_char).wrapping_offset_from((*page).page.as_mut_ptr())
                    as libc::c_long as size_t;
            if beg.wrapping_add(len) > (*page).len { return 0i32 as mrb_bool }
            return 1i32 as mrb_bool
        }
        page = (*page).next
    }
    return 0i32 as mrb_bool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_pool_realloc(mut pool: *mut mrb_pool,
                                          mut p: *mut libc::c_void,
                                          mut oldlen: size_t,
                                          mut newlen: size_t)
 -> *mut libc::c_void {
    let mut page: *mut mrb_pool_page = 0 as *mut mrb_pool_page;
    let mut np: *mut libc::c_void = 0 as *mut libc::c_void;
    if pool.is_null() { return 0 as *mut libc::c_void }
    oldlen =
        (oldlen as
             libc::c_ulong).wrapping_add(18446744073709551615u64.wrapping_sub(oldlen).wrapping_add(1i32
                                                                                                       as
                                                                                                       libc::c_ulong)
                                             & (8i32 - 1i32) as libc::c_ulong)
            as size_t as size_t;
    newlen =
        (newlen as
             libc::c_ulong).wrapping_add(18446744073709551615u64.wrapping_sub(newlen).wrapping_add(1i32
                                                                                                       as
                                                                                                       libc::c_ulong)
                                             & (8i32 - 1i32) as libc::c_ulong)
            as size_t as size_t;
    page = (*pool).pages;
    while !page.is_null() {
        if (*page).last == p {
            let mut beg: size_t = 0;
            beg =
                (p as
                     *mut libc::c_char).wrapping_offset_from((*page).page.as_mut_ptr())
                    as libc::c_long as size_t;
            if beg.wrapping_add(oldlen) != (*page).offset { break ; }
            if beg.wrapping_add(newlen) > (*page).len {
                (*page).offset = beg;
                break ;
            } else { (*page).offset = beg.wrapping_add(newlen); return p }
        } else { page = (*page).next }
    }
    np = mrb_pool_alloc(pool, newlen);
    if np.is_null() { return 0 as *mut libc::c_void }
    memcpy(np, p, oldlen);
    return np;
}