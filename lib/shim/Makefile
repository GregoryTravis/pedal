# Project Name
TARGET = pedal

OPT = -O0

APP_TYPE = BOOT_SRAM

LIBDIR = -L ../../board/target/thumbv7em-none-eabihf/debug
#LIBDIR = -L ../../board/target/thumbv7em-none-eabihf/release
LIBS = -lpedalboard

C_DEFS  ?= -DPEDAL_MAIN=$(PEDAL_MAIN)

LDFLAGS += -u _printf_float

clean_exes: clean_pedal_dependent_o
	rm -f build/pedal.{bin,hex,map,elf}

clean_pedal_dependent_o:
	rm -f build/pedal.o

# Sources
CPP_SOURCES = pedal.cpp spew.cpp hw.cpp load.cpp vibrato.cpp playhead.cpp circbuf.cpp patch.cpp

# Library Locations
DAISY_EXAMPLES = ../../../DaisyExamples
LIBDAISY_DIR = $(DAISY_EXAMPLES)/libDaisy
DAISYSP_DIR = $(DAISY_EXAMPLES)/DaisySP

# Core location, and generic Makefile.
SYSTEM_FILES_DIR = $(LIBDAISY_DIR)/core
include $(SYSTEM_FILES_DIR)/Makefile
