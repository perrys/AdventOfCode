AS=as

TARGET=../target
BUILD=$(TARGET)/build

SRCS = day01_p1.s # day01_p2.s
BINS = $(SRCS:%.s=%)
TARGETS = $(BINS:%=$(TARGET)/%)

all: $(TARGETS)

debug_print:
	@echo srcs=$(SRCS)
	@echo targets=$(TARGETS)

$(TARGET) $(BUILD):
	mkdir -p $@

$(BUILD)/%.o : %.s
	$(AS) -g $< -o $@

$(TARGET)/day01_% : $(BUILD)/day01_%.o $(BUILD)/common.o $(BUILD) 
	ld -dynamic-linker /lib64/ld-linux-x86-64.so.2  -o $@ $< $(BUILD)/common.o  -lc
