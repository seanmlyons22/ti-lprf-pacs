#[doc = "Register `RFCPEIFG` reader"]
pub type R = crate::R<RfcpeifgSpec>;
#[doc = "Register `RFCPEIFG` writer"]
pub type W = crate::W<RfcpeifgSpec>;
#[doc = "Field `COMMAND_DONE` reader - 0:0\\]
Interrupt flag 0. A radio operation has finished. (IEEE 802.15.4 mode: A background level radio operation command has finished.) Write zero to clear flag. Write to one has no effect."]
pub type CommandDoneR = crate::BitReader;
#[doc = "Field `COMMAND_DONE` writer - 0:0\\]
Interrupt flag 0. A radio operation has finished. (IEEE 802.15.4 mode: A background level radio operation command has finished.) Write zero to clear flag. Write to one has no effect."]
pub type CommandDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAST_COMMAND_DONE` reader - 1:1\\]
Interrupt flag 1. The last radio operation command in a chain of commands has finished. (IEEE 802.15.4 mode: The last background level radio operation command in a chain of commands has finished.) Write zero to clear flag. Write to one has no effect."]
pub type LastCommandDoneR = crate::BitReader;
#[doc = "Field `LAST_COMMAND_DONE` writer - 1:1\\]
Interrupt flag 1. The last radio operation command in a chain of commands has finished. (IEEE 802.15.4 mode: The last background level radio operation command in a chain of commands has finished.) Write zero to clear flag. Write to one has no effect."]
pub type LastCommandDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FG_COMMAND_DONE` reader - 2:2\\]
Interrupt flag 2. IEEE 802.15.4 mode only: A foreground radio operation command has finished. Write zero to clear flag. Write to one has no effect."]
pub type FgCommandDoneR = crate::BitReader;
#[doc = "Field `FG_COMMAND_DONE` writer - 2:2\\]
Interrupt flag 2. IEEE 802.15.4 mode only: A foreground radio operation command has finished. Write zero to clear flag. Write to one has no effect."]
pub type FgCommandDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAST_FG_COMMAND_DONE` reader - 3:3\\]
Interrupt flag 3. IEEE 802.15.4 mode only: The last foreground radio operation command in a chain of commands has finished. Write zero to clear flag. Write to one has no effect."]
pub type LastFgCommandDoneR = crate::BitReader;
#[doc = "Field `LAST_FG_COMMAND_DONE` writer - 3:3\\]
Interrupt flag 3. IEEE 802.15.4 mode only: The last foreground radio operation command in a chain of commands has finished. Write zero to clear flag. Write to one has no effect."]
pub type LastFgCommandDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DONE` reader - 4:4\\]
Interrupt flag 4. Packet transmitted. (BLE mode: A packet has been transmitted.) (IEEE 802.15.4 mode: A frame has been transmitted). Write zero to clear flag. Write to one has no effect."]
pub type TxDoneR = crate::BitReader;
#[doc = "Field `TX_DONE` writer - 4:4\\]
Interrupt flag 4. Packet transmitted. (BLE mode: A packet has been transmitted.) (IEEE 802.15.4 mode: A frame has been transmitted). Write zero to clear flag. Write to one has no effect."]
pub type TxDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ACK` reader - 5:5\\]
Interrupt flag 5. BLE mode: Acknowledgement received on a transmitted packet. IEEE 802.15.4 mode: Transmitted automatic ACK frame. Write zero to clear flag. Write to one has no effect."]
pub type TxAckR = crate::BitReader;
#[doc = "Field `TX_ACK` writer - 5:5\\]
Interrupt flag 5. BLE mode: Acknowledgement received on a transmitted packet. IEEE 802.15.4 mode: Transmitted automatic ACK frame. Write zero to clear flag. Write to one has no effect."]
pub type TxAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CTRL` reader - 6:6\\]
Interrupt flag 6. BLE mode: Transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
pub type TxCtrlR = crate::BitReader;
#[doc = "Field `TX_CTRL` writer - 6:6\\]
Interrupt flag 6. BLE mode: Transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
pub type TxCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CTRL_ACK` reader - 7:7\\]
Interrupt flag 7. BLE mode: Acknowledgement received on a transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
pub type TxCtrlAckR = crate::BitReader;
#[doc = "Field `TX_CTRL_ACK` writer - 7:7\\]
Interrupt flag 7. BLE mode: Acknowledgement received on a transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
pub type TxCtrlAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CTRL_ACK_ACK` reader - 8:8\\]
Interrupt flag 8. BLE mode only: Acknowledgement received on a transmitted LL control packet, and acknowledgement transmitted for that packet. Write zero to clear flag. Write to one has no effect."]
pub type TxCtrlAckAckR = crate::BitReader;
#[doc = "Field `TX_CTRL_ACK_ACK` writer - 8:8\\]
Interrupt flag 8. BLE mode only: Acknowledgement received on a transmitted LL control packet, and acknowledgement transmitted for that packet. Write zero to clear flag. Write to one has no effect."]
pub type TxCtrlAckAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_RETRANS` reader - 9:9\\]
Interrupt flag 9. BLE mode only: Packet retransmitted. Write zero to clear flag. Write to one has no effect."]
pub type TxRetransR = crate::BitReader;
#[doc = "Field `TX_RETRANS` writer - 9:9\\]
Interrupt flag 9. BLE mode only: Packet retransmitted. Write zero to clear flag. Write to one has no effect."]
pub type TxRetransW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ENTRY_DONE` reader - 10:10\\]
Interrupt flag 10. Tx queue data entry state changed to finished. Write zero to clear flag. Write to one has no effect."]
pub type TxEntryDoneR = crate::BitReader;
#[doc = "Field `TX_ENTRY_DONE` writer - 10:10\\]
Interrupt flag 10. Tx queue data entry state changed to finished. Write zero to clear flag. Write to one has no effect."]
pub type TxEntryDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_BUFFER_CHANGED` reader - 11:11\\]
Interrupt flag 11. BLE mode only: A buffer change is complete after CMD_BLE_ADV_PAYLOAD. Write zero to clear flag. Write to one has no effect."]
pub type TxBufferChangedR = crate::BitReader;
#[doc = "Field `TX_BUFFER_CHANGED` writer - 11:11\\]
Interrupt flag 11. BLE mode only: A buffer change is complete after CMD_BLE_ADV_PAYLOAD. Write zero to clear flag. Write to one has no effect."]
pub type TxBufferChangedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ12` reader - 12:12\\]
Interrupt flag 12. Write zero to clear flag. Write to one has no effect."]
pub type Irq12R = crate::BitReader;
#[doc = "Field `IRQ12` writer - 12:12\\]
Interrupt flag 12. Write zero to clear flag. Write to one has no effect."]
pub type Irq12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ13` reader - 13:13\\]
Interrupt flag 13. Write zero to clear flag. Write to one has no effect."]
pub type Irq13R = crate::BitReader;
#[doc = "Field `IRQ13` writer - 13:13\\]
Interrupt flag 13. Write zero to clear flag. Write to one has no effect."]
pub type Irq13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ14` reader - 14:14\\]
Interrupt flag 14. Write zero to clear flag. Write to one has no effect."]
pub type Irq14R = crate::BitReader;
#[doc = "Field `IRQ14` writer - 14:14\\]
Interrupt flag 14. Write zero to clear flag. Write to one has no effect."]
pub type Irq14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ15` reader - 15:15\\]
Interrupt flag 15. Write zero to clear flag. Write to one has no effect."]
pub type Irq15R = crate::BitReader;
#[doc = "Field `IRQ15` writer - 15:15\\]
Interrupt flag 15. Write zero to clear flag. Write to one has no effect."]
pub type Irq15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OK` reader - 16:16\\]
Interrupt flag 16. Packet received correctly. BLE mode: Packet received with CRC OK, payload, and not to be ignored. IEEE 802.15.4 mode: Frame received with CRC OK. Write zero to clear flag. Write to one has no effect."]
pub type RxOkR = crate::BitReader;
#[doc = "Field `RX_OK` writer - 16:16\\]
Interrupt flag 16. Packet received correctly. BLE mode: Packet received with CRC OK, payload, and not to be ignored. IEEE 802.15.4 mode: Frame received with CRC OK. Write zero to clear flag. Write to one has no effect."]
pub type RxOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_NOK` reader - 17:17\\]
Interrupt flag 17. Packet received with CRC error. BLE mode: Packet received with CRC error. IEEE 802.15.4 mode: Frame received with CRC error. Write zero to clear flag. Write to one has no effect."]
pub type RxNokR = crate::BitReader;
#[doc = "Field `RX_NOK` writer - 17:17\\]
Interrupt flag 17. Packet received with CRC error. BLE mode: Packet received with CRC error. IEEE 802.15.4 mode: Frame received with CRC error. Write zero to clear flag. Write to one has no effect."]
pub type RxNokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_IGNORED` reader - 18:18\\]
Interrupt flag 18. Packet received, but can be ignored. BLE mode: Packet received with CRC OK, but to be ignored. IEEE 802.15.4 mode: Frame received with ignore flag set. Write zero to clear flag. Write to one has no effect."]
pub type RxIgnoredR = crate::BitReader;
#[doc = "Field `RX_IGNORED` writer - 18:18\\]
Interrupt flag 18. Packet received, but can be ignored. BLE mode: Packet received with CRC OK, but to be ignored. IEEE 802.15.4 mode: Frame received with ignore flag set. Write zero to clear flag. Write to one has no effect."]
pub type RxIgnoredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EMPTY` reader - 19:19\\]
Interrupt flag 19. BLE mode only: Packet received with CRC OK, not to be ignored, no payload. Write zero to clear flag. Write to one has no effect."]
pub type RxEmptyR = crate::BitReader;
#[doc = "Field `RX_EMPTY` writer - 19:19\\]
Interrupt flag 19. BLE mode only: Packet received with CRC OK, not to be ignored, no payload. Write zero to clear flag. Write to one has no effect."]
pub type RxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CTRL` reader - 20:20\\]
Interrupt flag 20. BLE mode only: LL control packet received with CRC OK, not to be ignored. Write zero to clear flag. Write to one has no effect."]
pub type RxCtrlR = crate::BitReader;
#[doc = "Field `RX_CTRL` writer - 20:20\\]
Interrupt flag 20. BLE mode only: LL control packet received with CRC OK, not to be ignored. Write zero to clear flag. Write to one has no effect."]
pub type RxCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CTRL_ACK` reader - 21:21\\]
Interrupt flag 21. BLE mode only: LL control packet received with CRC OK, not to be ignored, then acknowledgement sent. Write zero to clear flag. Write to one has no effect."]
pub type RxCtrlAckR = crate::BitReader;
#[doc = "Field `RX_CTRL_ACK` writer - 21:21\\]
Interrupt flag 21. BLE mode only: LL control packet received with CRC OK, not to be ignored, then acknowledgement sent. Write zero to clear flag. Write to one has no effect."]
pub type RxCtrlAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_BUF_FULL` reader - 22:22\\]
Interrupt flag 22. Packet received that did not fit in Rx queue. BLE mode: Packet received that did not fit in the Rx queue. IEEE 802.15.4 mode: Frame received that did not fit in the Rx queue. Write zero to clear flag. Write to one has no effect."]
pub type RxBufFullR = crate::BitReader;
#[doc = "Field `RX_BUF_FULL` writer - 22:22\\]
Interrupt flag 22. Packet received that did not fit in Rx queue. BLE mode: Packet received that did not fit in the Rx queue. IEEE 802.15.4 mode: Frame received that did not fit in the Rx queue. Write zero to clear flag. Write to one has no effect."]
pub type RxBufFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ENTRY_DONE` reader - 23:23\\]
Interrupt flag 23. Rx queue data entry changing state to finished. Write zero to clear flag. Write to one has no effect."]
pub type RxEntryDoneR = crate::BitReader;
#[doc = "Field `RX_ENTRY_DONE` writer - 23:23\\]
Interrupt flag 23. Rx queue data entry changing state to finished. Write zero to clear flag. Write to one has no effect."]
pub type RxEntryDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_WRITTEN` reader - 24:24\\]
Interrupt flag 24. Data written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
pub type RxDataWrittenR = crate::BitReader;
#[doc = "Field `RX_DATA_WRITTEN` writer - 24:24\\]
Interrupt flag 24. Data written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
pub type RxDataWrittenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_N_DATA_WRITTEN` reader - 25:25\\]
Interrupt flag 25. Specified number of bytes written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
pub type RxNDataWrittenR = crate::BitReader;
#[doc = "Field `RX_N_DATA_WRITTEN` writer - 25:25\\]
Interrupt flag 25. Specified number of bytes written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
pub type RxNDataWrittenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ABORTED` reader - 26:26\\]
Interrupt flag 26. Packet reception stopped before packet was done. Write zero to clear flag. Write to one has no effect."]
pub type RxAbortedR = crate::BitReader;
#[doc = "Field `RX_ABORTED` writer - 26:26\\]
Interrupt flag 26. Packet reception stopped before packet was done. Write zero to clear flag. Write to one has no effect."]
pub type RxAbortedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ27` reader - 27:27\\]
Interrupt flag 27. Write zero to clear flag. Write to one has no effect."]
pub type Irq27R = crate::BitReader;
#[doc = "Field `IRQ27` writer - 27:27\\]
Interrupt flag 27. Write zero to clear flag. Write to one has no effect."]
pub type Irq27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNTH_NO_LOCK` reader - 28:28\\]
Interrupt flag 28. The phase-locked loop in frequency synthesizer has reported loss of lock. Write zero to clear flag. Write to one has no effect."]
pub type SynthNoLockR = crate::BitReader;
#[doc = "Field `SYNTH_NO_LOCK` writer - 28:28\\]
Interrupt flag 28. The phase-locked loop in frequency synthesizer has reported loss of lock. Write zero to clear flag. Write to one has no effect."]
pub type SynthNoLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODULES_UNLOCKED` reader - 29:29\\]
Interrupt flag 29. As part of command and packet engine (CPE) boot process, it has opened access to RF Core modules and memories. Write zero to clear flag. Write to one has no effect."]
pub type ModulesUnlockedR = crate::BitReader;
#[doc = "Field `MODULES_UNLOCKED` writer - 29:29\\]
Interrupt flag 29. As part of command and packet engine (CPE) boot process, it has opened access to RF Core modules and memories. Write zero to clear flag. Write to one has no effect."]
pub type ModulesUnlockedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_DONE` reader - 30:30\\]
Interrupt flag 30. The command and packet engine (CPE) boot is finished. Write zero to clear flag. Write to one has no effect."]
pub type BootDoneR = crate::BitReader;
#[doc = "Field `BOOT_DONE` writer - 30:30\\]
Interrupt flag 30. The command and packet engine (CPE) boot is finished. Write zero to clear flag. Write to one has no effect."]
pub type BootDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERNAL_ERROR` reader - 31:31\\]
Interrupt flag 31. The command and packet engine (CPE) has observed an unexpected error. A reset of the CPE is needed. This can be done by switching the RF Core power domain off and on in PRCM:PDCTL1RFC. Write zero to clear flag. Write to one has no effect."]
pub type InternalErrorR = crate::BitReader;
#[doc = "Field `INTERNAL_ERROR` writer - 31:31\\]
Interrupt flag 31. The command and packet engine (CPE) has observed an unexpected error. A reset of the CPE is needed. This can be done by switching the RF Core power domain off and on in PRCM:PDCTL1RFC. Write zero to clear flag. Write to one has no effect."]
pub type InternalErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt flag 0. A radio operation has finished. (IEEE 802.15.4 mode: A background level radio operation command has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn command_done(&self) -> CommandDoneR {
        CommandDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt flag 1. The last radio operation command in a chain of commands has finished. (IEEE 802.15.4 mode: The last background level radio operation command in a chain of commands has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn last_command_done(&self) -> LastCommandDoneR {
        LastCommandDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt flag 2. IEEE 802.15.4 mode only: A foreground radio operation command has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn fg_command_done(&self) -> FgCommandDoneR {
        FgCommandDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt flag 3. IEEE 802.15.4 mode only: The last foreground radio operation command in a chain of commands has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn last_fg_command_done(&self) -> LastFgCommandDoneR {
        LastFgCommandDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt flag 4. Packet transmitted. (BLE mode: A packet has been transmitted.) (IEEE 802.15.4 mode: A frame has been transmitted). Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_done(&self) -> TxDoneR {
        TxDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt flag 5. BLE mode: Acknowledgement received on a transmitted packet. IEEE 802.15.4 mode: Transmitted automatic ACK frame. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ack(&self) -> TxAckR {
        TxAckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt flag 6. BLE mode: Transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ctrl(&self) -> TxCtrlR {
        TxCtrlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt flag 7. BLE mode: Acknowledgement received on a transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ctrl_ack(&self) -> TxCtrlAckR {
        TxCtrlAckR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt flag 8. BLE mode only: Acknowledgement received on a transmitted LL control packet, and acknowledgement transmitted for that packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&self) -> TxCtrlAckAckR {
        TxCtrlAckAckR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt flag 9. BLE mode only: Packet retransmitted. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_retrans(&self) -> TxRetransR {
        TxRetransR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt flag 10. Tx queue data entry state changed to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_entry_done(&self) -> TxEntryDoneR {
        TxEntryDoneR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt flag 11. BLE mode only: A buffer change is complete after CMD_BLE_ADV_PAYLOAD. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_buffer_changed(&self) -> TxBufferChangedR {
        TxBufferChangedR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt flag 12. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq12(&self) -> Irq12R {
        Irq12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt flag 13. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq13(&self) -> Irq13R {
        Irq13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt flag 14. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq14(&self) -> Irq14R {
        Irq14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt flag 15. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq15(&self) -> Irq15R {
        Irq15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt flag 16. Packet received correctly. BLE mode: Packet received with CRC OK, payload, and not to be ignored. IEEE 802.15.4 mode: Frame received with CRC OK. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ok(&self) -> RxOkR {
        RxOkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt flag 17. Packet received with CRC error. BLE mode: Packet received with CRC error. IEEE 802.15.4 mode: Frame received with CRC error. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_nok(&self) -> RxNokR {
        RxNokR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt flag 18. Packet received, but can be ignored. BLE mode: Packet received with CRC OK, but to be ignored. IEEE 802.15.4 mode: Frame received with ignore flag set. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ignored(&self) -> RxIgnoredR {
        RxIgnoredR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt flag 19. BLE mode only: Packet received with CRC OK, not to be ignored, no payload. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_empty(&self) -> RxEmptyR {
        RxEmptyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Interrupt flag 20. BLE mode only: LL control packet received with CRC OK, not to be ignored. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ctrl(&self) -> RxCtrlR {
        RxCtrlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Interrupt flag 21. BLE mode only: LL control packet received with CRC OK, not to be ignored, then acknowledgement sent. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ctrl_ack(&self) -> RxCtrlAckR {
        RxCtrlAckR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt flag 22. Packet received that did not fit in Rx queue. BLE mode: Packet received that did not fit in the Rx queue. IEEE 802.15.4 mode: Frame received that did not fit in the Rx queue. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_buf_full(&self) -> RxBufFullR {
        RxBufFullR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Interrupt flag 23. Rx queue data entry changing state to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_entry_done(&self) -> RxEntryDoneR {
        RxEntryDoneR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Interrupt flag 24. Data written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_data_written(&self) -> RxDataWrittenR {
        RxDataWrittenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Interrupt flag 25. Specified number of bytes written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_n_data_written(&self) -> RxNDataWrittenR {
        RxNDataWrittenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Interrupt flag 26. Packet reception stopped before packet was done. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_aborted(&self) -> RxAbortedR {
        RxAbortedR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Interrupt flag 27. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq27(&self) -> Irq27R {
        Irq27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Interrupt flag 28. The phase-locked loop in frequency synthesizer has reported loss of lock. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn synth_no_lock(&self) -> SynthNoLockR {
        SynthNoLockR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Interrupt flag 29. As part of command and packet engine (CPE) boot process, it has opened access to RF Core modules and memories. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn modules_unlocked(&self) -> ModulesUnlockedR {
        ModulesUnlockedR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Interrupt flag 30. The command and packet engine (CPE) boot is finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn boot_done(&self) -> BootDoneR {
        BootDoneR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Interrupt flag 31. The command and packet engine (CPE) has observed an unexpected error. A reset of the CPE is needed. This can be done by switching the RF Core power domain off and on in PRCM:PDCTL1RFC. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn internal_error(&self) -> InternalErrorR {
        InternalErrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt flag 0. A radio operation has finished. (IEEE 802.15.4 mode: A background level radio operation command has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn command_done(&mut self) -> CommandDoneW<RfcpeifgSpec> {
        CommandDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt flag 1. The last radio operation command in a chain of commands has finished. (IEEE 802.15.4 mode: The last background level radio operation command in a chain of commands has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn last_command_done(&mut self) -> LastCommandDoneW<RfcpeifgSpec> {
        LastCommandDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt flag 2. IEEE 802.15.4 mode only: A foreground radio operation command has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn fg_command_done(&mut self) -> FgCommandDoneW<RfcpeifgSpec> {
        FgCommandDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt flag 3. IEEE 802.15.4 mode only: The last foreground radio operation command in a chain of commands has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn last_fg_command_done(&mut self) -> LastFgCommandDoneW<RfcpeifgSpec> {
        LastFgCommandDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt flag 4. Packet transmitted. (BLE mode: A packet has been transmitted.) (IEEE 802.15.4 mode: A frame has been transmitted). Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_done(&mut self) -> TxDoneW<RfcpeifgSpec> {
        TxDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt flag 5. BLE mode: Acknowledgement received on a transmitted packet. IEEE 802.15.4 mode: Transmitted automatic ACK frame. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ack(&mut self) -> TxAckW<RfcpeifgSpec> {
        TxAckW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt flag 6. BLE mode: Transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl(&mut self) -> TxCtrlW<RfcpeifgSpec> {
        TxCtrlW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt flag 7. BLE mode: Acknowledgement received on a transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl_ack(&mut self) -> TxCtrlAckW<RfcpeifgSpec> {
        TxCtrlAckW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt flag 8. BLE mode only: Acknowledgement received on a transmitted LL control packet, and acknowledgement transmitted for that packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl_ack_ack(&mut self) -> TxCtrlAckAckW<RfcpeifgSpec> {
        TxCtrlAckAckW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt flag 9. BLE mode only: Packet retransmitted. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_retrans(&mut self) -> TxRetransW<RfcpeifgSpec> {
        TxRetransW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt flag 10. Tx queue data entry state changed to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_entry_done(&mut self) -> TxEntryDoneW<RfcpeifgSpec> {
        TxEntryDoneW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt flag 11. BLE mode only: A buffer change is complete after CMD_BLE_ADV_PAYLOAD. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_buffer_changed(&mut self) -> TxBufferChangedW<RfcpeifgSpec> {
        TxBufferChangedW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt flag 12. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn irq12(&mut self) -> Irq12W<RfcpeifgSpec> {
        Irq12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt flag 13. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn irq13(&mut self) -> Irq13W<RfcpeifgSpec> {
        Irq13W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt flag 14. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn irq14(&mut self) -> Irq14W<RfcpeifgSpec> {
        Irq14W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt flag 15. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn irq15(&mut self) -> Irq15W<RfcpeifgSpec> {
        Irq15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt flag 16. Packet received correctly. BLE mode: Packet received with CRC OK, payload, and not to be ignored. IEEE 802.15.4 mode: Frame received with CRC OK. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ok(&mut self) -> RxOkW<RfcpeifgSpec> {
        RxOkW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt flag 17. Packet received with CRC error. BLE mode: Packet received with CRC error. IEEE 802.15.4 mode: Frame received with CRC error. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_nok(&mut self) -> RxNokW<RfcpeifgSpec> {
        RxNokW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt flag 18. Packet received, but can be ignored. BLE mode: Packet received with CRC OK, but to be ignored. IEEE 802.15.4 mode: Frame received with ignore flag set. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ignored(&mut self) -> RxIgnoredW<RfcpeifgSpec> {
        RxIgnoredW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt flag 19. BLE mode only: Packet received with CRC OK, not to be ignored, no payload. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_empty(&mut self) -> RxEmptyW<RfcpeifgSpec> {
        RxEmptyW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Interrupt flag 20. BLE mode only: LL control packet received with CRC OK, not to be ignored. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ctrl(&mut self) -> RxCtrlW<RfcpeifgSpec> {
        RxCtrlW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Interrupt flag 21. BLE mode only: LL control packet received with CRC OK, not to be ignored, then acknowledgement sent. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ctrl_ack(&mut self) -> RxCtrlAckW<RfcpeifgSpec> {
        RxCtrlAckW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt flag 22. Packet received that did not fit in Rx queue. BLE mode: Packet received that did not fit in the Rx queue. IEEE 802.15.4 mode: Frame received that did not fit in the Rx queue. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_buf_full(&mut self) -> RxBufFullW<RfcpeifgSpec> {
        RxBufFullW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Interrupt flag 23. Rx queue data entry changing state to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_entry_done(&mut self) -> RxEntryDoneW<RfcpeifgSpec> {
        RxEntryDoneW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Interrupt flag 24. Data written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_written(&mut self) -> RxDataWrittenW<RfcpeifgSpec> {
        RxDataWrittenW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Interrupt flag 25. Specified number of bytes written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_n_data_written(&mut self) -> RxNDataWrittenW<RfcpeifgSpec> {
        RxNDataWrittenW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Interrupt flag 26. Packet reception stopped before packet was done. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_aborted(&mut self) -> RxAbortedW<RfcpeifgSpec> {
        RxAbortedW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Interrupt flag 27. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn irq27(&mut self) -> Irq27W<RfcpeifgSpec> {
        Irq27W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Interrupt flag 28. The phase-locked loop in frequency synthesizer has reported loss of lock. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn synth_no_lock(&mut self) -> SynthNoLockW<RfcpeifgSpec> {
        SynthNoLockW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Interrupt flag 29. As part of command and packet engine (CPE) boot process, it has opened access to RF Core modules and memories. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn modules_unlocked(&mut self) -> ModulesUnlockedW<RfcpeifgSpec> {
        ModulesUnlockedW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Interrupt flag 30. The command and packet engine (CPE) boot is finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn boot_done(&mut self) -> BootDoneW<RfcpeifgSpec> {
        BootDoneW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Interrupt flag 31. The command and packet engine (CPE) has observed an unexpected error. A reset of the CPE is needed. This can be done by switching the RF Core power domain off and on in PRCM:PDCTL1RFC. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn internal_error(&mut self) -> InternalErrorW<RfcpeifgSpec> {
        InternalErrorW::new(self, 31)
    }
}
#[doc = "Interrupt Flags For Command and Packet Engine Generated Interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcpeifg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcpeifg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcpeifgSpec;
impl crate::RegisterSpec for RfcpeifgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcpeifg::R`](R) reader structure"]
impl crate::Readable for RfcpeifgSpec {}
#[doc = "`write(|w| ..)` method takes [`rfcpeifg::W`](W) writer structure"]
impl crate::Writable for RfcpeifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFCPEIFG to value 0"]
impl crate::Resettable for RfcpeifgSpec {
    const RESET_VALUE: u32 = 0;
}
