AS=as

TARGET=../target
BUILD=$(TARGET)/build

SRCS = day01.s # day01_p2.s
BINS = $(SRCS:%.s=%)
TARGETS = $(BINS:%=$(TARGET)/%)

all: $(TARGETS)

debug_print:
	@echo srcs=$(SRCS)
	@echo targets=$(TARGETS)

$(TARGET) $(BUILD):
	mkdir -p $@

$(BUILD)/%.o : %.s $(BUILD) 
	$(AS) -g $< -o $@

$(TARGET)/day01 : $(BUILD)/day01.o $(BUILD)/common.o
	ld -dynamic-linker /lib64/ld-linux-x86-64.so.2  -o $@ $< $(BUILD)/common.o  -lc
