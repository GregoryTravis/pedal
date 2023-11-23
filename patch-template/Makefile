# Project Name
TARGET = $PEDAL_PATCH_NAME

APP_TYPE = BOOT_SRAM

LIBDIR = -L /Users/gmt/pedal/$PEDAL_PATCH_NAME/patch/target/thumbv7em-none-eabihf/debug -L /Users/gmt/pedal/dsp/target/thumbv7em-none-eabihf/debug
LIBS = -l$PEDAL_PATCH_NAME -ldsp

LDFLAGS += -u _printf_float

# Sources
CPP_SOURCES = pedal.cpp

# Library Locations
LIBDAISY_DIR = ../../DaisyExamples/libDaisy
DAISYSP_DIR = ../../DaisyExamples/DaisySP

# Core location, and generic Makefile.
SYSTEM_FILES_DIR = $(LIBDAISY_DIR)/core
include $(SYSTEM_FILES_DIR)/Makefile
