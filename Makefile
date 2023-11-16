# Project Name
TARGET = pedal

#APP_TYPE = BOOT_SRAM

LIBDIR = -L /Users/gmt/pedal/step/target/thumbv7em-none-eabihf/debug
LIBS = -lstep

LDFLAGS += -u _printf_float

# Sources
CPP_SOURCES = pedal.cpp

# Library Locations
LIBDAISY_DIR = ../DaisyExamples/libDaisy
DAISYSP_DIR = ../DaisyExamples/DaisySP

# Core location, and generic Makefile.
SYSTEM_FILES_DIR = $(LIBDAISY_DIR)/core
include $(SYSTEM_FILES_DIR)/Makefile
