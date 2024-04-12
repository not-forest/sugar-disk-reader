# Building script for the daemon binary.

# Target architecture
ARCH ?= x86_64

# Local makefile variables
DEBUG ?= 1

# Toolchain config
CC := gcc
LD := ld.bfd
LDFLAGS += -nostdlib --nmagic

# Directories and file definitions.
DAEMON_DIR := $(abspath .)
SRC_DIR := ${DAEMON_DIR}/src
BUILD_DIR := ${DAEMON_DIR}/build
DAEMON_ELF := ${BUILD_DIR}/daemon.elf
DAEMON_BIN := ${BUILD_DIR}/daemon.bin
IMG := ${BUILD_DIR}/daemon.img

# Source files.
SOURCE_C := $(wildcard $(SRC_DIR)/*.c)
SOURCE_ASM := $(wildcard $(SRC_DIR)/asm/*.asm)
SOURCE_LD := $(wildcard $(SRC_DIR)/scripts/*.ld)

# Object files
OBJ_C := $(patsubst $(SRC_DIR)/%.c,$(BUILD_DIR)/%.o,$(SOURCE_C))
OBJ_ASM := $(patsubst $(SRC_DIR)/asm/%.asm,$(BUILD_DIR)/%.o,$(SOURCE_ASM))
OBJECTS := $(OBJ_C) $(OBJ_ASM)

# GDB
GDB_PORT := 1234

# CFLAGS
CFLAGS += -RELEASE -mgeneral-regs-only -c -m32 -D__KERNEL__
CFLAGS += -march=x86-64 -ffreestanding -fomit-frame-pointer -fno-exceptions -fno-pic
CFLAGS += -fno-asynchronous-unwind-tables -fno-unwind-tables -Wall -Werror

ifeq ($(DEBUG), 1)
	CFLAGS += -g -Og -D__RELEASE__=0
else
	CFLAGS += -O3 -D__RELEASE__=1
endif

.PHONY: clean
.NOTPARALLEL:

# Compile assembly files
$(BUILD_DIR)/%.o: $(SRC_DIR)/asm/%.asm
	@echo "Assembling " $<
	@mkdir -p $(dir $@)
	@cd $(SRC_DIR)/asm && nasm -felf32 -g $< -o $@

# Compile C files.
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c 
	@echo "Compiling C files... "
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@ 

# Linking
${DAEMON_ELF}: $(OBJECTS)
	@echo "Linking..."
	@cd $(BUILD_DIR) && cp $(SOURCE_LD) . && $(LD) -m elf_i386 -T linker.ld $(OBJ_C) -o ${DAEMON_ELF}
	@objcopy -O binary ${DAEMON_ELF} ${DAEMON_BIN}
	@dd if=/dev/zero of=${IMG} bs=512 count=24
	@dd if=${DAEMON_BIN} of=${IMG}

all: ${DAEMON_ELF}

run: clean ${DAEMON_ELF}
ifeq ($(DEBUG), 1)
	@qemu-system-x86_64 -drive format=raw,file=$(IMG) -m 1M -s -S -no-reboot -no-shutdown & 	
	@echo "Waiting for QEMU to start..."
	@gdb -ex "target remote :$(GDB_PORT)" -ex "symbol-file $(DAEMON_ELF)" -ex "layout asm"
else
	@qemu-system-x86_64 -drive format=raw,file=$(IMG) -m 1M -d int -no-reboot -no-shutdown 	
endif

clean:
	@rm -rf ${BUILD_DIR}
