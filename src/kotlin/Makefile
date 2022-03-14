.PHONY: fact
fact:build/bin/native/releaseExecutable/dogd.kexe
	./$<

.INTERMEDIATE :build/bin/native/releaseExecutable/dogd.kexe
.PHONY: build/bin/native/releaseExecutable/dogd.kexe
build/bin/native/releaseExecutable/dogd.kexe:
	./gradlew nativeBinaries >/dev/null 2>/dev/null
