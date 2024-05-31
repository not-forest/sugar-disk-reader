/* 
 *  Daemon definitions and data types for proper communication.
 * */

#ifndef __SDAEMON__
#define __SDAEMON__

#include<libusb-1.0/libusb.h>

#define BUFFER_SIZE 1024
#define MAX_PARTITIONS 16
#define MAX_FILES 256

/* Representation of system files. */
typedef struct {
    char name[256];
    char path[1024];
} File;

/* Representation of system partitions. */
typedef struct {
    char name[256];
    char mount_point[1024];
    File files[MAX_FILES];
    int file_count;
} Partition;

/* Representation of system disks. */
typedef struct {
    char name[256];
    Partition partitions[MAX_PARTITIONS];
    int partition_count;
} Disk;

/* Command bytes defined in Rust code. */
enum daemon_command_byte {
    REQ   = 0x00, // Request for acknowledgement
    ACK   = 0x01, // Acknowledgement
    NACK  = 0x02, // No acknowledgement
    SIZE  = 0xff, // Size indication
    CONN  = 0x03, // Connection request
    SHUT  = 0x04, // Shutdown request
    _SEL  = 0x05, // Select disk/partition
    UNSEL = 0x06, // Unselect disk/partition
    READ  = 0x07, // Read files
    RET   = 0x08, // Retry operation
    NAME  = 0x20, // Name follows
    PART  = 0x21, // Partition
    _FILE = 0x22, // File
    _DIR  = 0x23, // Directory
    BID   = 0x24, // Bridge's ID
};

/* Gets names of the disks. */
void get_disk_names(Disk *disks, int *disk_count);
/* Gets disk partitions. */
void get_partitions(Disk *disk);
/* Lists all files in a partition. */
void list_files(Partition *partition);
/* Helper function to copy the file contents for libusb sending. */
void copy_file(const char *src, const char *dest);
/* Parses the command. */
void parse_command(libusb_device_handle *devh, uint8_t command, Disk *disks, int disk_count);

#endif

