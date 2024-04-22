mod parse;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Expr, LitStr};

// noinspection SpellCheckingInspection
/// For documentation on how to use this, visit [`SupportedFields`]
///
/// [`SupportedFields`]: ./struct.SupportedFields.html
#[proc_macro]
pub fn arcdps_export(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as parse::ArcDpsGen);
    let sig = input.sig;
    let build = std::env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION is not set") + "\0";
    let build = LitStr::new(build.as_str(), Span::call_site());
    let (raw_name, span) = if let Some(input_name) = input.name {
        let name = input_name.value();
        (name, input_name.span())
    } else {
        let name = std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME is not set");
        (name, Span::call_site())
    };
    let name = LitStr::new(raw_name.as_str(), span.clone());
    let out_name = raw_name + "\0";
    let out_name = LitStr::new(out_name.as_str(), span);

    let (abstract_combat, cb_combat) = build_combat(input.raw_combat, input.combat);
    let (abstract_combat_local, cb_combat_local) =
        build_combat_local(input.raw_combat_local, input.combat_local);
    let (abstract_imgui, cb_imgui) = build_imgui(input.raw_imgui, input.imgui);
    let (abstract_options_end, cb_options_end) =
        build_options_end(input.raw_options_end, input.options_end);
    let (abstract_options_windows, cb_options_windows) =
        build_options_windows(input.raw_options_windows, input.options_windows);
    let (abstract_wnd_filter, cb_wnd_filter) =
        build_wnd_filter(input.raw_wnd_filter, input.wnd_filter);
    let (abstract_wnd_nofilter, cb_wnd_nofilter) =
        build_wnd_nofilter(input.raw_wnd_nofilter, input.wnd_nofilter);

    let export = quote! {
        ArcDpsExport {
            size: ::std::mem::size_of::<ArcDpsExport>(),
            sig: #sig,
            imgui_version: 18000,
            out_build: #build.as_ptr(),
            out_name: #out_name.as_ptr(),
            combat: #cb_combat,
            combat_local: #cb_combat_local,
            imgui: #cb_imgui,
            options_end: #cb_options_end,
            options_windows: #cb_options_windows,
            wnd_filter: #cb_wnd_filter,
            wnd_nofilter: #cb_wnd_nofilter,
        }
    };

    let init = if let Some(init) = input.init {
        let span = syn::Error::new_spanned(&init, "").span();
        quote_spanned! (span => (#init as InitFunc)(SWAPCHAIN))
    } else {
        quote! {Ok(())}
    };

    let release = if let Some(release) = input.release {
        let span = syn::Error::new_spanned(&release, "").span();
        quote_spanned! (span => (#release as ReleaseFunc)())
    } else {
        quote! {}
    };

    let (abstract_extras_squad_update, extras_squad_update) = build_extras_squad_update(
        input.raw_unofficial_extras_squad_update,
        input.unofficial_extras_squad_update,
    );
    let (abstract_extras_chat_message, extras_chat_message) = build_extras_chat_message(
        input.raw_unofficial_extras_chat_message,
        input.unofficial_extras_chat_message,
    );
    let abstract_extras_init = build_extras_init(
        input.raw_unofficial_extras_init,
        input.unofficial_extras_init,
        extras_squad_update,
        extras_chat_message,
        &out_name,
    );

    #[cfg(feature = "imgui")]
    let sys_init = quote! {
        use ::arcdps::imgui;

        #[no_mangle]
        // export -- arcdps looks for this exported function and calls the address it returns on client load
        // if you need any of the ignored values, create an issue with your use case
        pub unsafe extern "system" fn get_init_addr(
            arc_version: PCCHAR,
            imguictx: *mut imgui::sys::ImGuiContext,
            id3dptr: LPVOID,
            arc_dll: HANDLE,
            mallocfn: Option<unsafe extern "C" fn(sz: usize, user_data: *mut c_void) -> *mut c_void>,
            freefn: Option<unsafe extern "C" fn(ptr: *mut c_void, user_data: *mut c_void)>,
        ) -> fn() -> &'static ArcDpsExport {
            imgui::sys::igSetCurrentContext(imguictx);
            imgui::sys::igSetAllocatorFunctions(mallocfn, freefn, ::core::ptr::null_mut());
            CTX = Some(imgui::Context::current());
            UI = Some(imgui::Ui::from_ctx(CTX.as_ref().unwrap()));
            SWAPCHAIN = if !id3dptr.is_null() { Some(id3dptr) } else { None };
            ::arcdps::__init(arc_version, arc_dll, #name);
            load
        }

        static mut CTX: Option<imgui::Context> = None;
        static mut UI: Option<imgui::Ui> = None;
        static mut SWAPCHAIN: Option<LPVOID> = None;
    };

    #[cfg(not(feature = "imgui"))]
    let sys_init = quote! {
        #[no_mangle]
        // export -- arcdps looks for this exported function and calls the address it returns on client load
        // if you need any of the ignored values, create an issue with your use case
        pub unsafe extern "system" fn get_init_addr(
            arc_version: PCCHAR,
            _imguictx: *mut c_void,
            id3dptr: LPVOID,
            arc_dll: HANDLE,
            _mallocfn: Option<unsafe extern "C" fn(sz: usize, user_data: *mut c_void) -> *mut c_void>,
            _freefn: Option<unsafe extern "C" fn(ptr: *mut c_void, user_data: *mut c_void)>,
        ) -> fn() -> &'static ArcDpsExport {
            SWAPCHAIN = if !id3dptr.is_null() { Some(id3dptr) } else { None };
            ::arcdps::__init(arc_version, arc_dll, #name);
            load
        }

        static mut SWAPCHAIN: LPVOID = Option<LPVOID> = None;
    };

    let res = quote! {
        mod __arcdps_gen_export {
            use super::*;
            use ::std::os::raw::{c_char, c_void};
            use ::arcdps::helpers;
            use ::arcdps::ArcDpsExport;
            use ::arcdps::{InitFunc, ReleaseFunc};

            type LPARAM = isize;
            type LPVOID = *mut c_void;
            type UINT = u32;
            type WPARAM = usize;
            type PCCHAR = *mut c_char;
            type HWND = *mut c_void;
            type HANDLE = *mut c_void;

            #abstract_combat
            #abstract_combat_local
            #abstract_imgui
            #abstract_options_end
            #abstract_options_windows
            #abstract_wnd_filter
            #abstract_wnd_nofilter
            #abstract_extras_squad_update
            #abstract_extras_chat_message
            #abstract_extras_init

            static EXPORT: ArcDpsExport = #export;
            static mut EXPORT_ERROR: ArcDpsExport = ArcDpsExport {
                    size: 0,
                    sig: 0,
                    imgui_version: 18000,
                    out_build: #build.as_ptr(),
                    out_name: #out_name.as_ptr(),
                    combat: None,
                    combat_local: None,
                    imgui: None,
                    options_end: None,
                    options_windows: None,
                    wnd_filter: None,
                    wnd_nofilter: None,
                };
            static mut ERROR_STRING: String = String::new();

            fn load() -> &'static ArcDpsExport {
                let mut export = &EXPORT;
                let res: Result<(), Box<dyn ::std::error::Error>> = unsafe { #init };
                if let Err(e) = res {
                    unsafe {
                        ERROR_STRING = e.to_string() + "\0";
                        EXPORT_ERROR.size = ERROR_STRING.as_ptr() as _;
                        export = &EXPORT_ERROR;
                    }
                }

                export
            }

            fn unload() {
                #release
            }

            #sys_init

            #[no_mangle]
            /* export -- arcdps looks for this exported function and calls the address it returns on client exit */
            pub extern "system" fn get_release_addr() -> LPVOID {
                unload as LPVOID
            }
        }
    };
    res.into()
}

fn build_extras_squad_update(
    raw: Option<Expr>,
    safe: Option<Expr>,
) -> (TokenStream, Option<TokenStream>) {
    let mut abstract_wrapper = quote! {};
    let cb_safe = match (raw, safe) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            Some(quote_spanned!(span => Some(#raw as _) ))
        }
        (_, Some(safe)) => {
            let span = syn::Error::new_spanned(&safe, "").span();
            abstract_wrapper = quote_spanned!(span =>
            unsafe extern "C" fn abstract_extras_squad_update(users: *const ::arcdps::RawUserInfo, count: u64) {
                let _ = #safe as ::arcdps::ExtrasSquadUpdateCallback;
                let users = ::std::slice::from_raw_parts(users, count as _);
                let users = users.iter().map(::arcdps::helpers::convert_extras_user as ::arcdps::UserConvert);
                #safe(users)
            });
            Some(
                quote_spanned!(span => Some(__arcdps_gen_export::abstract_extras_squad_update as _) ),
            )
        }
        _ => None,
    };
    (abstract_wrapper, cb_safe)
}

fn build_extras_chat_message(
    raw: Option<Expr>,
    safe: Option<Expr>,
) -> (TokenStream, Option<TokenStream>) {
    let mut abstract_wrapper = quote! {};
    let cb_safe = match (raw, safe) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            Some(quote_spanned!(span => Some(#raw as _) ))
        }
        (_, Some(safe)) => {
            let span = syn::Error::new_spanned(&safe, "").span();
            abstract_wrapper = quote_spanned!(span =>
            unsafe extern "C" fn abstract_extras_chat_message(msg: *const ::arcdps::RawChatMessageInfo) {
                let _ = #safe as ::arcdps::ExtrasChatMessageCallback;
                let msg = ::arcdps::helpers::convert_extras_chat_message(&*msg);
                #safe(&msg)
            });
            Some(
                quote_spanned!(span => Some(__arcdps_gen_export::abstract_extras_chat_message as _) ),
            )
        }
        _ => None,
    };
    (abstract_wrapper, cb_safe)
}

fn build_extras_init(
    raw: Option<Expr>,
    safe: Option<Expr>,
    squad_update: Option<TokenStream>,
    chat_message: Option<TokenStream>,
    name: &LitStr,
) -> TokenStream {
    let needs_init = squad_update.is_some() || chat_message.is_some();
    let squad_cb = squad_update.unwrap_or(quote! { None });
    let chat_cb = chat_message.unwrap_or(quote! { None });

    let basic_init = quote!(
        if addon.api_version != 2 {
            return;
        }
        if addon.max_info_version < 1 {
            return;
        }

        if addon.max_info_version == 1 {
            let sub: *mut ::arcdps::RawExtrasSubscriberInfoHeader = sub;
            let sub = &mut *(sub as *mut ::arcdps::RawExtrasSubscriberInfoV1);

            sub.info_version = 1;
            sub.subscriber_name = #name.as_ptr();
            sub.squad_update_callback = #squad_cb;
            sub.language_changed_callback = None;
            sub.key_bind_changed_callback = None;
        } else {
            let sub: *mut ::arcdps::RawExtrasSubscriberInfoHeader = sub;
            let sub = &mut *(sub as *mut ::arcdps::RawExtrasSubscriberInfoV2);

            sub.info_version = 2;
            sub.subscriber_name = #name.as_ptr();
            sub.squad_update_callback = #squad_cb;
            sub.language_changed_callback = None;
            sub.key_bind_changed_callback = None;
            sub.chat_message_callback = #chat_cb;
        }
    );

    let abstract_wrapper = match (raw, safe) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            quote_spanned!(span =>
                let _ = #raw as ::arcdps::RawExtrasSubscriberInitSignature;

                #raw(addon, sub)
            )
        }
        (_, Some(safe)) => {
            let span = syn::Error::new_spanned(&safe, "").span();
            quote_spanned!(span =>
                #basic_init

                let _ = #safe as ::arcdps::ExtrasInitFunc;
                use ::arcdps::helpers::get_str_from_pc_char as pc;
                let user = pc(addon.self_account_name as _).map(|n| n.trim_start_matches(':'));
                let version = pc(addon.string_version as _);

                #safe(user, version)
            )
        }
        _ if needs_init => basic_init,
        _ => return quote! {},
    };
    use syn::spanned::Spanned;
    quote_spanned!(abstract_wrapper.span() =>
        #[no_mangle]
        unsafe extern "system" fn arcdps_unofficial_extras_subscriber_init(
            addon: &::arcdps::RawExtrasAddonInfo, sub: &mut ::arcdps::RawExtrasSubscriberInfoHeader) {

            #abstract_wrapper
        }
    )
}

fn build_wnd_filter(raw_wnd: Option<Expr>, wnd: Option<Expr>) -> (TokenStream, TokenStream) {
    build_wnd(raw_wnd, wnd, quote! { abstract_wnd_filter })
}

fn build_wnd_nofilter(raw_wnd: Option<Expr>, wnd: Option<Expr>) -> (TokenStream, TokenStream) {
    build_wnd(raw_wnd, wnd, quote! { abstract_wnd_nofilter })
}

fn build_wnd(
    raw_wnd_filter: Option<Expr>,
    wnd_filter: Option<Expr>,
    func_name: TokenStream,
) -> (TokenStream, TokenStream) {
    let mut abstract_wnd_filter = quote! {};
    let cb_wnd_filter = match (raw_wnd_filter, wnd_filter) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            quote_spanned!(span => Some(#raw as _) )
        }
        (_, Some(safe)) => {
            let span = syn::Error::new_spanned(&safe, "").span();
            abstract_wnd_filter = quote_spanned!(span =>
            unsafe extern "C" fn #func_name (_h_wnd: HWND, u_msg: UINT,
                    w_param: WPARAM, l_param: LPARAM
                ) -> UINT {
                let _ = #safe as ::arcdps::WndProcCallback;
                use ::arcdps::{WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP};
                match u_msg {
                    WM_KEYDOWN | WM_KEYUP | WM_SYSKEYDOWN | WM_SYSKEYUP => {
                        let key_down = u_msg & 1 == 0;
                        let prev_key_down = (l_param >> 30) & 1 == 1;

                        if #safe(w_param, key_down, prev_key_down)
                        {
                            u_msg
                        } else {
                            0
                        }
                    },
                    _ => u_msg,
                }
            });
            quote_spanned!(span => Some(__arcdps_gen_export::#func_name as _) )
        }
        _ => quote! { None },
    };
    (abstract_wnd_filter, cb_wnd_filter)
}

fn build_options_windows(
    raw_options_windows: Option<Expr>,
    options_windows: Option<Expr>,
) -> (TokenStream, TokenStream) {
    let mut abstract_options_windows = quote! {};
    let cb_options_windows = match (raw_options_windows, options_windows) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            quote_spanned!(span => Some(#raw as _) )
        }
        (_, Some(safe)) => {
            let span = syn::Error::new_spanned(&safe, "").span();
            abstract_options_windows = quote_spanned!(span =>
            unsafe extern "C" fn abstract_options_windows(window_name: PCCHAR) -> bool {
                let _ = #safe as ::arcdps::OptionsWindowsCallback;
                let ui = UI.as_ref().unwrap();
                #safe(ui, helpers::get_str_from_pc_char(window_name))
            });
            quote_spanned!(span => Some(__arcdps_gen_export::abstract_options_windows as _) )
        }
        _ => quote! { None },
    };
    (abstract_options_windows, cb_options_windows)
}

fn build_options_end(
    raw_options_end: Option<Expr>,
    options_end: Option<Expr>,
) -> (TokenStream, TokenStream) {
    let mut abstract_options_end = quote! {};
    let cb_options_end = match (raw_options_end, options_end) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            quote_spanned!(span => Some(#raw as _) )
        }
        (_, Some(safe)) => {
            let span = syn::Error::new_spanned(&safe, "").span();
            abstract_options_end = quote_spanned!(span =>
            unsafe extern "C" fn abstract_options_end() {
                let _ = #safe as ::arcdps::OptionsCallback;
                let ui = UI.as_ref().unwrap();
                #safe(ui)
            });
            quote_spanned!(span => Some(__arcdps_gen_export::abstract_options_end as _) )
        }
        _ => quote! { None },
    };
    (abstract_options_end, cb_options_end)
}

fn build_imgui(raw_imgui: Option<Expr>, imgui: Option<Expr>) -> (TokenStream, TokenStream) {
    let mut abstract_imgui = quote! {};
    let cb_imgui = match (raw_imgui, imgui) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            quote_spanned!(span => Some(#raw as _) )
        }
        (_, Some(safe)) => {
            let span = syn::Error::new_spanned(&safe, "").span();
            abstract_imgui = quote_spanned!(span =>
            unsafe extern "C" fn abstract_imgui(loading: u32) {
                let _ = #safe as ::arcdps::ImguiCallback;
                let ui = UI.as_ref().unwrap();
                #safe(ui, loading != 0)
            });
            quote_spanned!(span => Some(__arcdps_gen_export::abstract_imgui as _) )
        }
        _ => quote! { None },
    };
    (abstract_imgui, cb_imgui)
}

fn build_combat_local(
    raw_combat: Option<Expr>,
    combat: Option<Expr>,
) -> (TokenStream, TokenStream) {
    build_cbt(raw_combat, combat, quote! { abstract_combat_local })
}

fn build_combat(raw_combat: Option<Expr>, combat: Option<Expr>) -> (TokenStream, TokenStream) {
    build_cbt(raw_combat, combat, quote! { abstract_combat })
}

fn build_cbt(
    raw_combat: Option<Expr>,
    combat: Option<Expr>,
    func_name: TokenStream,
) -> (TokenStream, TokenStream) {
    let mut abstract_combat = quote! {};
    let cb_combat = match (raw_combat, combat) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            quote_spanned!(span => Some(#raw as _) )
        }
        (_, Some(safe)) => {
            let span = syn::Error::new_spanned(&safe, "").span();
            abstract_combat = quote_spanned!(span =>
            unsafe extern "C" fn #func_name(
                    ev: Option<&::arcdps::CombatEvent>,
                    src: Option<&::arcdps::RawAgent>,
                    dst: Option<&::arcdps::RawAgent>,
                    skill_name: PCCHAR,
                    id: u64,
                    revision: u64,
                ) {
                    let _ = #safe as ::arcdps::CombatCallback;
                    let args = ::arcdps::helpers::get_combat_args_from_raw(ev, src, dst, skill_name);
                    #safe(args.ev, args.src, args.dst, args.skill_name, id, revision)
            });
            quote_spanned!(span => Some(__arcdps_gen_export::#func_name as _) )
        }
        _ => quote! { None },
    };
    (abstract_combat, cb_combat)
}
