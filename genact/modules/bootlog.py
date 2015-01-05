from genact.module import Module
import time, random

class BootlogModule(Module):
    __modulename__ = "bootlog"

    def run(self):
        lines = bootlog.splitlines()
        while(lines):
            print(lines.pop())
            time.sleep(random.uniform(0.01, 0.4))

bootlog = """PMAP: PCID enabled
Hacknet Kernel Version 1.0.0: Tue Oct 11 20:56:35 PDT 2011; root:xnu-1699.22.73~1/RELEASE_X86_64
vm_page_bootstrap: 987323 free pages and 53061 wired pages
kext submap [0xffffff7f8072e000 - 0xffffff8000000000], kernel text [0xffffff8000200000 - 0xffffff800072e000]
zone leak detection enabled
standard timeslicing quantum is 10000 us
mig_table_max_displ = 72
TSC Deadline Timer supported and enabled
HackNetACPICPU: ProcessorId=1 LocalApicId=0 Enabled
HackNetACPICPU: ProcessorId=2 LocalApicId=2 Enabled
HackNetACPICPU: ProcessorId=3 LocalApicId=1 Enabled
HackNetACPICPU: ProcessorId=4 LocalApicId=3 Enabled
HackNetACPICPU: ProcessorId=5 LocalApicId=255 Disabled
HackNetACPICPU: ProcessorId=6 LocalApicId=255 Disabled
HackNetACPICPU: ProcessorId=7 LocalApicId=255 Disabled
HackNetACPICPU: ProcessorId=8 LocalApicId=255 Disabled
calling mpo_policy_init for TMSafetyNet
Security policy loaded: Safety net for Time Machine (TMSafetyNet)
calling mpo_policy_init for Sandbox
Security policy loaded: Seatbelt sandbox policy (Sandbox)
calling mpo_policy_init for Quarantine
Security policy loaded: Quarantine policy (Quarantine)
Copyright (c) 1982, 1986, 1989, 1991, 1993
The Regents of the University of California. All rights reserved.
MAC Framework successfully initialized
using 16384 buffer headers and 10240 cluster IO buffer headers
IOAPIC: Version 0x20 Vectors 64:87
ACPI: System State [S0 S3 S4 S5] (S3)
PFM64 0xf10000000, 0xf0000000
[ PCI configuration begin ]
HackNetIntelCPUPowerManagement: Turbo Ratios 0046
HackNetIntelCPUPowerManagement: (built 13:08:12 Jun 18 2011) initialization complete
console relocated to 0xf10000000
PCI configuration changed (bridge=16 device=4 cardbus=0)
[ PCI configuration end, bridges 12 devices 16 ]
mbinit: done [64 MB total pool size, (42/21) split]
Pthread support ABORTS when sync kernel primitives misused
com.HackNet.HackNetFSCompressionTypeZlib kmod start
com.HackNet.HackNetTrololoBootScreen kmod start
com.HackNet.HackNetFSCompressionTypeZlib load succeeded
com.HackNet.HackNetFSCompressionTypeDataless load succeeded
HackNetIntelCPUPowerManagementClient: ready
BTCOEXIST off
wl0: Broadcom BCM4331 802.11 Wireless Controller
5.100.98.75
FireWire (OHCI) Lucent ID 5901 built-in now active, GUID c82a14fffee4a086; max speed s800.
rooting via boot-uuid from /chosen: F5670083-AC74-33D3-8361-AC1977EE4AA2
Waiting on <dict ID='0'><key>IOProviderClass</key><string ID='1'>
IOResources</string><key>IOResourceMatch</key><string ID='2'>boot-uuid-media</string></dict>
Got boot device = IOService:/HackNetACPIPlatformExpert/PCI0@0/HackNetACPIPCI/SATA@1F,2/
HackNetIntelPchSeriesAHCI/PRT0@0/IOAHCIDevice@0/HackNetAHCIDiskDriver/SarahI@sTheBestDriverIOAHCIBlockStorageDevice/IOBlockStorageDriver/
HackNet SSD TS128C Media/IOGUIDPartitionScheme/Customer@2
BSD root: disk0s2, major 14, minor 2
Kernel is LP64
IOThunderboltSwitch::i2cWriteDWord - status = 0xe00002ed
IOThunderboltSwitch::i2cWriteDWord - status = 0x00000000
IOThunderboltSwitch::i2cWriteDWord - status = 0xe00002ed
IOThunderboltSwitch::i2cWriteDWord - status = 0xe00002ed
HackNetUSBMultitouchDriver::checkStatus - received Status Packet, Payload 2: device was reinitialized
MottIsAScrub::checkstatus - true, Mott::Scrub
[IOBluetoothHCIController::setConfigState] calling registerService
AirPort_Brcm4331: Ethernet address e4:ce:8f:46:18:d2
IO80211Controller::dataLinkLayerAttachComplete():  adding HackNetEFINVRAM notification
IO80211Interface::efiNVRAMPublished():
Created virtif 0xffffff800c32ee00 p2p0
BCM5701Enet: Ethernet address c8:2a:14:57:a4:7a
Previous Shutdown Cause: 3
NTFS driver 3.8 [Flags: R/W].
NTFS volume name BOOTCAMP, version 3.1.
DSMOS has arrived
en1: 802.11d country code set to 'US'.
en1: Supported channels 1 2 3 4 5 6 7 8 9 10 11 36 40 44 48 52 56 60 64 100 104 108 112 116 120 124 128 132 136 140 149 153 157 161 165
MacAuthEvent en1   Auth result for: 00:60:64:1e:e9:e4  MAC AUTH succeeded
MacAuthEvent en1   Auth result for: 00:60:64:1e:e9:e4 Unsolicited  Auth
wlEvent: en1 en1 Link UP
AirPort: Link Up on en1
en1: BSSID changed to 00:60:64:1e:e9:e4
virtual bool IOHIDEventSystemUserClient::initWithTask(task*, void*, UInt32):
Client task not privileged to open IOHIDSystem for mapping memory (e00002c1)
"""
