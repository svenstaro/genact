#!/usr/bin/env python3

import os
import random
import sys
import time
import argparse


TERMINAL_SIZE = (25, 80)
if os.name == "posix":
    TERMINAL_SIZE = rows, columns = os.popen('stty size', 'r').read().split()

class ActivityGenerator(object):
    def __init__(self):
        self.activities = [self.activity_initialize, self.activity_dump_log,
                           self.activity_configure, self.activity_compile,
                           self.activity_memory_dump, self.activity_backtrace,
                           self.activity_netstat, self.activity_download,
                           self.activity_boot]
        self.activity_names = []
        for activity in self.activities:
            self.activity_names.append(activity.__name__.replace("activity_", ""))

    def human_readable_size(size):
        names = ['KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB']
        for name in names:
            size /= 1024
            if size < 1024:
                return '{0:.1f} {1}'.format(size, name)
        return "huge!"

    def draw_header(self, message, width = TERMINAL_SIZE[1],
                    centered = True, rows = 3, fill_char = "*"):
        for i in range(rows):
            if i == int(rows / 2):
                print("{0:{2}^{1}}".format(" "+message+" ", width, fill_char))
            else:
                print("{0:{0}^{1}}".format(fill_char, width))

    def draw_progress(self, left_side, right_side, width = 30, duration = 5,
                        fill_char = "="):
        start_time = time.time()
        spinner_statuses = ["|", "/", "-", "\\"]
        spinner_pos = 0
        elapsed_time = time.time() - start_time
        while(elapsed_time < duration):
            elapsed_time = time.time() - start_time

            # Make sure to get a fill_char at the end of our cycle when done.
            if elapsed_time >= duration:
                spinner_statuses = [fill_char]
                spinner_pos = 0

            print("\r{3} [{0:{0}^{1}}{2}]".format(fill_char,
                                        int((width / duration) * elapsed_time),
                                        spinner_statuses[spinner_pos],
                                        left_side, right_side), end="")
            sys.stdout.flush()

            if spinner_pos >= len(spinner_statuses) - 1:
                spinner_pos = 0
            else:
                spinner_pos += 1

            time.sleep(duration / width)

    def activity_initialize(self):
        self.draw_progress("Initializing core system", "done")
        print()

    def generate_bootlog(self):
        bootlog = ""
        bootlog += "PMAP: PCID enabled\n"
        bootlog += "Hacknet Kernel Version 1.0.0: Tue Oct 11 20:56:35 PDT 2011; root:xnu-1699.22.73~1/RELEASE_X86_64\n"
        bootlog += "vm_page_bootstrap: 987323 free pages and 53061 wired pages\n"
        bootlog += "kext submap [0xffffff7f8072e000 - 0xffffff8000000000], kernel text [0xffffff8000200000 - 0xffffff800072e000]\n"
        bootlog += "zone leak detection enabled\n"
        bootlog += "standard timeslicing quantum is 10000 us\n"
        bootlog += "mig_table_max_displ = 72\n"
        bootlog += "TSC Deadline Timer supported and enabled\n"
        bootlog += "HackNetACPICPU: ProcessorId=1 LocalApicId=0 Enabled\n"
        bootlog += "HackNetACPICPU: ProcessorId=2 LocalApicId=2 Enabled\n"
        bootlog += "HackNetACPICPU: ProcessorId=3 LocalApicId=1 Enabled\n"
        bootlog += "HackNetACPICPU: ProcessorId=4 LocalApicId=3 Enabled\n"
        bootlog += "HackNetACPICPU: ProcessorId=5 LocalApicId=255 Disabled\n"
        bootlog += "HackNetACPICPU: ProcessorId=6 LocalApicId=255 Disabled\n"
        bootlog += "HackNetACPICPU: ProcessorId=7 LocalApicId=255 Disabled\n"
        bootlog += "HackNetACPICPU: ProcessorId=8 LocalApicId=255 Disabled\n"
        bootlog += "calling mpo_policy_init for TMSafetyNet\n"
        bootlog += "Security policy loaded: Safety net for Time Machine (TMSafetyNet)\n"
        bootlog += "calling mpo_policy_init for Sandbox\n"
        bootlog += "Security policy loaded: Seatbelt sandbox policy (Sandbox)\n"
        bootlog += "calling mpo_policy_init for Quarantine\n"
        bootlog += "Security policy loaded: Quarantine policy (Quarantine)\n"
        bootlog += "Copyright (c) 1982, 1986, 1989, 1991, 1993\n"
        bootlog += "The Regents of the University of California. All rights reserved.\n"
        bootlog += "MAC Framework successfully initialized\n"
        bootlog += "using 16384 buffer headers and 10240 cluster IO buffer headers\n"
        bootlog += "IOAPIC: Version 0x20 Vectors 64:87\n"
        bootlog += "ACPI: System State [S0 S3 S4 S5] (S3)\n"
        bootlog += "PFM64 0xf10000000, 0xf0000000\n"
        bootlog += "[ PCI configuration begin ]\n"
        bootlog += "HackNetIntelCPUPowerManagement: Turbo Ratios 0046\n"
        bootlog += "HackNetIntelCPUPowerManagement: (built 13:08:12 Jun 18 2011) initialization complete\n"
        bootlog += "console relocated to 0xf10000000\n"
        bootlog += "PCI configuration changed (bridge=16 device=4 cardbus=0)\n"
        bootlog += "[ PCI configuration end, bridges 12 devices 16 ]\n"
        bootlog += "mbinit: done [64 MB total pool size, (42/21) split]\n"
        bootlog += "Pthread support ABORTS when sync kernel primitives misused\n"
        bootlog += "com.HackNet.HackNetFSCompressionTypeZlib kmod start\n"
        bootlog += "com.HackNet.HackNetTrololoBootScreen kmod start\n"
        bootlog += "com.HackNet.HackNetFSCompressionTypeZlib load succeeded\n"
        bootlog += "com.HackNet.HackNetFSCompressionTypeDataless load succeeded\n"
        bootlog += "HackNetIntelCPUPowerManagementClient: ready\n"
        bootlog += "BTCOEXIST off\n"
        bootlog += "wl0: Broadcom BCM4331 802.11 Wireless Controller\n"
        bootlog += "5.100.98.75\n"
        bootlog += "FireWire (OHCI) Lucent ID 5901 built-in now active, GUID c82a14fffee4a086; max speed s800.\n"
        bootlog += "rooting via boot-uuid from /chosen: F5670083-AC74-33D3-8361-AC1977EE4AA2\n"
        bootlog += "Waiting on <dict ID='0'><key>IOProviderClass</key><string ID='1'>\n"
        bootlog += "IOResources</string><key>IOResourceMatch</key><string ID='2'>boot-uuid-media</string></dict>\n"
        bootlog += "Got boot device = IOService:/HackNetACPIPlatformExpert/PCI0@0/HackNetACPIPCI/SATA@1F,2/\n"
        bootlog += "HackNetIntelPchSeriesAHCI/PRT0@0/IOAHCIDevice@0/HackNetAHCIDiskDriver/SarahI@sTheBestDriverIOAHCIBlockStorageDevice/IOBlockStorageDriver/\n"
        bootlog += "HackNet SSD TS128C Media/IOGUIDPartitionScheme/Customer@2\n"
        bootlog += "BSD root: disk0s2, major 14, minor 2\n"
        bootlog += "Kernel is LP64\n"
        bootlog += "IOThunderboltSwitch::i2cWriteDWord - status = 0xe00002ed\n"
        bootlog += "IOThunderboltSwitch::i2cWriteDWord - status = 0x00000000\n"
        bootlog += "IOThunderboltSwitch::i2cWriteDWord - status = 0xe00002ed\n"
        bootlog += "IOThunderboltSwitch::i2cWriteDWord - status = 0xe00002ed\n"
        bootlog += "HackNetUSBMultitouchDriver::checkStatus - received Status Packet, Payload 2: device was reinitialized\n"
        bootlog += "MottIsAScrub::checkstatus - true, Mott::Scrub\n"
        bootlog += "[IOBluetoothHCIController::setConfigState] calling registerService\n"
        bootlog += "AirPort_Brcm4331: Ethernet address e4:ce:8f:46:18:d2\n"
        bootlog += "IO80211Controller::dataLinkLayerAttachComplete():  adding HackNetEFINVRAM notification\n"
        bootlog += "IO80211Interface::efiNVRAMPublished():\n"
        bootlog += "Created virtif 0xffffff800c32ee00 p2p0\n"
        bootlog += "BCM5701Enet: Ethernet address c8:2a:14:57:a4:7a\n"
        bootlog += "Previous Shutdown Cause: 3\n"
        bootlog += "NTFS driver 3.8 [Flags: R/W].\n"
        bootlog += "NTFS volume name BOOTCAMP, version 3.1.\n"
        bootlog += "DSMOS has arrived\n"
        bootlog += "en1: 802.11d country code set to 'US'.\n"
        bootlog += "en1: Supported channels 1 2 3 4 5 6 7 8 9 10 11 36 40 44 48 52 56 60 64 100 104 108 112 116 120 124 128 132 136 140 149 153 157 161 165\n"
        bootlog += "MacAuthEvent en1   Auth result for: 00:60:64:1e:e9:e4  MAC AUTH succeeded\n"
        bootlog += "MacAuthEvent en1   Auth result for: 00:60:64:1e:e9:e4 Unsolicited  Auth\n"
        bootlog += "wlEvent: en1 en1 Link UP\n"
        bootlog += "AirPort: Link Up on en1\n"
        bootlog += "en1: BSSID changed to 00:60:64:1e:e9:e4\n"
        bootlog += "virtual bool IOHIDEventSystemUserClient::initWithTask(task*, void*, UInt32):\n"
        bootlog += "Client task not privileged to open IOHIDSystem for mapping memory (e00002c1)\n"
        bootlog += "Boot Complete\n"
        return bootlog

    def activity_boot(self):
        bootlog = self.generate_bootlog()
        lines = bootlog.splitlines()
        while(lines):
            print(lines.pop())
            time.sleep(random.uniform(0.01, 0.4))

    def activity_dump_log(self):
        logs = ["kernel", "system", "user", "message", "error"]
        choice = random.choice(logs)
        if choice == "kernel":
            print("Dumping system log")

    def activity_configure(self):
        print("config")

    def activity_compile(self):
        print("compiling")

    def activity_memory_dump(self, columns = 8, format = "hex",
                             bits = 32, duration = 5, delay = 0.2):
        dumps = ["dumping memory map", "memory dump:",
                 "Dumping memory addresses", "memory image"]
        choice = random.choice(dumps)
        self.draw_header(choice)
        start_time = time.time()
        while(time.time() - start_time < duration):
            for i in range(columns):
                address = random.randint(0, pow(2, bits) - 1)
                print("{0:0<#{1}{2}}".format(address, int((bits / 4) + 2), "x"),
                                             end=" ")
                sys.stdout.flush()
                time.sleep(delay / columns)
            print()
            time.sleep(delay)

    def activity_backtrace(self):
        print("backtrace")

    def activity_netstat(self):
        print("netstat")

    def activity_download(self, width = 60, delay = 0.2):
        download_progress = 0
        file_size = random.randint(100000, 1000000)
        ip = str(random.randint(1,255)) + "." + \
                str(random.randint(0,255)) + "." + \
                str(random.randint(0,255)) + "." + \
                str(random.randint(1,255))
        ports = [21, 22, 80, 81, 443, 25652, 1337]
        port = str(random.choice(ports))
        print("Connecting to {0} on port {1} ...".format(ip,port));
        print("File size: {0}".format(ActivityGenerator.human_readable_size(file_size)))

        download_speed = random.randint(6000, 200000) # bytes per second
        received_size = 0
        done = False
        while not done:
            if received_size > file_size:
                received_size = file_size
            speed = download_speed * (1 + random.randint(-1000, 1000) / 10000.0)
            i = int(received_size / file_size * width)
            print("\r{0: >6.1%} [{2: <{3}}] {1: >10}/s".format(
                received_size / file_size,
                ActivityGenerator.human_readable_size(speed),
                "=" * i, width), end="")
            sys.stdout.flush()
            if received_size >= file_size:
                done = True
            received_size += speed * delay
            time.sleep(delay)
        print("\nDownload finished.")

if __name__ == "__main__":
    actgen = ActivityGenerator()

    parser = argparse.ArgumentParser(description="A nonsense activity generator")
    parser.add_argument('-a', '--activities', metavar='A', nargs='+', choices=actgen.activity_names,
                        help='Provide one or more of: \'' + '\', \''.join(actgen.activity_names) + '\'')
    args = parser.parse_args()
    activities = []
    if args.activities:
        for activity in args.activities:
            activities.append(getattr(actgen, "activity_" + activity))

    running = True
    while running:
        choice = random.choice(activities if activities else actgen.activities)
        choice()
        running = False
