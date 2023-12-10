# Project Name
TARGET = high_pass

APP_TYPE = BOOT_SRAM

LIBDIR = -L ../target/thumbv7em-none-eabihf/debug
LIBS = -lhigh_pass

C_DEFS  ?= -DPEDAL_MAIN=$(PEDAL_MAIN)

LDFLAGS += -u _printf_float

clean_exes:
	rm -f build/high_pass.{bin,hex,map,elf}

# Sources
CPP_SOURCES = pedal.cpp spew.cpp hw.cpp load.cpp

# Library Locations
LIBDAISY_DIR = ../../DaisyExamples/libDaisy
DAISYSP_DIR = ../../DaisyExamples/DaisySP

# Core location, and generic Makefile.
SYSTEM_FILES_DIR = $(LIBDAISY_DIR)/core
include $(SYSTEM_FILES_DIR)/Makefile
