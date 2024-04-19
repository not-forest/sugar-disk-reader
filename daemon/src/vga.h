/* 
 * This module declare types and constants which provide the most basic interface
 * to VGA buffer, which provides an output in form of text.
 * */

#ifndef _VGA_INTERFACE_
#define _VGA_INTERFACE_

#include<stdint.h>

#define BUFFER_PTR 0xb8000
#define BUFFER_WIDTH 80
#define BUFFER_HEIGHT 25

#define COLOR_BLACK 0b0
#define COLOR_BLUE 0x1
#define COLOR_GREEN 0x2
#define COLOR_CYAN 0x3
#define COLOR_RED 0x4
#define COLOR_MAGENTA 0x5
#define COLOR_BROWN 0x6
#define COLOR_LIGHTGRAY 0x7
#define COLOR_DARKGRAY 0x8
#define COLOR_LIGHTBLUE 0x9
#define COLOR_LIGHTGREEN 0xa
#define COLOR_LIGHTCYAN 0xb
#define COLOR_LIGHTRED 0xc
#define COLOR_LIGHTMAGENTA 0xd
#define COLOR_YELLOW 0xe
#define COLOR_WHITE 0xf

#define L_WARN COLOR_YELLOW
#define L_OK   COLOR_GREEN
#define L_INFO COLOR_WHITE
#define L_ERROR COLOR_RED 

#define OK "[OK]"
#define FD "[FAILED]"

// A struct that tracks the cursor to print out messages correctly.
typedef struct {
    uint_fast8_t row, col;
} VGABuffer;

/* Shifts the values in the buffer by rows 

The function is not suitable for switching rows, but only shift to a
given location. Values must be within the height of the buffer. */
void vga_shift(uint_fast8_t old, uint_fast8_t new);
/* Swaps two rows instead of shifting one to another. 
The swapping is done via XOR operation for each bit*/
void vga_swap(uint_fast8_t old, uint_fast8_t new);

/* Prints out a single char by writing the data into the buffer.

the background and foreground colors will be provided with the char. If the
fourth bit of the background color is set, the character will blink like the
underscore symbol. If the fourth bit of the foreground color is set, the
character will be bright.

It will always return 1 as long as it will not receive a null character.*/
__attribute__((no_caller_saved_registers))
int printc(const unsigned char c, uint8_t color_set, volatile VGABuffer* vga);
/* Prints the provided string into the buffer.

The provided atributes are used to every symbol within the string. */
__attribute__((no_caller_saved_registers))
void prints(const char* str, uint8_t color_set, volatile VGABuffer* vga);

/* Works the same as the regular 'prints' except that is adds a newline symbol. */
__attribute__((no_caller_saved_registers))
void println(const char* str, uint8_t color_set, volatile VGABuffer* vga);
// Disables the cursor in the VGA mode.
void disable_cursor();

/* Enables the cursor, with given start and end

The start and end values are basically describe the height by rows of the cursor. The
start must be smaller than end for cursor to be visible. The maximum value of both
start and end is 15.*/
void enable_cursor(uint8_t cursor_start, uint8_t cursor_end);
// Updates the cursor location.
void update_cursor(uint8_t row, uint8_t col);


#endif
