AS=as

%.o : %.s
	$(AS) $< -o $@

day01_% : day01_%.o common.o
	ld -dynamic-linker /lib64/ld-linux-x86-64.so.2  -o $@ $^  -lc
