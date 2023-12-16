# Project Name
TARGET = pedal

APP_TYPE = BOOT_SRAM

LIBDIR = -L ../../board/target/thumbv7em-none-eabihf/debug
LIBS = -lpedalboard

C_DEFS  ?= -DPEDAL_MAIN=$(PEDAL_MAIN)

LDFLAGS += -u _printf_float

clean_exes:
	rm -f build/pedal.{bin,hex,map,elf}

# Sources
CPP_SOURCES = pedal.cpp spew.cpp hw.cpp load.cpp

# Library Locations
LIBDAISY_DIR = ../../../DaisyExamples/libDaisy
DAISYSP_DIR = ../../../DaisyExamples/DaisySP

# Core location, and generic Makefile.
SYSTEM_FILES_DIR = $(LIBDAISY_DIR)/core
include $(SYSTEM_FILES_DIR)/Makefile
