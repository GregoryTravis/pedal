# Project Name
TARGET = pedal

OPT = -O3
#OPT = -O0

APP_TYPE = BOOT_SRAM

#LIBDIR = -L ../../board/target/thumbv7em-none-eabihf/debug
LIBDIR = -L ../../board/target/thumbv7em-none-eabihf/release
LIBS = -lpedalboard

C_DEFS  ?= -DPEDAL_MAIN=$(PEDAL_MAIN)

LDFLAGS += -u _printf_float

clean_exes: clean_pedal_dependent_o
	rm -f build/pedal.{bin,hex,map,elf}

clean_pedal_dependent_o:
	rm -f build/pedal.o

# Sources
CPP_SOURCES = spew.cpp hw.cpp load.cpp vibrato.cpp playhead.cpp circbuf.cpp patch.cpp speed_test.cpp daisy_seed.cpp rig.cpp knob.cpp switch.cpp
# CPP_SOURCES omitted since they take up code and bss space even when not used
# arm_fft.cpp shy_fft.cpp

ASM_SOURCES += \
$(LIBDAISY_DIR)/Drivers/CMSIS/DSP/Source/TransformFunctions/arm_bitreversal2.s

C_SOURCES += \
$(LIBDAISY_DIR)/Drivers/CMSIS/DSP/Source/TransformFunctions/arm_rfft_fast_f32.c \
$(LIBDAISY_DIR)/Drivers/CMSIS/DSP/Source/TransformFunctions/arm_rfft_fast_init_f32.c \
$(LIBDAISY_DIR)/Drivers/CMSIS/DSP/Source/TransformFunctions/arm_cfft_f32.c \
$(LIBDAISY_DIR)/Drivers/CMSIS/DSP/Source/TransformFunctions/arm_cfft_radix8_f32.c \
$(LIBDAISY_DIR)/Drivers/CMSIS/DSP/Source/CommonTables/arm_const_structs.c \
$(LIBDAISY_DIR)/Drivers/CMSIS/DSP/Source/CommonTables/arm_common_tables.c

C_DEFS += \
-DUSE_ARM_FFT \

#OBJECTS += $(addprefix $(BUILD_DIR)/,$(notdir $(ASM_SOURCES:.s=.o)))
#vpath %.s $(sort $(dir $(ASM_SOURCES)))
OBJECTS += $(addprefix $(BUILD_DIR)/,$(ASM_SOURCES:.s=.o))
vpath %.s $(sort $(dir $(ASM_SOURCES)))

# Library Locations
DAISY_EXAMPLES = ../../../DaisyExamples
LIBDAISY_DIR = $(DAISY_EXAMPLES)/libDaisy
DAISYSP_DIR = $(DAISY_EXAMPLES)/DaisySP

# Core location, and generic Makefile.
SYSTEM_FILES_DIR = $(LIBDAISY_DIR)/core
include $(SYSTEM_FILES_DIR)/Makefile
