
/*
 *  Main daemon running script.
 *
 *  This program handles the connection with the mobile host application and performs a handshake.
 */

#include<libusb-1.0/libusb.h>
#include<stdint.h>
#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<sys/types.h>

#include "sdaemon.h"

static libusb_context *ctx = NULL;

int main(void) {
    libusb_device **list;
    libusb_device_handle* devh = NULL;
    struct libusb_device_descriptor ddes;
    int disk_count = 0;

    printf("[INFO] Searching for devices...\n");

    if (libusb_init(&ctx)) {
        fprintf(stderr, "[ERROR] Failed to initialize libusb\n");
        return 1;
    }

    // Getting all connected devices.
    ssize_t amount = libusb_get_device_list(ctx, &list);
    if (amount < 0) {
        fprintf(stderr, "[ERROR] Failed to get device list\n");
        libusb_exit(ctx);
        return 1;
    }

    printf("[INFO] List of devices attached:\n");
    for (ssize_t i = 0; i < amount; ++i) {
        libusb_device *device = list[i];

        if (libusb_get_device_descriptor(device, &ddes)) {
            fprintf(stderr, "[ERROR] Failed to get device descriptor\n");
            libusb_free_device_list(list, 1);
            libusb_exit(ctx);
            return 1;
        }

        printf("[INFO] Device %zd: Vendor ID = %04x, Product ID = %04x\n", i, ddes.idVendor, ddes.idProduct);

        // Open the first device (just as an example, you might want to choose based on criteria)
        if (devh == NULL) {
            if (libusb_open(device, &devh)) {
                fprintf(stderr, "[ERROR] Failed to open device\n");
                libusb_free_device_list(list, 1);
                libusb_exit(ctx);
                return 1;
            } else {
                printf("[INFO] Device %zd opened successfully\n", i);
                break;
            }
        }
    }

    libusb_free_device_list(list, 1);

    if (devh == NULL) {
        fprintf(stderr, "[ERROR] No device opened\n");
        libusb_exit(ctx);
        return 1;
    }

    // Claims the required interface.
    int result = libusb_claim_interface(devh, 0);
    if (result != LIBUSB_SUCCESS) {
        fprintf(stderr, "[ERROR] Failed to claim interface: %s\n", libusb_error_name(result));
        libusb_close(devh);
        libusb_exit(ctx);
        return 1;
    }

    printf("[INFO] Connection established, sending the acknowledgement signal.\n");

    // Allocate memory for disks
    Disk *disks = malloc(disk_count * sizeof(Disk));
    if (disks == NULL) {
        fprintf(stderr, "Failed to allocate memory for disks array\n");
        libusb_release_interface(devh, 0);
        libusb_close(devh);
        libusb_exit(ctx);
        return 1;
    }

    // Fetch disks info once
    get_disk_names(disks, &disk_count);
    printf("[INFO] Number of disks found: %d\n", disk_count);

    // Communication loop.
    for (;;) {
        uint8_t command;
        int transferred;
        result = libusb_bulk_transfer(devh, LIBUSB_ENDPOINT_OUT, &command, sizeof(command), &transferred, 0);
        if (result == LIBUSB_SUCCESS && transferred == sizeof(command)) {
            printf("[DEBUG] Command received: 0x%02x\n", command);
            parse_command(devh, command, disks, disk_count);
        } else {
            fprintf(stderr, "[ERROR] Failed to receive command: %s\n", libusb_error_name(result));
        }
    }

    // Release the interface and close the device
    libusb_release_interface(devh, 0);
    libusb_close(devh);
    libusb_exit(ctx);
    free(disks); // Free memory allocated for disks
    return 0;
}

void parse_command(libusb_device_handle *devh, uint8_t command, Disk *disks, int disk_count) {
    static Disk *selected_disk = NULL;
    static Partition *selected_partition = NULL;

    printf("[DEBUG] Parsing command: 0x%02x\n", command);

    switch (command) {
        case NAME: {
            printf("[INFO] Handling NAME command\n");
            if (selected_disk != NULL) {
                for (int i = 0; i < disk_count; ++i) {
                    printf("[DEBUG] Sending disk name: %s\n", disks[i].name);
                    libusb_bulk_transfer(devh, LIBUSB_ENDPOINT_OUT, (unsigned char*)disks[i].name, sizeof(disks[i].name), NULL, 0);
                }
            }
            break;
        }
        case PART: {
            printf("[INFO] Handling PART command\n");
            if (selected_disk != NULL) {
                get_partitions(selected_disk);
                for (int i = 0; i < selected_disk->partition_count; ++i) {
                    printf("[DEBUG] Sending partition name: %s\n", selected_disk->partitions[i].name);
                    libusb_bulk_transfer(devh, LIBUSB_ENDPOINT_OUT, (unsigned char*)selected_disk->partitions[i].name, sizeof(selected_disk->partitions[i].name), NULL, 0);
                }
            }
            break;
        }
        case _FILE: {
            printf("[INFO] Handling FILE command\n");
            if (selected_partition != NULL) {
                list_files(selected_partition);
                for (int i = 0; i < selected_partition->file_count; ++i) {
                    printf("[DEBUG] Sending file name: %s\n", selected_partition->files[i].name);
                    libusb_bulk_transfer(devh, LIBUSB_ENDPOINT_OUT, (unsigned char*)selected_partition->files[i].name, sizeof(selected_partition->files[i].name), NULL, 0);
                }
            }
            break;
        }
        case _SEL: {
            printf("[INFO] Handling SELECT command\n");
            // Select disk or partition
            uint8_t buffer[256];
            int transferred;
            libusb_bulk_transfer(devh, LIBUSB_ENDPOINT_IN, buffer, sizeof(buffer), &transferred, 0);
            buffer[transferred] = '\0';
            printf("[DEBUG] Selection buffer received: %s\n", buffer);

            // Try to select disk first
            for (int i = 0; i < disk_count; ++i) {
                if (strcmp((char*)buffer, disks[i].name) == 0) {
                    selected_disk = &disks[i];
                    selected_partition = NULL;
                    printf("[INFO] Disk selected: %s\n", selected_disk->name);
                    return;
                }
            }

            // Try to select partition within the selected disk
            if (selected_disk != NULL) {
                get_partitions(selected_disk);
                for (int i = 0; i < selected_disk->partition_count; ++i) {
                    if (strcmp((char*)buffer, selected_disk->partitions[i].name) == 0) {
                        selected_partition = &selected_disk->partitions[i];
                        printf("[INFO] Partition selected: %s\n", selected_partition->name);
                        return;
                    }
                }
            }

            // If neither disk nor partition was found
            selected_disk = NULL;
            selected_partition = NULL;
            printf("[WARNING] No matching disk or partition found\n");
            break;
        }
        case UNSEL: {
            printf("[INFO] Handling UNSELECT command\n");
            selected_disk = NULL;
            selected_partition = NULL;
            printf("[INFO] Disk and partition unselected\n");
            break;
        }
        case READ: {
            printf("[INFO] Handling READ command\n");
            if (selected_partition != NULL && selected_partition->file_count > 0) {
                for (int i = 0; i < selected_partition->file_count; ++i) {
                    printf("[DEBUG] Reading file: %s\n", selected_partition->files[i].path);
                    copy_file(selected_partition->files[i].path, "/tmp/usb_transfer_file");
                    FILE *file = fopen("/tmp/usb_transfer_file", "rb");
                    if (file) {
                        char file_buffer[BUFFER_SIZE];
                        size_t bytes_read;
                        while ((bytes_read = fread(file_buffer, 1, BUFFER_SIZE, file)) > 0) {
                            printf("[DEBUG] Sending file content chunk of size %zu\n", bytes_read);
                            libusb_bulk_transfer(devh, LIBUSB_ENDPOINT_OUT, (unsigned char*)file_buffer, bytes_read, NULL, 0);
                        }
                        fclose(file);
                    }
                }
            }
            break;
        }
        default: {
            fprintf(stderr, "[ERROR] Unknown command: 0x%02x\n", command);
            break;
        }
    }
}

