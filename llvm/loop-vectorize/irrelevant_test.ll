; src: https://reviews.llvm.org/D54538
; cannot vectorize loop with unsafe dependency between uniform load (%tmp10) and store
; (%tmp12) to the same address
; PR39653
; Note: %tmp10 could be replaced by phi(%arg4, %tmp12), a potentially vectorizable
; 1st-order-recurrence
define void @unsafe_dep_uniform_load_store(i32 %arg, i32 %arg1, i64 %arg2, i16* %arg3, i32 %arg4, i64 %arg5) {
; CHECK-LABEL: unsafe_dep_uniform_load_store
; CHECK-NOT: <4 x i32>
bb:
  %tmp = alloca i32
  store i32 %arg4, i32* %tmp
  %tmp6 = getelementptr inbounds i16, i16* %arg3, i64 %arg5
  br label %bb7

bb7:
  %tmp8 = phi i64 [ 0, %bb ], [ %tmp24, %bb7 ]
  %tmp9 = phi i32 [ %arg1, %bb ], [ %tmp23, %bb7 ]
  %tmp10 = load i32, i32* %tmp
  %tmp11 = mul nsw i32 %tmp9, %tmp10
  %tmp12 = srem i32 %tmp11, 65536
  %tmp13 = add nsw i32 %tmp12, %tmp9
  %tmp14 = trunc i32 %tmp13 to i16
  %tmp15 = trunc i64 %tmp8 to i32
  %tmp16 = add i32 %arg, %tmp15
  %tmp17 = zext i32 %tmp16 to i64
  %tmp18 = getelementptr inbounds i16, i16* %tmp6, i64 %tmp17
  store i16 %tmp14, i16* %tmp18, align 2
  %tmp19 = add i32 %tmp13, %tmp9
  %tmp20 = trunc i32 %tmp19 to i16
  %tmp21 = and i16 %tmp20, 255
  %tmp22 = getelementptr inbounds i16, i16* %arg3, i64 %tmp17
  store i16 %tmp21, i16* %tmp22, align 2
  %tmp23 = add nsw i32 %tmp9, 1
  %tmp24 = add nuw nsw i64 %tmp8, 1
  %tmp25 = icmp eq i64 %tmp24, %arg2
  store i32 %tmp12, i32* %tmp
  br i1 %tmp25, label %bb26, label %bb7

bb26:
  ret void
}
