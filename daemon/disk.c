
/* 
 *  Module for obtaining data about the disks, partitions and files.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>
#include <unistd.h>

#include "sdaemon.h"

#define BUFFER_SIZE 1024
#define MAX_NAME_LENGTH 256
#define MAX_PATH_LENGTH 1024

/* Gets names of the disks. */
void get_disk_names(Disk *disks, int *disk_count) {
    FILE *fp;
    char path[MAX_NAME_LENGTH];

    printf("[INFO] Fetching disk names...\n");

    fp = popen("lsblk -dn -o NAME", "r");
    if (fp == NULL) {
        perror("[ERROR] popen failed");
        return;
    }

    *disk_count = 0;
    while (fgets(path, sizeof(path) - 1, fp) != NULL) {
        path[strcspn(path, "\n")] = 0;  // Remove newline character
        strncpy(disks[*disk_count].name, path, sizeof(disks[*disk_count].name) - 1);
        disks[*disk_count].name[sizeof(disks[*disk_count].name) - 1] = '\0';
        disks[*disk_count].partition_count = 0;
        printf("[DEBUG] Disk found: %s\n", disks[*disk_count].name);
        (*disk_count)++;
    }

    pclose(fp);
    printf("[INFO] Total number of disks found: %d\n", *disk_count);
}

/* Gets disk partitions. */
void get_partitions(Disk *disk) {
    char command[BUFFER_SIZE];
    snprintf(command, sizeof(command), "lsblk -ln -o NAME /dev/%s", disk->name);
    FILE *fp;
    char path[MAX_NAME_LENGTH];

    printf("[INFO] Fetching partitions for disk: %s\n", disk->name);

    fp = popen(command, "r");
    if (fp == NULL) {
        perror("[ERROR] popen failed");
        return;
    }

    while (fgets(path, sizeof(path) - 1, fp) != NULL) {
        path[strcspn(path, "\n")] = 0;  // Remove newline character
        if (strstr(path, disk->name) != NULL && strcmp(path, disk->name) != 0) {
            Partition *partition = &disk->partitions[disk->partition_count];
            strncpy(partition->name, path, sizeof(partition->name) - 1);
            partition->name[sizeof(partition->name) - 1] = '\0';
            snprintf(partition->mount_point, sizeof(partition->mount_point), "/mnt/disks/%s", path);
            partition->mount_point[sizeof(partition->mount_point) - 1] = '\0';
            partition->file_count = 0;
            printf("[DEBUG] Partition found: %s, mount point: %s\n", partition->name, partition->mount_point);
            disk->partition_count++;
        }
    }

    pclose(fp);
    printf("[INFO] Total number of partitions found for disk %s: %d\n", disk->name, disk->partition_count);
}

/* Lists all files in a partition. */
void list_files(Partition *partition) {
    DIR *d;
    struct dirent *dir;

    printf("[INFO] Listing files for partition: %s\n", partition->name);

    d = opendir(partition->mount_point);
    if (d) {
        partition->file_count = 0;
        while ((dir = readdir(d)) != NULL) {
            if (dir->d_type == DT_REG) {  // Only regular files
                File *file = &partition->files[partition->file_count];
                strncpy(file->name, dir->d_name, sizeof(file->name) - 1);
                file->name[sizeof(file->name) - 1] = '\0';

                // Check the length of the combined path
                size_t mount_point_len = strlen(partition->mount_point);
                size_t dir_name_len = strlen(dir->d_name);

                if (mount_point_len + 1 + dir_name_len < sizeof(file->path)) {
                    snprintf(file->path, sizeof(file->path), "%s/%s", partition->mount_point, dir->d_name);
                    printf("[DEBUG] File found: %s, path: %s\n", file->name, file->path);
                    partition->file_count++;
                } else {
                    fprintf(stderr, "[WARNING] Path too long: %s/%s\n", partition->mount_point, dir->d_name);
                }
            }
        }
        closedir(d);
        printf("[INFO] Total number of files found in partition %s: %d\n", partition->name, partition->file_count);
    } else {
        perror("[ERROR] opendir failed");
    }
}

/* Helper function to copy the file contents for libusb sending. */
void copy_file(const char *src, const char *dest) {
    char buffer[BUFFER_SIZE];
    FILE *source, *destination;
    size_t bytes;

    printf("[INFO] Copying file from %s to %s\n", src, dest);

    source = fopen(src, "rb");
    if (!source) {
        perror("[ERROR] fopen source file failed");
        return;
    }

    destination = fopen(dest, "wb");
    if (!destination) {
        fclose(source);
        perror("[ERROR] fopen destination file failed");
        return;
    }

    while ((bytes = fread(buffer, 1, BUFFER_SIZE, source)) != 0) {
        fwrite(buffer, 1, bytes, destination);
        printf("[DEBUG] Copied %zu bytes from %s to %s\n", bytes, src, dest);
    }

    fclose(source);
    fclose(destination);
    printf("[INFO] Finished copying file from %s to %s\n", src, dest);
}

