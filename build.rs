use std::env;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

static RUSTIFIED_ENUMS: &[&str] = &[];

static ALLOWED_TYPES: &[&str] = &["C*"];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=primedev");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let msvc = target.contains("msvc");

    // let mut is_reading_headers = {
    //     let mut is_reading_headers = false;
    //     move |line: &str| {
    //         if !is_reading_headers {
    //             is_reading_headers = line.starts_with("add_library")
    //         }

    //         if is_reading_headers && line.starts_with(')') {
    //             is_reading_headers = false;
    //         }

    //         eprintln!("{}", line);

    //         is_reading_headers && line.find('"').is_some() && line.ends_with(".h\"")
    //     }
    // };

    // let ns_cmake = fs::read_to_string("primedev/primedev/Northstar.cmake")
    //     .expect("Northstar.cmake should exist");
    // let headers = ns_cmake
    //     .lines()
    //     .filter(|line| is_reading_headers(line))
    //     .filter_map(|line| {
    //         line.trim()
    //             .split_once('"')
    //             .map(|split| split.1)
    //             .and_then(|rest_of_line| rest_of_line.split_once('"').map(|split| split.0))
    //     });

    let builder = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate_inline_functions(false)
        .generate_cstr(true)
        .ignore_functions()
        // .opaque_type("std")
        .opaque_type("std::string")
        .derive_debug(true)
        .size_t_is_usize(true)
        .blocklist_file("/thirdparty/")
        .clang_arg("-v")
        .clang_arg("-I \"primedev/primedev\"")
        .clang_args(&["-x", "c++", "-std=c++20"]);

    let mut headers = Vec::with_capacity(32);
    let mut dirs = Vec::with_capacity(32);
    find_all_headers(
        &mut headers,
        &mut dirs,
        &PathBuf::from("primedev/primedev/"),
    );

    let mut builder = ALL_THE_FILES.iter().fold(
        builder.clang_args(
            dirs.into_iter()
                .map(|dir| format!("--include-directory={}", dir.to_string_lossy())),
        ),
        |builder, header| builder.header(*header),
    );

    if msvc {
        builder = builder.clang_arg("-fms-compatibility-version=19");
    }

    for &enumm in RUSTIFIED_ENUMS {
        builder = builder.rustified_enum(enumm);
    }

    for &allowed_type in ALLOWED_TYPES {
        builder = builder.allowlist_type(allowed_type)
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_dir = out_dir.join("bindings.rs");

    println!("out dir : {out_dir:?}");

    bindings
        .write_to_file(out_dir.clone())
        .expect("Couldn't write bindings!");
}

fn find_all_headers(headers: &mut Vec<PathBuf>, dirs: &mut Vec<PathBuf>, folder_path: &Path) {
    for path in folder_path
        .read_dir()
        .expect("this was not a dir")
        .filter_map(Result::<DirEntry, std::io::Error>::ok)
        .map(|item| item.path())
    {
        match path.is_dir() {
            true => find_all_headers(headers, dirs, path.as_path()),
            false if matches!(path.extension().and_then(|s| s.to_str()), Some("h")) => {
                headers.push(path)
            }
            _ => {}
        }
    }
    dirs.push(folder_path.to_path_buf());
}

const ALL_THE_FILES: &[&str] = &[
    // r#"primedev/primedev/appframework\IAppSystem.h"#,
    r#"primedev/primedev/client\localchatwriter.h"#,
    // r#"primedev/primedev/codecs\miles\core.h"#,
    // r#"primedev/primedev/common\callbacks.h"#,
    r#"primedev/primedev/common\globals_cvar.h"#,
    r#"primedev/primedev/core\assert.h"#,
    r#"primedev/primedev/core\basetypes.h"#,
    r#"primedev/primedev/core\hooks.h"#,
    r#"primedev/primedev/core\init.h"#,
    r#"primedev/primedev/core\macros.h"#,
    r#"primedev/primedev/core\stdafx.h"#,
    r#"primedev/primedev/dedicated\dedicated.h"#,
    r#"primedev/primedev/dedicated\dedicatedlogtoclient.h"#,
    r#"primedev/primedev/engine\baseserver.h"#,
    r#"primedev/primedev/engine\cdll_engine_int.h"#,
    r#"primedev/primedev/engine\client\client.h"#,
    r#"primedev/primedev/engine\cl_splitscreen.h"#,
    r#"primedev/primedev/engine\cmd.h"#,
    r#"primedev/primedev/engine\datamap.h"#,
    r#"primedev/primedev/engine\debugoverlay.h"#,
    r#"primedev/primedev/engine\edict.h"#,
    r#"primedev/primedev/engine\eiface.h"#,
    r#"primedev/primedev/engine\globalvars_base.h"#,
    r#"primedev/primedev/engine\host.h"#,
    r#"primedev/primedev/engine\hoststate.h"#,
    r#"primedev/primedev/engine\ivdebugoverlay.h"#,
    r#"primedev/primedev/engine\server\server.h"#,
    r#"primedev/primedev/engine\sys_engine.h"#,
    r#"primedev/primedev/engine\vengineserver_impl.h"#,
    r#"primedev/primedev/engine\vphysics_interface.h"#,
    r#"primedev/primedev/filesystem\basefilesystem.h"#,
    r#"primedev/primedev/game\client\cdll_client_int.h"#,
    r#"primedev/primedev/game\client\hud_chat.h"#,
    r#"primedev/primedev/game\client\vscript_client.h"#,
    r#"primedev/primedev/game\server\ai_helper.h"#,
    r#"primedev/primedev/game\server\ai_navmesh.h"#,
    r#"primedev/primedev/game\server\ai_networkmanager.h"#,
    r#"primedev/primedev/game\server\ai_node.h"#,
    r#"primedev/primedev/game\server\baseanimating.h"#,
    r#"primedev/primedev/game\server\baseanimatingoverlay.h"#,
    r#"primedev/primedev/game\server\basecombatcharacter.h"#,
    r#"primedev/primedev/game\server\baseentity.h"#,
    r#"primedev/primedev/game\server\basetoggle.h"#,
    r#"primedev/primedev/game\server\enginecallback.h"#,
    r#"primedev/primedev/game\server\entitylist.h"#,
    r#"primedev/primedev/game\server\gameinterface.h"#,
    r#"primedev/primedev/game\server\player.h"#,
    r#"primedev/primedev/game\server\recipientfilter.h"#,
    r#"primedev/primedev/game\server\triggers.h"#,
    r#"primedev/primedev/game\server\util_server.h"#,
    r#"primedev/primedev/game\server\vscript_server.h"#,
    r#"primedev/primedev/game\shared\vscript_shared.h"#,
    r#"primedev/primedev/gameui\GameConsole.h"#,
    r#"primedev/primedev/logging\logging.h"#,
    r#"primedev/primedev/logging\loghooks.h"#,
    r#"primedev/primedev/materialsystem\cmaterialglue.h"#,
    r#"primedev/primedev/materialsystem\cshaderglue.h"#,
    r#"primedev/primedev/mathlib\bitbuf.h"#,
    r#"primedev/primedev/mathlib\bits.h"#,
    r#"primedev/primedev/mathlib\bitvec.h"#,
    r#"primedev/primedev/mathlib\color.h"#,
    r#"primedev/primedev/mathlib\mathlib.h"#,
    r#"primedev/primedev/mathlib\math_pfns.h"#,
    r#"primedev/primedev/mathlib\matrix.h"#,
    r#"primedev/primedev/mathlib\quaternion.h"#,
    r#"primedev/primedev/mathlib\swap.h"#,
    r#"primedev/primedev/mathlib\vector.h"#,
    r#"primedev/primedev/mathlib\vplane.h"#,
    r#"primedev/primedev/mods\audio.h"#,
    r#"primedev/primedev/mods\modmanager.h"#,
    r#"primedev/primedev/mods\modsavefiles.h"#,
    r#"primedev/primedev/networksystem\atlas.h"#,
    r#"primedev/primedev/networksystem\bansystem.h"#,
    r#"primedev/primedev/networksystem\bcrypt.h"#,
    r#"primedev/primedev/networksystem\inetmsghandler.h"#,
    r#"primedev/primedev/networksystem\netchannel.h"#,
    r#"primedev/primedev/originsdk\origin.h"#,
    r#"primedev/primedev/originsdk\overlay.h"#,
    r#"primedev/primedev/rtech\datatable.h"#,
    r#"primedev/primedev/rtech\pakapi.h"#,
    r#"primedev/primedev/shared\exploit_fixes\ns_limits.h"#,
    r#"primedev/primedev/shared\misccommands.h"#,
    r#"primedev/primedev/shared\playlist.h"#,
    // r#"primedev/primedev/thirdparty\silver-bun\memaddr.h"#,
    // r#"primedev/primedev/thirdparty\silver-bun\module.h"#,
    // r#"primedev/primedev/thirdparty\silver-bun\utils.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\async.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\async_logger-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\async_logger.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\cfg\argv.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\cfg\env.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\cfg\helpers-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\cfg\helpers.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\common-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\common.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\backtracer-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\backtracer.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\circular_q.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\console_globals.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\file_helper-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\file_helper.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\fmt_helper.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\log_msg-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\log_msg.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\log_msg_buffer-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\log_msg_buffer.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\mpmc_blocking_q.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\null_mutex.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\os-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\os.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\periodic_worker-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\periodic_worker.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\registry-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\registry.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\synchronous_factory.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\tcp_client-windows.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\tcp_client.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\thread_pool-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\thread_pool.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\details\windows_include.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\bin_to_hex.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\bundled\chrono.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\bundled\color.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\bundled\compile.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\bundled\core.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\bundled\format-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\bundled\format.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\bundled\locale.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\bundled\os.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\bundled\ostream.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\bundled\posix.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\bundled\printf.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\bundled\ranges.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\chrono.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\fmt.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fmt\ostr.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\formatter.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\fwd.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\logger-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\logger.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\pattern_formatter-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\pattern_formatter.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\android_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\ansicolor_sink-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\ansicolor_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\base_sink-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\base_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\basic_file_sink-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\basic_file_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\daily_file_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\dist_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\dup_filter_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\hourly_file_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\msvc_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\null_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\ostream_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\ringbuffer_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\rotating_file_sink-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\rotating_file_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\sink-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\stdout_color_sinks-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\stdout_color_sinks.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\stdout_sinks-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\stdout_sinks.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\syslog_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\systemd_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\tcp_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\wincolor_sink-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\wincolor_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\sinks\win_eventlog_sink.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\spdlog-inl.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\spdlog.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\stopwatch.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\tweakme.h"#,
    // r#"primedev/primedev/thirdparty\spdlog\version.h"#,
    r#"primedev/primedev/tier0\commandline.h"#,
    r#"primedev/primedev/tier0\cpu.h"#,
    r#"primedev/primedev/tier0\cputopology.h"#,
    r#"primedev/primedev/tier0\crashhandler.h"#,
    r#"primedev/primedev/tier0\dbg.h"#,
    r#"primedev/primedev/tier0\fasttimer.h"#,
    r#"primedev/primedev/tier0\filestream.h"#,
    r#"primedev/primedev/tier0\memstd.h"#,
    r#"primedev/primedev/tier0\platform.h"#,
    r#"primedev/primedev/tier0\taskscheduler.h"#,
    r#"primedev/primedev/tier0\threadtools.h"#,
    r#"primedev/primedev/tier0\utils.h"#,
    r#"primedev/primedev/tier1\cmd.h"#,
    r#"primedev/primedev/tier1\convar.h"#,
    r#"primedev/primedev/tier1\cvar.h"#,
    r#"primedev/primedev/tier1\interface.h"#,
    r#"primedev/primedev/tier1\keyvalues.h"#,
    r#"primedev/primedev/tier1\utlmemory.h"#,
    r#"primedev/primedev/tier1\utlvector.h"#,
    r#"primedev/primedev/tier2\curlutils.h"#,
    r#"primedev/primedev/toolframework\itoolentity.h"#,
    r#"primedev/primedev/utils\primelauncher\launcher.h"#,
    r#"primedev/primedev/utils\primelauncher\resource1.h"#,
    r#"primedev/primedev/vgui\vgui_baseui_interface.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\include\squirrel.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\squirrel\sqarray.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\squirrel\sqclosure.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\squirrel\sqcompiler.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\squirrel\sqfunctionproto.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\squirrel\sqlexer.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\squirrel\sqobject.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\squirrel\sqopcodes.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\squirrel\sqstate.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\squirrel\sqstring.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\squirrel\sqstruct.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\squirrel\sqtable.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\squirrel\squserdata.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\squirrel\sqvector.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\squirrel\sqvm.h"#,
    r#"primedev/primedev/vscript\languages\squirrel_re\vsquirrel.h"#,
    r#"primedev/primedev/vscript\vscript.h"#,
    r#"primedev/primedev/windows\libsys.h"#,
    r#"primedev/primedev/windows\wconsole.h"#,
    r#"primedev/primedev/windows\window.h"#,
];
