; from: https://bugs.llvm.org/show_bug.cgi?id=48340
target triple = "x86_64-unknown-linux-gnu"
attributes #0 = { "target-cpu"="skylake" } ; fail
;attributes #0 = { "target-cpu"="haswell" } ; ok
;attributes #0 = { "target-cpu"="skylake-avx512" } ; ok
;attributes #0 = { "target-cpu"="tigerlake" } ; ok
;attributes #0 = { "target-cpu"="tremont" } ; ok
;attributes #0 = { "target-cpu"="westmere" } ; ok
;attributes #0 = { "target-cpu"="x86-64" } ; ok
;attributes #0 = { "target-cpu"="skx" } ; ok
;attributes #0 = { "target-cpu"="silvermont" } ; ok
;attributes #0 = { "target-cpu"="skylake" } ; fail
;attributes #0 = { "target-cpu"="broadwell" } ; ok
;attributes #0 = { "target-cpu"="bonnell" } ; ok
;attributes #0 = { "target-cpu"="cannonlake" } ; ok
;attributes #0 = { "target-cpu"="icelake-client" } ; ok
;attributes #0 = { "target-cpu"="icelake-server" } ; ok
;attributes #0 = { "target-cpu"="cascadelake" } ; ok
;attributes #0 = { "target-cpu"="cooperlake" } ; ok
;attributes #0 = { "target-cpu"="amdfam10" } ; ok
;attributes #0 = { "target-cpu"="ivybridge" } ; ok
;attributes #0 = { "target-cpu"="sandybridge" } ; ok
;attributes #0 = { "target-cpu"="nehalem" } ; ok
;attributes #0 = { "target-cpu"="native" } ; fail because it maps to skylake

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

;define void @aa() unnamed_addr #0 {
;  br label %1
;
;1:                                                ; preds = %1, %0
;  %2 = phi i64* [ undef, %0 ], [ %3, %1 ]
;  %3 = getelementptr inbounds i64, i64* %2, i64 2
;  %4 = bitcast i64* %2 to %0**
;  %5 = load %0*, %0** %4, align 8
;  %6 = icmp eq i64* %3, undef
;  br i1 %6, label %7, label %1
;
;7:                                                ; preds = %1
;  ret void
;}

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
;
;
;define void @bb() unnamed_addr #0 {
;  br label %1
;
;1:                                                ; preds = %1, %0
;  %2 = phi i64* [ undef, %0 ], [ %3, %1 ]
;  %3 = getelementptr inbounds i64, i64* %2, i64 2
;  %4 = bitcast i64* %2 to %1**
;  %5 = load %1*, %1** %4, align 8
;  %6 = icmp eq i64* %3, undef
;  br i1 %6, label %7, label %1
;
;7:                                                ; preds = %1
;  ret void
;}


