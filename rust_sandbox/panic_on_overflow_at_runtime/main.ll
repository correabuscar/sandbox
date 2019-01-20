; ModuleID = 'main.7rcbfp3g-cgu.0'
source_filename = "main.7rcbfp3g-cgu.0"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::fmt::Formatter" = type { [0 x i64], { i64, i64 }, [0 x i64], { i64, i64 }, [0 x i64], { {}*, [3 x i64]* }, [0 x i64], { i64*, i64* }, [0 x i64], { [0 x { i8*, i8* }]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i8], i8, [7 x i8] }
%"core::fmt::Void" = type { [0 x i8], {}, [0 x i8], %"core::marker::PhantomData<*mut core::ops::function::Fn<(), Output=()>>", [0 x i8] }
%"core::marker::PhantomData<*mut core::ops::function::Fn<(), Output=()>>" = type {}
%"core::fmt::Arguments" = type { [0 x i64], { [0 x { [0 x i8]*, i64 }]*, i64 }, [0 x i64], { i64*, i64 }, [0 x i64], { [0 x { i8*, i8* }]*, i64 }, [0 x i64] }
%"core::fmt::rt::v1::Argument" = type { [0 x i64], { i64, i64 }, [0 x i64], %"core::fmt::rt::v1::FormatSpec", [0 x i64] }
%"core::fmt::rt::v1::FormatSpec" = type { [0 x i64], { i64, i64 }, [0 x i64], { i64, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i8], i8, [7 x i8] }
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@vtable.0 = private unnamed_addr constant { void (i8**)*, i64, i64, i32 (i8**)*, i32 (i8**)*, i32 (i8**)* } { void (i8**)* @_ZN4core3ptr18real_drop_in_place17h90c7b6ca8f2d7accE, i64 8, i64 8, i32 (i8**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he828a9bd8659b11aE", i32 (i8**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he828a9bd8659b11aE", i32 (i8**)* @"_ZN4core3ops8function6FnOnce9call_once32_$u7b$$u7b$vtable.shim$u7d$$u7d$17h00bb45de9c0b046aE" }, align 8, !dbg !0
@0 = private unnamed_addr constant <{ [0 x i8] }> zeroinitializer, align 1
@1 = private unnamed_addr constant <{ [3 x i8] }> <{ [3 x i8] c" / " }>, align 1
@2 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@3 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [8 x i8], i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [0 x i8] }>, <{ [0 x i8] }>* @0, i32 0, i32 0, i32 0), [8 x i8] zeroinitializer, i8* getelementptr inbounds (<{ [3 x i8] }>, <{ [3 x i8] }>* @1, i32 0, i32 0, i32 0), [8 x i8] c"\03\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [1 x i8] }>, <{ [1 x i8] }>* @2, i32 0, i32 0, i32 0), [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@4 = private unnamed_addr constant <{ [128 x i8] }> <{ [128 x i8] c"\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\03\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\03\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00 \00\00\00\00\00\00\00\03\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\03\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\03\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00 \00\00\00\00\00\00\00\03\00\00\00\00\00\00\00" }>, align 8
@str.1 = internal constant [13 x i8] c"./src/main.rs"
@str.2 = internal constant [33 x i8] c"attempt to multiply with overflow"
@panic_loc.3 = private unnamed_addr constant { { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 } { { [0 x i8]*, i64 } { [0 x i8]* bitcast ([33 x i8]* @str.2 to [0 x i8]*), i64 33 }, { [0 x i8]*, i64 } { [0 x i8]* bitcast ([13 x i8]* @str.1 to [0 x i8]*), i64 13 }, i32 13, i32 15 }, align 8
@__rustc_debug_gdb_scripts_section__ = linkonce_odr unnamed_addr constant [34 x i8] c"\01gdb_load_rust_pretty_printers.py\00", section ".debug_gdb_scripts", align 1

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden i64 @_ZN3std2rt10lang_start17he8f5f9e82f478b04E(void ()* nonnull, i64, i8**) unnamed_addr #0 !dbg !34 {
start:
  %_7 = alloca i8*, align 8
  %argv = alloca i8**, align 8
  %argc = alloca i64, align 8
  %main = alloca void ()*, align 8
  store void ()* %0, void ()** %main, align 8
  call void @llvm.dbg.declare(metadata void ()** %main, metadata !46, metadata !DIExpression()), !dbg !48
  store i64 %1, i64* %argc, align 8
  call void @llvm.dbg.declare(metadata i64* %argc, metadata !49, metadata !DIExpression()), !dbg !48
  store i8** %2, i8*** %argv, align 8
  call void @llvm.dbg.declare(metadata i8*** %argv, metadata !50, metadata !DIExpression()), !dbg !48
  %3 = bitcast i8** %_7 to void ()**, !dbg !51
  %4 = load void ()*, void ()** %main, align 8, !dbg !51, !nonnull !4
  store void ()* %4, void ()** %3, align 8, !dbg !51
  %5 = bitcast i8** %_7 to {}*, !dbg !52
  %6 = load i64, i64* %argc, align 8, !dbg !53
  %7 = load i8**, i8*** %argv, align 8, !dbg !54
; call std::rt::lang_start_internal
  %8 = call i64 @_ZN3std2rt19lang_start_internal17h571775945f864cd1E({}* nonnull align 1 %5, [3 x i64]* noalias readonly align 8 dereferenceable(24) bitcast ({ void (i8**)*, i64, i64, i32 (i8**)*, i32 (i8**)*, i32 (i8**)* }* @vtable.0 to [3 x i64]*), i64 %6, i8** %7), !dbg !55
  br label %bb1, !dbg !55

bb1:                                              ; preds = %start
  ret i64 %8, !dbg !56
}

; std::rt::lang_start::{{closure}}
; Function Attrs: nonlazybind uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he828a9bd8659b11aE"(i8** noalias readonly align 8 dereferenceable(8)) unnamed_addr #0 !dbg !57 {
start:
  %arg0 = alloca i8**, align 8
  store i8** %0, i8*** %arg0, align 8
  call void @llvm.dbg.declare(metadata i8*** %arg0, metadata !63, metadata !DIExpression(DW_OP_deref, DW_OP_plus_uconst, 0)), !dbg !64
  %1 = load i8**, i8*** %arg0, align 8, !dbg !65, !nonnull !4
  %2 = bitcast i8** %1 to void ()**, !dbg !65
  %3 = load void ()*, void ()** %2, align 8, !dbg !65, !nonnull !4
  call void %3(), !dbg !65
  br label %bb1, !dbg !65

bb1:                                              ; preds = %start
; call <() as std::process::Termination>::report
  %4 = call i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h7fcb6e908fc010e9E"(), !dbg !65
  br label %bb2, !dbg !65

bb2:                                              ; preds = %bb1
  ret i32 %4, !dbg !66
}

; std::sys::unix::process::process_common::ExitCode::as_i32
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217ha49dfef13433d42eE(i8* noalias readonly align 1 dereferenceable(1)) unnamed_addr #1 !dbg !67 {
start:
  %self = alloca i8*, align 8
  store i8* %0, i8** %self, align 8
  call void @llvm.dbg.declare(metadata i8** %self, metadata !79, metadata !DIExpression()), !dbg !80
  %1 = load i8*, i8** %self, align 8, !dbg !81, !nonnull !4
  %2 = load i8, i8* %1, align 1, !dbg !81
  %3 = zext i8 %2 to i32, !dbg !81
  ret i32 %3, !dbg !82
}

; core::fmt::ArgumentV1::new
; Function Attrs: nonlazybind uwtable
define internal { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17ha07a9e20c23a5afcE(i32* noalias readonly align 4 dereferenceable(4), i1 (i32*, %"core::fmt::Formatter"*)* nonnull) unnamed_addr #0 !dbg !83 {
start:
  %transmute_temp1 = alloca %"core::fmt::Void"*, align 8
  %transmute_temp = alloca i1 (%"core::fmt::Void"*, %"core::fmt::Formatter"*)*, align 8
  %_0 = alloca { i8*, i8* }, align 8
  %f = alloca i1 (i32*, %"core::fmt::Formatter"*)*, align 8
  %x = alloca i32*, align 8
  store i32* %0, i32** %x, align 8
  call void @llvm.dbg.declare(metadata i32** %x, metadata !170, metadata !DIExpression()), !dbg !171
  store i1 (i32*, %"core::fmt::Formatter"*)* %1, i1 (i32*, %"core::fmt::Formatter"*)** %f, align 8
  call void @llvm.dbg.declare(metadata i1 (i32*, %"core::fmt::Formatter"*)** %f, metadata !172, metadata !DIExpression()), !dbg !171
  %2 = load i1 (i32*, %"core::fmt::Formatter"*)*, i1 (i32*, %"core::fmt::Formatter"*)** %f, align 8, !dbg !173, !nonnull !4
  %3 = bitcast i1 (%"core::fmt::Void"*, %"core::fmt::Formatter"*)** %transmute_temp to i1 (i32*, %"core::fmt::Formatter"*)**, !dbg !175
  store i1 (i32*, %"core::fmt::Formatter"*)* %2, i1 (i32*, %"core::fmt::Formatter"*)** %3, align 8, !dbg !175
  %4 = load i1 (%"core::fmt::Void"*, %"core::fmt::Formatter"*)*, i1 (%"core::fmt::Void"*, %"core::fmt::Formatter"*)** %transmute_temp, align 8, !dbg !175, !nonnull !4
  br label %bb1, !dbg !175

bb1:                                              ; preds = %start
  %5 = load i32*, i32** %x, align 8, !dbg !176, !nonnull !4
  %6 = bitcast %"core::fmt::Void"** %transmute_temp1 to i32**, !dbg !177
  store i32* %5, i32** %6, align 8, !dbg !177
  %7 = load %"core::fmt::Void"*, %"core::fmt::Void"** %transmute_temp1, align 8, !dbg !177, !nonnull !4
  br label %bb2, !dbg !177

bb2:                                              ; preds = %bb1
  %8 = bitcast { i8*, i8* }* %_0 to %"core::fmt::Void"**, !dbg !178
  store %"core::fmt::Void"* %7, %"core::fmt::Void"** %8, align 8, !dbg !178
  %9 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %_0, i32 0, i32 1, !dbg !178
  %10 = bitcast i8** %9 to i1 (%"core::fmt::Void"*, %"core::fmt::Formatter"*)**, !dbg !178
  store i1 (%"core::fmt::Void"*, %"core::fmt::Formatter"*)* %4, i1 (%"core::fmt::Void"*, %"core::fmt::Formatter"*)** %10, align 8, !dbg !178
  %11 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %_0, i32 0, i32 0, !dbg !179
  %12 = load i8*, i8** %11, align 8, !dbg !179, !nonnull !4
  %13 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %_0, i32 0, i32 1, !dbg !179
  %14 = load i8*, i8** %13, align 8, !dbg !179, !nonnull !4
  %15 = insertvalue { i8*, i8* } undef, i8* %12, 0, !dbg !179
  %16 = insertvalue { i8*, i8* } %15, i8* %14, 1, !dbg !179
  ret { i8*, i8* } %16, !dbg !179
}

; core::fmt::num::<impl core::fmt::Debug for i32>::fmt
; Function Attrs: inlinehint nonlazybind uwtable
define internal zeroext i1 @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17h2b19ce393e6a9113E"(i32* noalias readonly align 4 dereferenceable(4), %"core::fmt::Formatter"* align 8 dereferenceable(96)) unnamed_addr #1 !dbg !180 {
start:
  %_0 = alloca i8, align 1
  %f = alloca %"core::fmt::Formatter"*, align 8
  %self = alloca i32*, align 8
  store i32* %0, i32** %self, align 8
  call void @llvm.dbg.declare(metadata i32** %self, metadata !184, metadata !DIExpression()), !dbg !185
  store %"core::fmt::Formatter"* %1, %"core::fmt::Formatter"** %f, align 8
  call void @llvm.dbg.declare(metadata %"core::fmt::Formatter"** %f, metadata !186, metadata !DIExpression()), !dbg !185
  %2 = load %"core::fmt::Formatter"*, %"core::fmt::Formatter"** %f, align 8, !dbg !187, !nonnull !4
; call core::fmt::Formatter::debug_lower_hex
  %3 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h6e09555707e9f39eE(%"core::fmt::Formatter"* noalias readonly align 8 dereferenceable(96) %2), !dbg !187
  br label %bb1, !dbg !187

bb1:                                              ; preds = %start
  br i1 %3, label %bb2, label %bb3, !dbg !188

bb2:                                              ; preds = %bb1
  %4 = load i32*, i32** %self, align 8, !dbg !189, !nonnull !4
  %5 = load %"core::fmt::Formatter"*, %"core::fmt::Formatter"** %f, align 8, !dbg !190, !nonnull !4
; call core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt
  %6 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h536b666d304b32d5E"(i32* noalias readonly align 4 dereferenceable(4) %4, %"core::fmt::Formatter"* align 8 dereferenceable(96) %5), !dbg !191
  %7 = zext i1 %6 to i8, !dbg !191
  store i8 %7, i8* %_0, align 1, !dbg !191
  br label %bb4, !dbg !191

bb3:                                              ; preds = %bb1
  %8 = load %"core::fmt::Formatter"*, %"core::fmt::Formatter"** %f, align 8, !dbg !192, !nonnull !4
; call core::fmt::Formatter::debug_upper_hex
  %9 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17h0522af91563adfeaE(%"core::fmt::Formatter"* noalias readonly align 8 dereferenceable(96) %8), !dbg !192
  br label %bb5, !dbg !192

bb4:                                              ; preds = %bb2
  br label %bb11, !dbg !188

bb5:                                              ; preds = %bb3
  br i1 %9, label %bb6, label %bb7, !dbg !193

bb6:                                              ; preds = %bb5
  %10 = load i32*, i32** %self, align 8, !dbg !194, !nonnull !4
  %11 = load %"core::fmt::Formatter"*, %"core::fmt::Formatter"** %f, align 8, !dbg !195, !nonnull !4
; call core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt
  %12 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17h67725b4bce673886E"(i32* noalias readonly align 4 dereferenceable(4) %10, %"core::fmt::Formatter"* align 8 dereferenceable(96) %11), !dbg !196
  %13 = zext i1 %12 to i8, !dbg !196
  store i8 %13, i8* %_0, align 1, !dbg !196
  br label %bb8, !dbg !196

bb7:                                              ; preds = %bb5
  %14 = load i32*, i32** %self, align 8, !dbg !197, !nonnull !4
  %15 = load %"core::fmt::Formatter"*, %"core::fmt::Formatter"** %f, align 8, !dbg !198, !nonnull !4
; call core::fmt::num::<impl core::fmt::Display for i32>::fmt
  %16 = call zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hc59d86480a2a32acE"(i32* noalias readonly align 4 dereferenceable(4) %14, %"core::fmt::Formatter"* align 8 dereferenceable(96) %15), !dbg !199
  %17 = zext i1 %16 to i8, !dbg !199
  store i8 %17, i8* %_0, align 1, !dbg !199
  br label %bb9, !dbg !199

bb8:                                              ; preds = %bb6
  br label %bb10, !dbg !193

bb9:                                              ; preds = %bb7
  br label %bb10, !dbg !193

bb10:                                             ; preds = %bb9, %bb8
  br label %bb11, !dbg !188

bb11:                                             ; preds = %bb10, %bb4
  %18 = load i8, i8* %_0, align 1, !dbg !200, !range !201
  %19 = trunc i8 %18 to i1, !dbg !200
  ret i1 %19, !dbg !200
}

; core::fmt::Arguments::new_v1_formatted
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3fmt9Arguments16new_v1_formatted17hf39008b85d5fa536E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48), [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8, i64, [0 x { i8*, i8* }]* noalias nonnull readonly align 8, i64, [0 x %"core::fmt::rt::v1::Argument"]* noalias nonnull readonly align 8, i64) unnamed_addr #1 !dbg !202 {
start:
  %_5 = alloca { i64*, i64 }, align 8
  %fmt = alloca { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }, align 8
  %args = alloca { [0 x { i8*, i8* }]*, i64 }, align 8
  %pieces = alloca { [0 x { [0 x i8]*, i64 }]*, i64 }, align 8
  %7 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %pieces, i32 0, i32 0
  store [0 x { [0 x i8]*, i64 }]* %1, [0 x { [0 x i8]*, i64 }]** %7, align 8
  %8 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %pieces, i32 0, i32 1
  store i64 %2, i64* %8, align 8
  call void @llvm.dbg.declare(metadata { [0 x { [0 x i8]*, i64 }]*, i64 }* %pieces, metadata !274, metadata !DIExpression()), !dbg !275
  %9 = getelementptr inbounds { [0 x { i8*, i8* }]*, i64 }, { [0 x { i8*, i8* }]*, i64 }* %args, i32 0, i32 0
  store [0 x { i8*, i8* }]* %3, [0 x { i8*, i8* }]** %9, align 8
  %10 = getelementptr inbounds { [0 x { i8*, i8* }]*, i64 }, { [0 x { i8*, i8* }]*, i64 }* %args, i32 0, i32 1
  store i64 %4, i64* %10, align 8
  call void @llvm.dbg.declare(metadata { [0 x { i8*, i8* }]*, i64 }* %args, metadata !276, metadata !DIExpression()), !dbg !275
  %11 = getelementptr inbounds { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }, { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }* %fmt, i32 0, i32 0
  store [0 x %"core::fmt::rt::v1::Argument"]* %5, [0 x %"core::fmt::rt::v1::Argument"]** %11, align 8
  %12 = getelementptr inbounds { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }, { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }* %fmt, i32 0, i32 1
  store i64 %6, i64* %12, align 8
  call void @llvm.dbg.declare(metadata { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }* %fmt, metadata !277, metadata !DIExpression()), !dbg !275
  %13 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %pieces, i32 0, i32 0, !dbg !278
  %14 = load [0 x { [0 x i8]*, i64 }]*, [0 x { [0 x i8]*, i64 }]** %13, align 8, !dbg !278, !nonnull !4
  %15 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %pieces, i32 0, i32 1, !dbg !278
  %16 = load i64, i64* %15, align 8, !dbg !278
  %17 = getelementptr inbounds { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }, { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }* %fmt, i32 0, i32 0, !dbg !279
  %18 = load [0 x %"core::fmt::rt::v1::Argument"]*, [0 x %"core::fmt::rt::v1::Argument"]** %17, align 8, !dbg !279, !nonnull !4
  %19 = getelementptr inbounds { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }, { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }* %fmt, i32 0, i32 1, !dbg !279
  %20 = load i64, i64* %19, align 8, !dbg !279
  %21 = bitcast { i64*, i64 }* %_5 to { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }*, !dbg !280
  %22 = getelementptr inbounds { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }, { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }* %21, i32 0, i32 0, !dbg !280
  store [0 x %"core::fmt::rt::v1::Argument"]* %18, [0 x %"core::fmt::rt::v1::Argument"]** %22, align 8, !dbg !280
  %23 = getelementptr inbounds { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }, { [0 x %"core::fmt::rt::v1::Argument"]*, i64 }* %21, i32 0, i32 1, !dbg !280
  store i64 %20, i64* %23, align 8, !dbg !280
  %24 = getelementptr inbounds { [0 x { i8*, i8* }]*, i64 }, { [0 x { i8*, i8* }]*, i64 }* %args, i32 0, i32 0, !dbg !281
  %25 = load [0 x { i8*, i8* }]*, [0 x { i8*, i8* }]** %24, align 8, !dbg !281, !nonnull !4
  %26 = getelementptr inbounds { [0 x { i8*, i8* }]*, i64 }, { [0 x { i8*, i8* }]*, i64 }* %args, i32 0, i32 1, !dbg !281
  %27 = load i64, i64* %26, align 8, !dbg !281
  %28 = bitcast %"core::fmt::Arguments"* %0 to { [0 x { [0 x i8]*, i64 }]*, i64 }*, !dbg !282
  %29 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %28, i32 0, i32 0, !dbg !282
  store [0 x { [0 x i8]*, i64 }]* %14, [0 x { [0 x i8]*, i64 }]** %29, align 8, !dbg !282
  %30 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %28, i32 0, i32 1, !dbg !282
  store i64 %16, i64* %30, align 8, !dbg !282
  %31 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 3, !dbg !282
  %32 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_5, i32 0, i32 0, !dbg !282
  %33 = load i64*, i64** %32, align 8, !dbg !282
  %34 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_5, i32 0, i32 1, !dbg !282
  %35 = load i64, i64* %34, align 8, !dbg !282
  %36 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %31, i32 0, i32 0, !dbg !282
  store i64* %33, i64** %36, align 8, !dbg !282
  %37 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %31, i32 0, i32 1, !dbg !282
  store i64 %35, i64* %37, align 8, !dbg !282
  %38 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 5, !dbg !282
  %39 = getelementptr inbounds { [0 x { i8*, i8* }]*, i64 }, { [0 x { i8*, i8* }]*, i64 }* %38, i32 0, i32 0, !dbg !282
  store [0 x { i8*, i8* }]* %25, [0 x { i8*, i8* }]** %39, align 8, !dbg !282
  %40 = getelementptr inbounds { [0 x { i8*, i8* }]*, i64 }, { [0 x { i8*, i8* }]*, i64 }* %38, i32 0, i32 1, !dbg !282
  store i64 %27, i64* %40, align 8, !dbg !282
  ret void, !dbg !283
}

; core::ops::function::FnOnce::call_once
; Function Attrs: nonlazybind uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17ha578c1ab5205ecd5E(i8* nonnull) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !284 {
start:
  %personalityslot = alloca { i8*, i32 }, align 8
  %arg1 = alloca {}, align 1
  %arg0 = alloca i8*, align 8
  store i8* %0, i8** %arg0, align 8
  call void @llvm.dbg.declare(metadata i8** %arg0, metadata !292, metadata !DIExpression()), !dbg !293
  call void @llvm.dbg.declare(metadata {}* %arg1, metadata !294, metadata !DIExpression()), !dbg !293
; invoke std::rt::lang_start::{{closure}}
  %1 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he828a9bd8659b11aE"(i8** align 8 dereferenceable(8) %arg0)
          to label %bb1 unwind label %cleanup, !dbg !295

bb1:                                              ; preds = %start
  br label %bb2, !dbg !295

bb2:                                              ; preds = %bb1
  ret i32 %1, !dbg !295

bb3:                                              ; preds = %cleanup
  br label %bb4, !dbg !295

bb4:                                              ; preds = %bb3
  %2 = bitcast { i8*, i32 }* %personalityslot to i8**, !dbg !295
  %3 = load i8*, i8** %2, align 8, !dbg !295
  %4 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1, !dbg !295
  %5 = load i32, i32* %4, align 8, !dbg !295
  %6 = insertvalue { i8*, i32 } undef, i8* %3, 0, !dbg !295
  %7 = insertvalue { i8*, i32 } %6, i32 %5, 1, !dbg !295
  resume { i8*, i32 } %7, !dbg !295

cleanup:                                          ; preds = %start
  %8 = landingpad { i8*, i32 }
          cleanup
  %9 = extractvalue { i8*, i32 } %8, 0
  %10 = extractvalue { i8*, i32 } %8, 1
  %11 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 0
  store i8* %9, i8** %11, align 8
  %12 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  store i32 %10, i32* %12, align 8
  br label %bb3
}

; core::ops::function::FnOnce::call_once::{{vtable.shim}}
; Function Attrs: nonlazybind uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce9call_once32_$u7b$$u7b$vtable.shim$u7d$$u7d$17h00bb45de9c0b046aE"(i8**) unnamed_addr #0 !dbg !296 {
start:
  %arg1 = alloca {}, align 1
  %arg0 = alloca i8**, align 8
  store i8** %0, i8*** %arg0, align 8
  call void @llvm.dbg.declare(metadata i8*** %arg0, metadata !300, metadata !DIExpression()), !dbg !301
  call void @llvm.dbg.declare(metadata {}* %arg1, metadata !302, metadata !DIExpression()), !dbg !301
  %1 = load i8**, i8*** %arg0, align 8, !dbg !303
  %2 = load i8*, i8** %1, align 8, !dbg !303, !nonnull !4
; call core::ops::function::FnOnce::call_once
  %3 = call i32 @_ZN4core3ops8function6FnOnce9call_once17ha578c1ab5205ecd5E(i8* nonnull %2), !dbg !303
  br label %bb1, !dbg !303

bb1:                                              ; preds = %start
  ret i32 %3, !dbg !303
}

; core::ptr::real_drop_in_place
; Function Attrs: nonlazybind uwtable
define internal void @_ZN4core3ptr18real_drop_in_place17h90c7b6ca8f2d7accE(i8** align 8 dereferenceable(8)) unnamed_addr #0 !dbg !304 {
start:
  %arg0 = alloca i8**, align 8
  store i8** %0, i8*** %arg0, align 8
  call void @llvm.dbg.declare(metadata i8*** %arg0, metadata !312, metadata !DIExpression()), !dbg !313
  ret void, !dbg !314
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h7fcb6e908fc010e9E"() unnamed_addr #1 !dbg !315 {
start:
  %self = alloca {}, align 1
  call void @llvm.dbg.declare(metadata {}* %self, metadata !321, metadata !DIExpression()), !dbg !322
; call <std::process::ExitCode as std::process::Termination>::report
  %0 = call i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17hc13a63113259717bE"(i8 0), !dbg !323
  br label %bb1, !dbg !323

bb1:                                              ; preds = %start
  ret i32 %0, !dbg !324
}

; <std::process::ExitCode as std::process::Termination>::report
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17hc13a63113259717bE"(i8) unnamed_addr #1 !dbg !325 {
start:
  %self = alloca i8, align 1
  store i8 %0, i8* %self, align 1
  call void @llvm.dbg.declare(metadata i8* %self, metadata !331, metadata !DIExpression()), !dbg !332
; call std::sys::unix::process::process_common::ExitCode::as_i32
  %1 = call i32 @_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217ha49dfef13433d42eE(i8* noalias readonly align 1 dereferenceable(1) %self), !dbg !333
  br label %bb1, !dbg !333

bb1:                                              ; preds = %start
  ret i32 %1, !dbg !334
}

; main::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN4main4main17hfe98083a4c87500fE() unnamed_addr #0 !dbg !335 {
start:
  %_c = alloca i8, align 1
  %arg1 = alloca i32*, align 8
  %arg0 = alloca i32*, align 8
  %_13 = alloca { i32*, i32* }, align 8
  %_12 = alloca [2 x { i8*, i8* }], align 8
  %_5 = alloca %"core::fmt::Arguments", align 8
  %b = alloca i32, align 4
  %a = alloca i32, align 4
  %in_num = alloca i8, align 1
  call void @llvm.dbg.declare(metadata i8* %in_num, metadata !337, metadata !DIExpression()), !dbg !340
  call void @llvm.dbg.declare(metadata i32* %a, metadata !341, metadata !DIExpression()), !dbg !343
  call void @llvm.dbg.declare(metadata i32* %b, metadata !344, metadata !DIExpression()), !dbg !346
  call void @llvm.dbg.declare(metadata i32** %arg0, metadata !347, metadata !DIExpression()), !dbg !349
  call void @llvm.dbg.declare(metadata i32** %arg1, metadata !350, metadata !DIExpression()), !dbg !349
  call void @llvm.dbg.declare(metadata i8* %_c, metadata !351, metadata !DIExpression()), !dbg !353
  store i8 122, i8* %in_num, align 1, !dbg !354
  store i32 0, i32* %a, align 4, !dbg !355
  store i32 0, i32* %b, align 4, !dbg !356
  %0 = bitcast { i32*, i32* }* %_13 to i32**, !dbg !357
  store i32* %a, i32** %0, align 8, !dbg !357
  %1 = getelementptr inbounds { i32*, i32* }, { i32*, i32* }* %_13, i32 0, i32 1, !dbg !357
  store i32* %b, i32** %1, align 8, !dbg !357
  %2 = bitcast { i32*, i32* }* %_13 to i32**, !dbg !357
  %3 = load i32*, i32** %2, align 8, !dbg !357, !nonnull !4
  store i32* %3, i32** %arg0, align 8, !dbg !357
  %4 = getelementptr inbounds { i32*, i32* }, { i32*, i32* }* %_13, i32 0, i32 1, !dbg !357
  %5 = load i32*, i32** %4, align 8, !dbg !357, !nonnull !4
  store i32* %5, i32** %arg1, align 8, !dbg !357
  %6 = load i32*, i32** %arg0, align 8, !dbg !349, !nonnull !4
; call core::fmt::ArgumentV1::new
  %7 = call { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17ha07a9e20c23a5afcE(i32* noalias readonly align 4 dereferenceable(4) %6, i1 (i32*, %"core::fmt::Formatter"*)* nonnull @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17h2b19ce393e6a9113E"), !dbg !349
  %8 = extractvalue { i8*, i8* } %7, 0, !dbg !349
  %9 = extractvalue { i8*, i8* } %7, 1, !dbg !349
  br label %bb1, !dbg !349

bb1:                                              ; preds = %start
  %10 = load i32*, i32** %arg1, align 8, !dbg !349, !nonnull !4
; call core::fmt::ArgumentV1::new
  %11 = call { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17ha07a9e20c23a5afcE(i32* noalias readonly align 4 dereferenceable(4) %10, i1 (i32*, %"core::fmt::Formatter"*)* nonnull @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17h2b19ce393e6a9113E"), !dbg !349
  %12 = extractvalue { i8*, i8* } %11, 0, !dbg !349
  %13 = extractvalue { i8*, i8* } %11, 1, !dbg !349
  br label %bb2, !dbg !349

bb2:                                              ; preds = %bb1
  %14 = bitcast [2 x { i8*, i8* }]* %_12 to { i8*, i8* }*, !dbg !349
  %15 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %14, i32 0, i32 0, !dbg !349
  store i8* %8, i8** %15, align 8, !dbg !349
  %16 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %14, i32 0, i32 1, !dbg !349
  store i8* %9, i8** %16, align 8, !dbg !349
  %17 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_12, i32 0, i32 1, !dbg !349
  %18 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %17, i32 0, i32 0, !dbg !349
  store i8* %12, i8** %18, align 8, !dbg !349
  %19 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %17, i32 0, i32 1, !dbg !349
  store i8* %13, i8** %19, align 8, !dbg !349
  %20 = bitcast [2 x { i8*, i8* }]* %_12 to [0 x { i8*, i8* }]*, !dbg !357
; call core::fmt::Arguments::new_v1_formatted
  call void @_ZN4core3fmt9Arguments16new_v1_formatted17hf39008b85d5fa536E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %_5, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 bitcast (<{ i8*, [8 x i8], i8*, [8 x i8], i8*, [8 x i8] }>* @3 to [0 x { [0 x i8]*, i64 }]*), i64 3, [0 x { i8*, i8* }]* noalias nonnull readonly align 8 %20, i64 2, [0 x %"core::fmt::rt::v1::Argument"]* noalias nonnull readonly align 8 bitcast (<{ [128 x i8] }>* @4 to [0 x %"core::fmt::rt::v1::Argument"]*), i64 2), !dbg !357
  br label %bb3, !dbg !357

bb3:                                              ; preds = %bb2
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17h9def9990a4e8710aE(%"core::fmt::Arguments"* noalias nocapture dereferenceable(48) %_5), !dbg !357
  br label %bb4, !dbg !357

bb4:                                              ; preds = %bb3
  %21 = load i8, i8* %in_num, align 1, !dbg !358
  %22 = call { i8, i1 } @llvm.smul.with.overflow.i8(i8 2, i8 %21), !dbg !359
  %23 = extractvalue { i8, i1 } %22, 0, !dbg !359
  %24 = extractvalue { i8, i1 } %22, 1, !dbg !359
  %25 = call i1 @llvm.expect.i1(i1 %24, i1 false), !dbg !359
  br i1 %25, label %panic, label %bb5, !dbg !359

bb5:                                              ; preds = %bb4
  store i8 %23, i8* %_c, align 1, !dbg !359
  ret void, !dbg !360

panic:                                            ; preds = %bb4
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h14b246061a55fbfaE({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast ({ { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 }* @panic_loc.3 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*)), !dbg !359
  unreachable, !dbg !359
}

; Function Attrs: nounwind readnone speculatable
declare void @llvm.dbg.declare(metadata, metadata, metadata) #2

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare i64 @_ZN3std2rt19lang_start_internal17h571775945f864cd1E({}* nonnull align 1, [3 x i64]* noalias readonly align 8 dereferenceable(24), i64, i8**) unnamed_addr #0

; core::fmt::Formatter::debug_lower_hex
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h6e09555707e9f39eE(%"core::fmt::Formatter"* noalias readonly align 8 dereferenceable(96)) unnamed_addr #0

; core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h536b666d304b32d5E"(i32* noalias readonly align 4 dereferenceable(4), %"core::fmt::Formatter"* align 8 dereferenceable(96)) unnamed_addr #0

; core::fmt::Formatter::debug_upper_hex
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17h0522af91563adfeaE(%"core::fmt::Formatter"* noalias readonly align 8 dereferenceable(96)) unnamed_addr #0

; core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17h67725b4bce673886E"(i32* noalias readonly align 4 dereferenceable(4), %"core::fmt::Formatter"* align 8 dereferenceable(96)) unnamed_addr #0

; core::fmt::num::<impl core::fmt::Display for i32>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hc59d86480a2a32acE"(i32* noalias readonly align 4 dereferenceable(4), %"core::fmt::Formatter"* align 8 dereferenceable(96)) unnamed_addr #0

; Function Attrs: nounwind nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #3

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17h9def9990a4e8710aE(%"core::fmt::Arguments"* noalias nocapture dereferenceable(48)) unnamed_addr #0

; Function Attrs: nounwind readnone speculatable
declare { i8, i1 } @llvm.smul.with.overflow.i8(i8, i8) #2

; Function Attrs: nounwind readnone
declare i1 @llvm.expect.i1(i1, i1) #4

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17h14b246061a55fbfaE({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40)) unnamed_addr #5

; Function Attrs: nonlazybind
define i32 @main(i32, i8**) unnamed_addr #6 {
top:
  %2 = load volatile i8, i8* getelementptr inbounds ([34 x i8], [34 x i8]* @__rustc_debug_gdb_scripts_section__, i32 0, i32 0), align 1
  %3 = sext i32 %0 to i64
; call std::rt::lang_start
  %4 = call i64 @_ZN3std2rt10lang_start17he8f5f9e82f478b04E(void ()* @_ZN4main4main17hfe98083a4c87500fE, i64 %3, i8** %1)
  %5 = trunc i64 %4 to i32
  ret i32 %5
}

attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { nounwind readnone speculatable }
attributes #3 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #4 = { nounwind readnone }
attributes #5 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #6 = { nonlazybind "target-cpu"="x86-64" }

!llvm.module.flags = !{!11, !12, !13}
!llvm.dbg.cu = !{!14}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "vtable", scope: null, file: !2, type: !3, isLocal: true, isDefinition: true)
!2 = !DIFile(filename: "<unknown>", directory: "")
!3 = !DICompositeType(tag: DW_TAG_structure_type, name: "vtable", file: !2, align: 64, flags: DIFlagArtificial, elements: !4, vtableHolder: !5, identifier: "vtable")
!4 = !{}
!5 = !DICompositeType(tag: DW_TAG_structure_type, name: "closure", file: !2, size: 64, align: 64, elements: !6, templateParams: !4, identifier: "131d83552f04fe27c49dcd7641d206")
!6 = !{!7}
!7 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !5, file: !2, baseType: !8, size: 64, align: 64)
!8 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn()", baseType: !9, size: 64, align: 64)
!9 = !DISubroutineType(types: !10)
!10 = !{null}
!11 = !{i32 7, !"PIE Level", i32 2}
!12 = !{i32 2, !"RtLibUseGOT", i32 1}
!13 = !{i32 2, !"Debug Info Version", i32 3}
!14 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !15, producer: "clang LLVM (rustc version 1.33.0-dev (daa53a52a 2019-01-17))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !16, globals: !33)
!15 = !DIFile(filename: "./src/main.rs", directory: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustlearnage/rust_sandbox/panic_on_overflow_at_runtime")
!16 = !{!17, !24}
!17 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Result", scope: !18, file: !2, baseType: !20, size: 8, align: 8, flags: DIFlagFixedEnum, elements: !21)
!18 = !DINamespace(name: "result", scope: !19)
!19 = !DINamespace(name: "core", scope: null)
!20 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!21 = !{!22, !23}
!22 = !DIEnumerator(name: "Ok", value: 0)
!23 = !DIEnumerator(name: "Err", value: 1)
!24 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Alignment", scope: !25, file: !2, baseType: !20, size: 8, align: 8, flags: DIFlagFixedEnum, elements: !28)
!25 = !DINamespace(name: "v1", scope: !26)
!26 = !DINamespace(name: "rt", scope: !27)
!27 = !DINamespace(name: "fmt", scope: !19)
!28 = !{!29, !30, !31, !32}
!29 = !DIEnumerator(name: "Left", value: 0)
!30 = !DIEnumerator(name: "Right", value: 1)
!31 = !DIEnumerator(name: "Center", value: 2)
!32 = !DIEnumerator(name: "Unknown", value: 3)
!33 = !{!0}
!34 = distinct !DISubprogram(name: "lang_start<()>", linkageName: "_ZN3std2rt10lang_start17he8f5f9e82f478b04E", scope: !36, file: !35, line: 61, type: !38, isLocal: true, isDefinition: true, scopeLine: 61, flags: DIFlagPrototyped, isOptimized: false, unit: !14, templateParams: !43, retainedNodes: !4)
!35 = !DIFile(filename: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/rt.rs", directory: "")
!36 = !DINamespace(name: "rt", scope: !37)
!37 = !DINamespace(name: "std", scope: null)
!38 = !DISubroutineType(types: !39)
!39 = !{!40, !8, !40, !41}
!40 = !DIBasicType(name: "isize", size: 64, encoding: DW_ATE_signed)
!41 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *const u8", baseType: !42, size: 64, align: 64)
!42 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !20, size: 64, align: 64)
!43 = !{!44}
!44 = !DITemplateTypeParameter(name: "T", type: !45)
!45 = !DIBasicType(name: "()", encoding: DW_ATE_unsigned)
!46 = !DILocalVariable(name: "main", arg: 1, scope: !34, file: !47, line: 1, type: !8)
!47 = !DIFile(filename: "./src/main.rs", directory: "")
!48 = !DILocation(line: 1, scope: !34)
!49 = !DILocalVariable(name: "argc", arg: 2, scope: !34, file: !47, line: 1, type: !40)
!50 = !DILocalVariable(name: "argv", arg: 3, scope: !34, file: !47, line: 1, type: !41)
!51 = !DILocation(line: 64, column: 25, scope: !34)
!52 = !DILocation(line: 64, column: 24, scope: !34)
!53 = !DILocation(line: 64, column: 50, scope: !34)
!54 = !DILocation(line: 64, column: 56, scope: !34)
!55 = !DILocation(line: 64, column: 4, scope: !34)
!56 = !DILocation(line: 65, column: 1, scope: !34)
!57 = distinct !DISubprogram(name: "{{closure}}<()>", linkageName: "_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he828a9bd8659b11aE", scope: !58, file: !35, line: 64, type: !59, isLocal: true, isDefinition: true, scopeLine: 64, flags: DIFlagPrototyped, isOptimized: false, unit: !14, templateParams: !43, retainedNodes: !4)
!58 = !DINamespace(name: "lang_start", scope: !36)
!59 = !DISubroutineType(types: !60)
!60 = !{!61, !62}
!61 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!62 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&closure", baseType: !5, size: 64, align: 64)
!63 = !DILocalVariable(name: "main", scope: !57, file: !47, line: 1, type: !8, align: 8)
!64 = !DILocation(line: 1, scope: !57)
!65 = !DILocation(line: 64, column: 33, scope: !57)
!66 = !DILocation(line: 64, column: 48, scope: !57)
!67 = distinct !DISubprogram(name: "as_i32", linkageName: "_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217ha49dfef13433d42eE", scope: !69, file: !68, line: 398, type: !76, isLocal: true, isDefinition: true, scopeLine: 398, flags: DIFlagPrototyped, isOptimized: false, unit: !14, templateParams: !4, retainedNodes: !4)
!68 = !DIFile(filename: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/sys/unix/process/process_common.rs", directory: "")
!69 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !70, file: !2, size: 8, align: 8, elements: !74, templateParams: !4, identifier: "e58ae212ecf4ec40cffe100c684b5442")
!70 = !DINamespace(name: "process_common", scope: !71)
!71 = !DINamespace(name: "process", scope: !72)
!72 = !DINamespace(name: "unix", scope: !73)
!73 = !DINamespace(name: "sys", scope: !37)
!74 = !{!75}
!75 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !69, file: !2, baseType: !20, size: 8, align: 8)
!76 = !DISubroutineType(types: !77)
!77 = !{!61, !78}
!78 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::sys::unix::process::process_common::ExitCode", baseType: !69, size: 64, align: 64)
!79 = !DILocalVariable(name: "self", arg: 1, scope: !67, file: !47, line: 1, type: !78)
!80 = !DILocation(line: 1, scope: !67)
!81 = !DILocation(line: 399, column: 8, scope: !67)
!82 = !DILocation(line: 400, column: 5, scope: !67)
!83 = distinct !DISubprogram(name: "new<i32>", linkageName: "_ZN4core3fmt10ArgumentV13new17ha07a9e20c23a5afcE", scope: !85, file: !84, line: 269, type: !162, isLocal: true, isDefinition: true, scopeLine: 269, flags: DIFlagPrototyped, isOptimized: false, unit: !14, templateParams: !168, retainedNodes: !4)
!84 = !DIFile(filename: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libcore/fmt/mod.rs", directory: "")
!85 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentV1", scope: !27, file: !2, size: 128, align: 64, elements: !86, templateParams: !4, identifier: "5e18274617ade21ced14d10ef163ba17")
!86 = !{!87, !109}
!87 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !85, file: !2, baseType: !88, size: 64, align: 64)
!88 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::fmt::Void", baseType: !89, size: 64, align: 64)
!89 = !DICompositeType(tag: DW_TAG_structure_type, name: "Void", scope: !27, file: !2, align: 8, elements: !90, templateParams: !4, identifier: "cc2b21611175c49275430a0fdd310e45")
!90 = !{!91, !92}
!91 = !DIDerivedType(tag: DW_TAG_member, name: "_priv", scope: !89, file: !2, baseType: !45, align: 8)
!92 = !DIDerivedType(tag: DW_TAG_member, name: "_oibit_remover", scope: !89, file: !2, baseType: !93, align: 8)
!93 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<*mut Fn<()>>", scope: !94, file: !2, align: 8, elements: !4, templateParams: !95, identifier: "74e06a68ebaa3ab1b76ae2842be3300c")
!94 = !DINamespace(name: "marker", scope: !19)
!95 = !{!96}
!96 = !DITemplateTypeParameter(name: "T", type: !97)
!97 = !DICompositeType(tag: DW_TAG_structure_type, name: "*mut Fn<()>", scope: !98, file: !2, size: 128, align: 64, elements: !100, templateParams: !4, identifier: "903f27233fd787a29974e07c148b7100")
!98 = !DINamespace(name: "function", scope: !99)
!99 = !DINamespace(name: "ops", scope: !19)
!100 = !{!101, !103}
!101 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !97, file: !2, baseType: !102, size: 64, align: 64, flags: DIFlagArtificial)
!102 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u8", baseType: !20, size: 64, align: 64)
!103 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !97, file: !2, baseType: !104, size: 64, align: 64, offset: 64, flags: DIFlagArtificial)
!104 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 3]", baseType: !105, size: 64, align: 64)
!105 = !DICompositeType(tag: DW_TAG_array_type, baseType: !106, size: 192, align: 64, elements: !107)
!106 = !DIBasicType(name: "usize", size: 64, encoding: DW_ATE_unsigned)
!107 = !{!108}
!108 = !DISubrange(count: 3)
!109 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !85, file: !2, baseType: !110, size: 64, align: 64, offset: 64)
!110 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&core::fmt::Void, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !111, size: 64, align: 64)
!111 = !DISubroutineType(types: !112)
!112 = !{!17, !88, !113}
!113 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !114, size: 64, align: 64)
!114 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !27, file: !2, size: 768, align: 64, elements: !115, templateParams: !4, identifier: "c77759b178e1bad246e43e6c7a1b6ac5")
!115 = !{!116, !118, !120, !121, !137, !138, !143, !157}
!116 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !114, file: !2, baseType: !117, size: 32, align: 32, offset: 640)
!117 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!118 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !114, file: !2, baseType: !119, size: 32, align: 32, offset: 672)
!119 = !DIBasicType(name: "char", size: 32, encoding: DW_ATE_unsigned_char)
!120 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !114, file: !2, baseType: !24, size: 8, align: 8, offset: 704)
!121 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !114, file: !2, baseType: !122, size: 128, align: 64)
!122 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<usize>", scope: !123, file: !2, size: 128, align: 64, elements: !124, identifier: "f5b3bcaa2d8b93ac6128b1ef67cb4b6")
!123 = !DINamespace(name: "option", scope: !19)
!124 = !{!125}
!125 = !DICompositeType(tag: DW_TAG_variant_part, scope: !123, file: !2, size: 128, align: 64, elements: !126, templateParams: !129, identifier: "f5b3bcaa2d8b93ac6128b1ef67cb4b6", discriminator: !135)
!126 = !{!127, !131}
!127 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !125, file: !2, baseType: !128, size: 128, align: 64, extraData: i64 0)
!128 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !122, file: !2, size: 128, align: 64, elements: !4, templateParams: !129, identifier: "f5b3bcaa2d8b93ac6128b1ef67cb4b6::None")
!129 = !{!130}
!130 = !DITemplateTypeParameter(name: "T", type: !106)
!131 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !125, file: !2, baseType: !132, size: 128, align: 64, extraData: i64 1)
!132 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !122, file: !2, size: 128, align: 64, elements: !133, templateParams: !129, identifier: "f5b3bcaa2d8b93ac6128b1ef67cb4b6::Some")
!133 = !{!134}
!134 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !132, file: !2, baseType: !106, size: 64, align: 64, offset: 64)
!135 = !DIDerivedType(tag: DW_TAG_member, scope: !123, file: !2, baseType: !136, size: 64, align: 64, flags: DIFlagArtificial)
!136 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!137 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !114, file: !2, baseType: !122, size: 128, align: 64, offset: 128)
!138 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !114, file: !2, baseType: !139, size: 128, align: 64, offset: 256)
!139 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut Write", scope: !27, file: !2, size: 128, align: 64, elements: !140, templateParams: !4, identifier: "538dbfd5433239e76384a5faeb7a29b2")
!140 = !{!141, !142}
!141 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !139, file: !2, baseType: !102, size: 64, align: 64, flags: DIFlagArtificial)
!142 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !139, file: !2, baseType: !104, size: 64, align: 64, offset: 64, flags: DIFlagArtificial)
!143 = !DIDerivedType(tag: DW_TAG_member, name: "curarg", scope: !114, file: !2, baseType: !144, size: 128, align: 64, offset: 384)
!144 = !DICompositeType(tag: DW_TAG_structure_type, name: "Iter<core::fmt::ArgumentV1>", scope: !145, file: !2, size: 128, align: 64, elements: !146, templateParams: !155, identifier: "87e592eba3a58b20b37977332659b930")
!145 = !DINamespace(name: "slice", scope: !19)
!146 = !{!147, !149, !150}
!147 = !DIDerivedType(tag: DW_TAG_member, name: "ptr", scope: !144, file: !2, baseType: !148, size: 64, align: 64)
!148 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const core::fmt::ArgumentV1", baseType: !85, size: 64, align: 64)
!149 = !DIDerivedType(tag: DW_TAG_member, name: "end", scope: !144, file: !2, baseType: !148, size: 64, align: 64, offset: 64)
!150 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !144, file: !2, baseType: !151, align: 8)
!151 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&core::fmt::ArgumentV1>", scope: !94, file: !2, align: 8, elements: !4, templateParams: !152, identifier: "6c8ab2d60ef01eaad4240f5eaf3140c8")
!152 = !{!153}
!153 = !DITemplateTypeParameter(name: "T", type: !154)
!154 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::fmt::ArgumentV1", baseType: !85, size: 64, align: 64)
!155 = !{!156}
!156 = !DITemplateTypeParameter(name: "T", type: !85)
!157 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !114, file: !2, baseType: !158, size: 128, align: 64, offset: 512)
!158 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::ArgumentV1]", file: !2, size: 128, align: 64, elements: !159, templateParams: !4, identifier: "159098df5e43096fb525b62f61628279")
!159 = !{!160, !161}
!160 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !158, file: !2, baseType: !148, size: 64, align: 64)
!161 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !158, file: !2, baseType: !106, size: 64, align: 64, offset: 64)
!162 = !DISubroutineType(types: !163)
!163 = !{!85, !164, !165}
!164 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&i32", baseType: !61, size: 64, align: 64)
!165 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&i32, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !166, size: 64, align: 64)
!166 = !DISubroutineType(types: !167)
!167 = !{!17, !164, !113}
!168 = !{!169}
!169 = !DITemplateTypeParameter(name: "T", type: !61)
!170 = !DILocalVariable(name: "x", arg: 1, scope: !83, file: !47, line: 1, type: !164)
!171 = !DILocation(line: 1, scope: !83)
!172 = !DILocalVariable(name: "f", arg: 2, scope: !83, file: !47, line: 1, type: !165)
!173 = !DILocation(line: 273, column: 42, scope: !174)
!174 = distinct !DILexicalBlock(scope: !83, file: !84, line: 271, column: 8)
!175 = !DILocation(line: 273, column: 27, scope: !174)
!176 = !DILocation(line: 274, column: 38, scope: !174)
!177 = !DILocation(line: 274, column: 23, scope: !174)
!178 = !DILocation(line: 272, column: 12, scope: !174)
!179 = !DILocation(line: 277, column: 5, scope: !83)
!180 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17h2b19ce393e6a9113E", scope: !182, file: !181, line: 144, type: !166, isLocal: true, isDefinition: true, scopeLine: 144, flags: DIFlagPrototyped, isOptimized: false, unit: !14, templateParams: !4, retainedNodes: !4)
!181 = !DIFile(filename: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libcore/fmt/num.rs", directory: "")
!182 = !DINamespace(name: "{{impl}}", scope: !183)
!183 = !DINamespace(name: "num", scope: !27)
!184 = !DILocalVariable(name: "self", arg: 1, scope: !180, file: !47, line: 1, type: !164)
!185 = !DILocation(line: 1, scope: !180)
!186 = !DILocalVariable(name: "f", arg: 2, scope: !180, file: !47, line: 1, type: !113)
!187 = !DILocation(line: 145, column: 19, scope: !180)
!188 = !DILocation(line: 145, column: 16, scope: !180)
!189 = !DILocation(line: 146, column: 39, scope: !180)
!190 = !DILocation(line: 146, column: 45, scope: !180)
!191 = !DILocation(line: 146, column: 20, scope: !180)
!192 = !DILocation(line: 147, column: 26, scope: !180)
!193 = !DILocation(line: 147, column: 23, scope: !180)
!194 = !DILocation(line: 148, column: 39, scope: !180)
!195 = !DILocation(line: 148, column: 45, scope: !180)
!196 = !DILocation(line: 148, column: 20, scope: !180)
!197 = !DILocation(line: 150, column: 38, scope: !180)
!198 = !DILocation(line: 150, column: 44, scope: !180)
!199 = !DILocation(line: 150, column: 20, scope: !180)
!200 = !DILocation(line: 152, column: 13, scope: !180)
!201 = !{i8 0, i8 2}
!202 = distinct !DISubprogram(name: "new_v1_formatted", linkageName: "_ZN4core3fmt9Arguments16new_v1_formatted17hf39008b85d5fa536E", scope: !203, file: !84, line: 323, type: !272, isLocal: true, isDefinition: true, scopeLine: 323, flags: DIFlagPrototyped, isOptimized: false, unit: !14, templateParams: !4, retainedNodes: !4)
!203 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !27, file: !2, size: 384, align: 64, elements: !204, templateParams: !4, identifier: "f868a333fcdc68afb00d41f34d6c3cce")
!204 = !{!205, !215, !271}
!205 = !DIDerivedType(tag: DW_TAG_member, name: "pieces", scope: !203, file: !2, baseType: !206, size: 128, align: 64)
!206 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[&str]", file: !2, size: 128, align: 64, elements: !207, templateParams: !4, identifier: "6dc4ddb2dbcf2912a5f3983b5bf0572")
!207 = !{!208, !214}
!208 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !206, file: !2, baseType: !209, size: 64, align: 64)
!209 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const &str", baseType: !210, size: 64, align: 64)
!210 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !2, size: 128, align: 64, elements: !211, templateParams: !4, identifier: "111094d970b097647de579f9c509ef08")
!211 = !{!212, !213}
!212 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !210, file: !2, baseType: !42, size: 64, align: 64)
!213 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !210, file: !2, baseType: !106, size: 64, align: 64, offset: 64)
!214 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !206, file: !2, baseType: !106, size: 64, align: 64, offset: 64)
!215 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !203, file: !2, baseType: !216, size: 128, align: 64, offset: 128)
!216 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&[core::fmt::rt::v1::Argument]>", scope: !123, file: !2, size: 128, align: 64, elements: !217, identifier: "ed1c18fe97708514bc2c395ee79b5fb6")
!217 = !{!218}
!218 = !DICompositeType(tag: DW_TAG_variant_part, scope: !123, file: !2, size: 128, align: 64, elements: !219, templateParams: !222, identifier: "ed1c18fe97708514bc2c395ee79b5fb6", discriminator: !135)
!219 = !{!220, !267}
!220 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !218, file: !2, baseType: !221, size: 128, align: 64, extraData: i64 0)
!221 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !216, file: !2, size: 128, align: 64, elements: !4, templateParams: !222, identifier: "ed1c18fe97708514bc2c395ee79b5fb6::None")
!222 = !{!223}
!223 = !DITemplateTypeParameter(name: "T", type: !224)
!224 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::v1::Argument]", file: !2, size: 128, align: 64, elements: !225, templateParams: !4, identifier: "f14718cb2189b8b9fd82564d5b380230")
!225 = !{!226, !266}
!226 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !224, file: !2, baseType: !227, size: 64, align: 64)
!227 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const core::fmt::rt::v1::Argument", baseType: !228, size: 64, align: 64)
!228 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !25, file: !2, size: 512, align: 64, elements: !229, templateParams: !4, identifier: "aa05deb2603ed963f3df1b4616728858")
!229 = !{!230, !242}
!230 = !DIDerivedType(tag: DW_TAG_member, name: "position", scope: !228, file: !2, baseType: !231, size: 128, align: 64)
!231 = !DICompositeType(tag: DW_TAG_structure_type, name: "Position", scope: !25, file: !2, size: 128, align: 64, elements: !232, identifier: "e2413f653bbd1f0d42eb753923cf5ee8")
!232 = !{!233}
!233 = !DICompositeType(tag: DW_TAG_variant_part, scope: !25, file: !2, size: 128, align: 64, elements: !234, templateParams: !4, identifier: "e2413f653bbd1f0d42eb753923cf5ee8", discriminator: !241)
!234 = !{!235, !237}
!235 = !DIDerivedType(tag: DW_TAG_member, name: "Next", scope: !233, file: !2, baseType: !236, size: 128, align: 64, extraData: i64 0)
!236 = !DICompositeType(tag: DW_TAG_structure_type, name: "Next", scope: !231, file: !2, size: 128, align: 64, elements: !4, templateParams: !4, identifier: "e2413f653bbd1f0d42eb753923cf5ee8::Next")
!237 = !DIDerivedType(tag: DW_TAG_member, name: "At", scope: !233, file: !2, baseType: !238, size: 128, align: 64, extraData: i64 1)
!238 = !DICompositeType(tag: DW_TAG_structure_type, name: "At", scope: !231, file: !2, size: 128, align: 64, elements: !239, templateParams: !4, identifier: "e2413f653bbd1f0d42eb753923cf5ee8::At")
!239 = !{!240}
!240 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !238, file: !2, baseType: !106, size: 64, align: 64, offset: 64)
!241 = !DIDerivedType(tag: DW_TAG_member, scope: !25, file: !2, baseType: !136, size: 64, align: 64, flags: DIFlagArtificial)
!242 = !DIDerivedType(tag: DW_TAG_member, name: "format", scope: !228, file: !2, baseType: !243, size: 384, align: 64, offset: 128)
!243 = !DICompositeType(tag: DW_TAG_structure_type, name: "FormatSpec", scope: !25, file: !2, size: 384, align: 64, elements: !244, templateParams: !4, identifier: "f67f91e932d084762031c544a5494060")
!244 = !{!245, !246, !247, !248, !265}
!245 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !243, file: !2, baseType: !119, size: 32, align: 32, offset: 256)
!246 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !243, file: !2, baseType: !24, size: 8, align: 8, offset: 320)
!247 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !243, file: !2, baseType: !117, size: 32, align: 32, offset: 288)
!248 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !243, file: !2, baseType: !249, size: 128, align: 64)
!249 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !25, file: !2, size: 128, align: 64, elements: !250, identifier: "291b503487179e15948bf1b76dc05902")
!250 = !{!251}
!251 = !DICompositeType(tag: DW_TAG_variant_part, scope: !25, file: !2, size: 128, align: 64, elements: !252, templateParams: !4, identifier: "291b503487179e15948bf1b76dc05902", discriminator: !241)
!252 = !{!253, !257, !261, !263}
!253 = !DIDerivedType(tag: DW_TAG_member, name: "Is", scope: !251, file: !2, baseType: !254, size: 128, align: 64, extraData: i64 0)
!254 = !DICompositeType(tag: DW_TAG_structure_type, name: "Is", scope: !249, file: !2, size: 128, align: 64, elements: !255, templateParams: !4, identifier: "291b503487179e15948bf1b76dc05902::Is")
!255 = !{!256}
!256 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !254, file: !2, baseType: !106, size: 64, align: 64, offset: 64)
!257 = !DIDerivedType(tag: DW_TAG_member, name: "Param", scope: !251, file: !2, baseType: !258, size: 128, align: 64, extraData: i64 1)
!258 = !DICompositeType(tag: DW_TAG_structure_type, name: "Param", scope: !249, file: !2, size: 128, align: 64, elements: !259, templateParams: !4, identifier: "291b503487179e15948bf1b76dc05902::Param")
!259 = !{!260}
!260 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !258, file: !2, baseType: !106, size: 64, align: 64, offset: 64)
!261 = !DIDerivedType(tag: DW_TAG_member, name: "NextParam", scope: !251, file: !2, baseType: !262, size: 128, align: 64, extraData: i64 2)
!262 = !DICompositeType(tag: DW_TAG_structure_type, name: "NextParam", scope: !249, file: !2, size: 128, align: 64, elements: !4, templateParams: !4, identifier: "291b503487179e15948bf1b76dc05902::NextParam")
!263 = !DIDerivedType(tag: DW_TAG_member, name: "Implied", scope: !251, file: !2, baseType: !264, size: 128, align: 64, extraData: i64 3)
!264 = !DICompositeType(tag: DW_TAG_structure_type, name: "Implied", scope: !249, file: !2, size: 128, align: 64, elements: !4, templateParams: !4, identifier: "291b503487179e15948bf1b76dc05902::Implied")
!265 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !243, file: !2, baseType: !249, size: 128, align: 64, offset: 128)
!266 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !224, file: !2, baseType: !106, size: 64, align: 64, offset: 64)
!267 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !218, file: !2, baseType: !268, size: 128, align: 64)
!268 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !216, file: !2, size: 128, align: 64, elements: !269, templateParams: !222, identifier: "ed1c18fe97708514bc2c395ee79b5fb6::Some")
!269 = !{!270}
!270 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !268, file: !2, baseType: !224, size: 128, align: 64)
!271 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !203, file: !2, baseType: !158, size: 128, align: 64, offset: 256)
!272 = !DISubroutineType(types: !273)
!273 = !{!203, !206, !158, !224}
!274 = !DILocalVariable(name: "pieces", arg: 1, scope: !202, file: !47, line: 1, type: !206)
!275 = !DILocation(line: 1, scope: !202)
!276 = !DILocalVariable(name: "args", arg: 2, scope: !202, file: !47, line: 1, type: !158)
!277 = !DILocalVariable(name: "fmt", arg: 3, scope: !202, file: !47, line: 1, type: !224)
!278 = !DILocation(line: 327, column: 12, scope: !202)
!279 = !DILocation(line: 328, column: 22, scope: !202)
!280 = !DILocation(line: 328, column: 17, scope: !202)
!281 = !DILocation(line: 329, column: 12, scope: !202)
!282 = !DILocation(line: 326, column: 8, scope: !202)
!283 = !DILocation(line: 331, column: 5, scope: !202)
!284 = distinct !DISubprogram(name: "call_once<closure,()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17ha578c1ab5205ecd5E", scope: !286, file: !285, line: 231, type: !287, isLocal: true, isDefinition: true, scopeLine: 231, flags: DIFlagPrototyped, isOptimized: false, unit: !14, templateParams: !289, retainedNodes: !4)
!285 = !DIFile(filename: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libcore/ops/function.rs", directory: "")
!286 = !DINamespace(name: "FnOnce", scope: !98)
!287 = !DISubroutineType(types: !288)
!288 = !{!61, !5}
!289 = !{!290, !291}
!290 = !DITemplateTypeParameter(name: "Self", type: !5)
!291 = !DITemplateTypeParameter(name: "Args", type: !45)
!292 = !DILocalVariable(arg: 1, scope: !284, file: !47, line: 1, type: !5)
!293 = !DILocation(line: 1, scope: !284)
!294 = !DILocalVariable(arg: 2, scope: !284, file: !47, line: 1, type: !45)
!295 = !DILocation(line: 231, column: 4, scope: !284)
!296 = distinct !DISubprogram(name: "call_once<closure,()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once32_$u7b$$u7b$vtable.shim$u7d$$u7d$17h00bb45de9c0b046aE", scope: !286, file: !285, line: 231, type: !297, isLocal: true, isDefinition: true, scopeLine: 231, flags: DIFlagPrototyped, isOptimized: false, unit: !14, templateParams: !289, retainedNodes: !4)
!297 = !DISubroutineType(types: !298)
!298 = !{!61, !299}
!299 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut closure", baseType: !5, size: 64, align: 64)
!300 = !DILocalVariable(arg: 1, scope: !296, file: !47, line: 1, type: !299)
!301 = !DILocation(line: 1, scope: !296)
!302 = !DILocalVariable(arg: 2, scope: !296, file: !47, line: 1, type: !45)
!303 = !DILocation(line: 231, column: 4, scope: !296)
!304 = distinct !DISubprogram(name: "real_drop_in_place<closure>", linkageName: "_ZN4core3ptr18real_drop_in_place17h90c7b6ca8f2d7accE", scope: !306, file: !305, line: 193, type: !307, isLocal: true, isDefinition: true, scopeLine: 193, flags: DIFlagPrototyped, isOptimized: false, unit: !14, templateParams: !310, retainedNodes: !4)
!305 = !DIFile(filename: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libcore/ptr.rs", directory: "")
!306 = !DINamespace(name: "ptr", scope: !19)
!307 = !DISubroutineType(types: !308)
!308 = !{null, !309}
!309 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut closure", baseType: !5, size: 64, align: 64)
!310 = !{!311}
!311 = !DITemplateTypeParameter(name: "T", type: !5)
!312 = !DILocalVariable(arg: 1, scope: !304, file: !47, line: 1, type: !309)
!313 = !DILocation(line: 1, scope: !304)
!314 = !DILocation(line: 193, scope: !304)
!315 = distinct !DISubprogram(name: "report", linkageName: "_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h7fcb6e908fc010e9E", scope: !317, file: !316, line: 1589, type: !319, isLocal: true, isDefinition: true, scopeLine: 1589, flags: DIFlagPrototyped, isOptimized: false, unit: !14, templateParams: !4, retainedNodes: !4)
!316 = !DIFile(filename: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/process.rs", directory: "")
!317 = !DINamespace(name: "{{impl}}", scope: !318)
!318 = !DINamespace(name: "process", scope: !37)
!319 = !DISubroutineType(types: !320)
!320 = !{!61, !45}
!321 = !DILocalVariable(name: "self", arg: 1, scope: !315, file: !47, line: 1, type: !45)
!322 = !DILocation(line: 1, scope: !315)
!323 = !DILocation(line: 1589, column: 29, scope: !315)
!324 = !DILocation(line: 1589, column: 57, scope: !315)
!325 = distinct !DISubprogram(name: "report", linkageName: "_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17hc13a63113259717bE", scope: !317, file: !316, line: 1619, type: !326, isLocal: true, isDefinition: true, scopeLine: 1619, flags: DIFlagPrototyped, isOptimized: false, unit: !14, templateParams: !4, retainedNodes: !4)
!326 = !DISubroutineType(types: !327)
!327 = !{!61, !328}
!328 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !318, file: !2, size: 8, align: 8, elements: !329, templateParams: !4, identifier: "4ee0af316ff1b359a4ead61a586804a3")
!329 = !{!330}
!330 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !328, file: !2, baseType: !69, size: 8, align: 8)
!331 = !DILocalVariable(name: "self", arg: 1, scope: !325, file: !47, line: 1, type: !328)
!332 = !DILocation(line: 1, scope: !325)
!333 = !DILocation(line: 1620, column: 8, scope: !325)
!334 = !DILocation(line: 1621, column: 5, scope: !325)
!335 = distinct !DISubprogram(name: "main", linkageName: "_ZN4main4main17hfe98083a4c87500fE", scope: !336, file: !15, line: 2, type: !9, isLocal: true, isDefinition: true, scopeLine: 2, flags: DIFlagPrototyped | DIFlagMainSubprogram, isOptimized: false, unit: !14, templateParams: !4, retainedNodes: !4)
!336 = !DINamespace(name: "main", scope: null)
!337 = !DILocalVariable(name: "in_num", scope: !338, file: !15, line: 6, type: !339, align: 1)
!338 = distinct !DILexicalBlock(scope: !335, file: !15, line: 6, column: 4)
!339 = !DIBasicType(name: "i8", size: 8, encoding: DW_ATE_signed)
!340 = !DILocation(line: 6, column: 8, scope: !338)
!341 = !DILocalVariable(name: "a", scope: !342, file: !15, line: 9, type: !61, align: 4)
!342 = distinct !DILexicalBlock(scope: !338, file: !15, line: 9, column: 4)
!343 = !DILocation(line: 9, column: 8, scope: !342)
!344 = !DILocalVariable(name: "b", scope: !345, file: !15, line: 11, type: !61, align: 4)
!345 = distinct !DILexicalBlock(scope: !342, file: !15, line: 11, column: 4)
!346 = !DILocation(line: 11, column: 8, scope: !345)
!347 = !DILocalVariable(name: "arg0", scope: !348, file: !15, line: 12, type: !164, align: 8)
!348 = distinct !DILexicalBlock(scope: !345, file: !15, line: 12, column: 13)
!349 = !DILocation(line: 12, column: 4, scope: !348)
!350 = !DILocalVariable(name: "arg1", scope: !348, file: !15, line: 12, type: !164, align: 8)
!351 = !DILocalVariable(name: "_c", scope: !352, file: !15, line: 13, type: !339, align: 1)
!352 = distinct !DILexicalBlock(scope: !345, file: !15, line: 13, column: 4)
!353 = !DILocation(line: 13, column: 8, scope: !352)
!354 = !DILocation(line: 6, column: 18, scope: !335)
!355 = !DILocation(line: 9, column: 10, scope: !338)
!356 = !DILocation(line: 11, column: 10, scope: !342)
!357 = !DILocation(line: 12, column: 4, scope: !345)
!358 = !DILocation(line: 13, column: 16, scope: !345)
!359 = !DILocation(line: 13, column: 14, scope: !345)
!360 = !DILocation(line: 15, column: 1, scope: !335)
