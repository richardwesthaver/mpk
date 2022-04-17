// gcc -Wall -framework DiskArbitration -framework CoreFoundation disk_arbiter_test.c
#include <DiskArbitration/DiskArbitration.h>

#define DEFAULT_DISK "/dev/disk1"
#define IFERR(ptr, msg) if (!ptr) { fprintf(stderr, "%s\n", msg); goto out; }

int main() {
  int ret = -1;
  DASessionRef session = NULL;
  DADiskRef disk = NULL;
  CFDictionaryRef diskinfo = NULL;

  char *disk_name = DEFAULT_DISK;
  
  session = DASessionCreate(kCFAllocatorDefault);
  IFERR(session, "failed to create Disk Arbitration session");

  disk = DADiskCreateFromBSDName(kCFAllocatorDefault, session, disk_name);
  IFERR(disk, "failed to create disk object");

  diskinfo = DADiskCopyDescription(disk);
  IFERR(diskinfo, "failed to retrieve disk description");

  CFShow(diskinfo);

 out:
  if (diskinfo)
    CFRelease(diskinfo);
  if (disk)
    CFRelease(disk);
  if (session)
    CFRelease(session);
  exit(ret);
}
