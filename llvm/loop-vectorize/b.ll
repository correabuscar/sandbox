; from: https://bugs.llvm.org/show_bug.cgi?id=48340
target triple = "x86_64-unknown-linux-gnu"
attributes #0 = { "target-cpu"="skylake" } ; fail

%0 = type { i32 }
%1 = type { i64 }

define void @a() unnamed_addr #0 {
  br label %1
1:                                                ; preds = %1, %0
  %2 = phi i64* [ undef, %0 ], [ %3, %1 ]
  %3 = getelementptr inbounds i64, i64* %2, i64 2
  %4 = bitcast i64* %2 to %0**
  %5 = load %0*, %0** %4, align 8
  %6 = icmp eq i64* %3, undef
  br i1 %6, label %7, label %1
7:                                                ; preds = %1
  ret void
}

define void @b() unnamed_addr #0 {
  br label %1
1:                                                ; preds = %1, %0
  %2 = phi i64* [ undef, %0 ], [ %3, %1 ]
  %3 = getelementptr inbounds i64, i64* %2, i64 2
  %4 = bitcast i64* %2 to %1**
  %5 = load %1*, %1** %4, align 8
  %6 = icmp eq i64* %3, undef
  br i1 %6, label %7, label %1
7:                                                ; preds = %1
  ret void
}
