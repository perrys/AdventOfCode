AS=as

TARGET=../target
BUILD=$(TARGET)/build

SRCS = $(wildcard day*.s)
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

$(TARGET)/day% : $(BUILD)/day%.o $(BUILD)/common.o
	ld -dynamic-linker /lib64/ld-linux-x86-64.so.2  -o $@ $< $(BUILD)/common.o  -lc
