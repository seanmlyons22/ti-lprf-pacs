#[doc = "Register `RFCPEIFG` reader"]
pub struct R(crate::R<RFCPEIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCPEIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCPEIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCPEIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFCPEIFG` writer"]
pub struct W(crate::W<RFCPEIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCPEIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RFCPEIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCPEIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND_DONE` reader - 0:0\\]
Interrupt flag 0. A radio operation has finished. (IEEE 802.15.4 mode: A background level radio operation command has finished.) Write zero to clear flag. Write to one has no effect."]
pub type COMMAND_DONE_R = crate::BitReader<bool>;
#[doc = "Field `COMMAND_DONE` writer - 0:0\\]
Interrupt flag 0. A radio operation has finished. (IEEE 802.15.4 mode: A background level radio operation command has finished.) Write zero to clear flag. Write to one has no effect."]
pub type COMMAND_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `LAST_COMMAND_DONE` reader - 1:1\\]
Interrupt flag 1. The last radio operation command in a chain of commands has finished. (IEEE 802.15.4 mode: The last background level radio operation command in a chain of commands has finished.) Write zero to clear flag. Write to one has no effect."]
pub type LAST_COMMAND_DONE_R = crate::BitReader<bool>;
#[doc = "Field `LAST_COMMAND_DONE` writer - 1:1\\]
Interrupt flag 1. The last radio operation command in a chain of commands has finished. (IEEE 802.15.4 mode: The last background level radio operation command in a chain of commands has finished.) Write zero to clear flag. Write to one has no effect."]
pub type LAST_COMMAND_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `FG_COMMAND_DONE` reader - 2:2\\]
Interrupt flag 2. IEEE 802.15.4 mode only: A foreground radio operation command has finished. Write zero to clear flag. Write to one has no effect."]
pub type FG_COMMAND_DONE_R = crate::BitReader<bool>;
#[doc = "Field `FG_COMMAND_DONE` writer - 2:2\\]
Interrupt flag 2. IEEE 802.15.4 mode only: A foreground radio operation command has finished. Write zero to clear flag. Write to one has no effect."]
pub type FG_COMMAND_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `LAST_FG_COMMAND_DONE` reader - 3:3\\]
Interrupt flag 3. IEEE 802.15.4 mode only: The last foreground radio operation command in a chain of commands has finished. Write zero to clear flag. Write to one has no effect."]
pub type LAST_FG_COMMAND_DONE_R = crate::BitReader<bool>;
#[doc = "Field `LAST_FG_COMMAND_DONE` writer - 3:3\\]
Interrupt flag 3. IEEE 802.15.4 mode only: The last foreground radio operation command in a chain of commands has finished. Write zero to clear flag. Write to one has no effect."]
pub type LAST_FG_COMMAND_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `TX_DONE` reader - 4:4\\]
Interrupt flag 4. Packet transmitted. (BLE mode: A packet has been transmitted.) (IEEE 802.15.4 mode: A frame has been transmitted). Write zero to clear flag. Write to one has no effect."]
pub type TX_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_DONE` writer - 4:4\\]
Interrupt flag 4. Packet transmitted. (BLE mode: A packet has been transmitted.) (IEEE 802.15.4 mode: A frame has been transmitted). Write zero to clear flag. Write to one has no effect."]
pub type TX_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `TX_ACK` reader - 5:5\\]
Interrupt flag 5. BLE mode: Acknowledgement received on a transmitted packet. IEEE 802.15.4 mode: Transmitted automatic ACK frame. Write zero to clear flag. Write to one has no effect."]
pub type TX_ACK_R = crate::BitReader<bool>;
#[doc = "Field `TX_ACK` writer - 5:5\\]
Interrupt flag 5. BLE mode: Acknowledgement received on a transmitted packet. IEEE 802.15.4 mode: Transmitted automatic ACK frame. Write zero to clear flag. Write to one has no effect."]
pub type TX_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `TX_CTRL` reader - 6:6\\]
Interrupt flag 6. BLE mode: Transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
pub type TX_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `TX_CTRL` writer - 6:6\\]
Interrupt flag 6. BLE mode: Transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
pub type TX_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `TX_CTRL_ACK` reader - 7:7\\]
Interrupt flag 7. BLE mode: Acknowledgement received on a transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
pub type TX_CTRL_ACK_R = crate::BitReader<bool>;
#[doc = "Field `TX_CTRL_ACK` writer - 7:7\\]
Interrupt flag 7. BLE mode: Acknowledgement received on a transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
pub type TX_CTRL_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `TX_CTRL_ACK_ACK` reader - 8:8\\]
Interrupt flag 8. BLE mode only: Acknowledgement received on a transmitted LL control packet, and acknowledgement transmitted for that packet. Write zero to clear flag. Write to one has no effect."]
pub type TX_CTRL_ACK_ACK_R = crate::BitReader<bool>;
#[doc = "Field `TX_CTRL_ACK_ACK` writer - 8:8\\]
Interrupt flag 8. BLE mode only: Acknowledgement received on a transmitted LL control packet, and acknowledgement transmitted for that packet. Write zero to clear flag. Write to one has no effect."]
pub type TX_CTRL_ACK_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `TX_RETRANS` reader - 9:9\\]
Interrupt flag 9. BLE mode only: Packet retransmitted. Write zero to clear flag. Write to one has no effect."]
pub type TX_RETRANS_R = crate::BitReader<bool>;
#[doc = "Field `TX_RETRANS` writer - 9:9\\]
Interrupt flag 9. BLE mode only: Packet retransmitted. Write zero to clear flag. Write to one has no effect."]
pub type TX_RETRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `TX_ENTRY_DONE` reader - 10:10\\]
Interrupt flag 10. Tx queue data entry state changed to finished. Write zero to clear flag. Write to one has no effect."]
pub type TX_ENTRY_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_ENTRY_DONE` writer - 10:10\\]
Interrupt flag 10. Tx queue data entry state changed to finished. Write zero to clear flag. Write to one has no effect."]
pub type TX_ENTRY_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `TX_BUFFER_CHANGED` reader - 11:11\\]
Interrupt flag 11. BLE mode only: A buffer change is complete after CMD_BLE_ADV_PAYLOAD. Write zero to clear flag. Write to one has no effect."]
pub type TX_BUFFER_CHANGED_R = crate::BitReader<bool>;
#[doc = "Field `TX_BUFFER_CHANGED` writer - 11:11\\]
Interrupt flag 11. BLE mode only: A buffer change is complete after CMD_BLE_ADV_PAYLOAD. Write zero to clear flag. Write to one has no effect."]
pub type TX_BUFFER_CHANGED_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `IRQ12` reader - 12:12\\]
Interrupt flag 12. Write zero to clear flag. Write to one has no effect."]
pub type IRQ12_R = crate::BitReader<bool>;
#[doc = "Field `IRQ12` writer - 12:12\\]
Interrupt flag 12. Write zero to clear flag. Write to one has no effect."]
pub type IRQ12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `IRQ13` reader - 13:13\\]
Interrupt flag 13. Write zero to clear flag. Write to one has no effect."]
pub type IRQ13_R = crate::BitReader<bool>;
#[doc = "Field `IRQ13` writer - 13:13\\]
Interrupt flag 13. Write zero to clear flag. Write to one has no effect."]
pub type IRQ13_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `IRQ14` reader - 14:14\\]
Interrupt flag 14. Write zero to clear flag. Write to one has no effect."]
pub type IRQ14_R = crate::BitReader<bool>;
#[doc = "Field `IRQ14` writer - 14:14\\]
Interrupt flag 14. Write zero to clear flag. Write to one has no effect."]
pub type IRQ14_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `IRQ15` reader - 15:15\\]
Interrupt flag 15. Write zero to clear flag. Write to one has no effect."]
pub type IRQ15_R = crate::BitReader<bool>;
#[doc = "Field `IRQ15` writer - 15:15\\]
Interrupt flag 15. Write zero to clear flag. Write to one has no effect."]
pub type IRQ15_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `RX_OK` reader - 16:16\\]
Interrupt flag 16. Packet received correctly. BLE mode: Packet received with CRC OK, payload, and not to be ignored. IEEE 802.15.4 mode: Frame received with CRC OK. Write zero to clear flag. Write to one has no effect."]
pub type RX_OK_R = crate::BitReader<bool>;
#[doc = "Field `RX_OK` writer - 16:16\\]
Interrupt flag 16. Packet received correctly. BLE mode: Packet received with CRC OK, payload, and not to be ignored. IEEE 802.15.4 mode: Frame received with CRC OK. Write zero to clear flag. Write to one has no effect."]
pub type RX_OK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `RX_NOK` reader - 17:17\\]
Interrupt flag 17. Packet received with CRC error. BLE mode: Packet received with CRC error. IEEE 802.15.4 mode: Frame received with CRC error. Write zero to clear flag. Write to one has no effect."]
pub type RX_NOK_R = crate::BitReader<bool>;
#[doc = "Field `RX_NOK` writer - 17:17\\]
Interrupt flag 17. Packet received with CRC error. BLE mode: Packet received with CRC error. IEEE 802.15.4 mode: Frame received with CRC error. Write zero to clear flag. Write to one has no effect."]
pub type RX_NOK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `RX_IGNORED` reader - 18:18\\]
Interrupt flag 18. Packet received, but can be ignored. BLE mode: Packet received with CRC OK, but to be ignored. IEEE 802.15.4 mode: Frame received with ignore flag set. Write zero to clear flag. Write to one has no effect."]
pub type RX_IGNORED_R = crate::BitReader<bool>;
#[doc = "Field `RX_IGNORED` writer - 18:18\\]
Interrupt flag 18. Packet received, but can be ignored. BLE mode: Packet received with CRC OK, but to be ignored. IEEE 802.15.4 mode: Frame received with ignore flag set. Write zero to clear flag. Write to one has no effect."]
pub type RX_IGNORED_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `RX_EMPTY` reader - 19:19\\]
Interrupt flag 19. BLE mode only: Packet received with CRC OK, not to be ignored, no payload. Write zero to clear flag. Write to one has no effect."]
pub type RX_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `RX_EMPTY` writer - 19:19\\]
Interrupt flag 19. BLE mode only: Packet received with CRC OK, not to be ignored, no payload. Write zero to clear flag. Write to one has no effect."]
pub type RX_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `RX_CTRL` reader - 20:20\\]
Interrupt flag 20. BLE mode only: LL control packet received with CRC OK, not to be ignored. Write zero to clear flag. Write to one has no effect."]
pub type RX_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `RX_CTRL` writer - 20:20\\]
Interrupt flag 20. BLE mode only: LL control packet received with CRC OK, not to be ignored. Write zero to clear flag. Write to one has no effect."]
pub type RX_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `RX_CTRL_ACK` reader - 21:21\\]
Interrupt flag 21. BLE mode only: LL control packet received with CRC OK, not to be ignored, then acknowledgement sent. Write zero to clear flag. Write to one has no effect."]
pub type RX_CTRL_ACK_R = crate::BitReader<bool>;
#[doc = "Field `RX_CTRL_ACK` writer - 21:21\\]
Interrupt flag 21. BLE mode only: LL control packet received with CRC OK, not to be ignored, then acknowledgement sent. Write zero to clear flag. Write to one has no effect."]
pub type RX_CTRL_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `RX_BUF_FULL` reader - 22:22\\]
Interrupt flag 22. Packet received that did not fit in Rx queue. BLE mode: Packet received that did not fit in the Rx queue. IEEE 802.15.4 mode: Frame received that did not fit in the Rx queue. Write zero to clear flag. Write to one has no effect."]
pub type RX_BUF_FULL_R = crate::BitReader<bool>;
#[doc = "Field `RX_BUF_FULL` writer - 22:22\\]
Interrupt flag 22. Packet received that did not fit in Rx queue. BLE mode: Packet received that did not fit in the Rx queue. IEEE 802.15.4 mode: Frame received that did not fit in the Rx queue. Write zero to clear flag. Write to one has no effect."]
pub type RX_BUF_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `RX_ENTRY_DONE` reader - 23:23\\]
Interrupt flag 23. Rx queue data entry changing state to finished. Write zero to clear flag. Write to one has no effect."]
pub type RX_ENTRY_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_ENTRY_DONE` writer - 23:23\\]
Interrupt flag 23. Rx queue data entry changing state to finished. Write zero to clear flag. Write to one has no effect."]
pub type RX_ENTRY_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `RX_DATA_WRITTEN` reader - 24:24\\]
Interrupt flag 24. Data written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
pub type RX_DATA_WRITTEN_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_WRITTEN` writer - 24:24\\]
Interrupt flag 24. Data written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
pub type RX_DATA_WRITTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `RX_N_DATA_WRITTEN` reader - 25:25\\]
Interrupt flag 25. Specified number of bytes written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
pub type RX_N_DATA_WRITTEN_R = crate::BitReader<bool>;
#[doc = "Field `RX_N_DATA_WRITTEN` writer - 25:25\\]
Interrupt flag 25. Specified number of bytes written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
pub type RX_N_DATA_WRITTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `RX_ABORTED` reader - 26:26\\]
Interrupt flag 26. Packet reception stopped before packet was done. Write zero to clear flag. Write to one has no effect."]
pub type RX_ABORTED_R = crate::BitReader<bool>;
#[doc = "Field `RX_ABORTED` writer - 26:26\\]
Interrupt flag 26. Packet reception stopped before packet was done. Write zero to clear flag. Write to one has no effect."]
pub type RX_ABORTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `IRQ27` reader - 27:27\\]
Interrupt flag 27. Write zero to clear flag. Write to one has no effect."]
pub type IRQ27_R = crate::BitReader<bool>;
#[doc = "Field `IRQ27` writer - 27:27\\]
Interrupt flag 27. Write zero to clear flag. Write to one has no effect."]
pub type IRQ27_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `SYNTH_NO_LOCK` reader - 28:28\\]
Interrupt flag 28. The phase-locked loop in frequency synthesizer has reported loss of lock. Write zero to clear flag. Write to one has no effect."]
pub type SYNTH_NO_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `SYNTH_NO_LOCK` writer - 28:28\\]
Interrupt flag 28. The phase-locked loop in frequency synthesizer has reported loss of lock. Write zero to clear flag. Write to one has no effect."]
pub type SYNTH_NO_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `MODULES_UNLOCKED` reader - 29:29\\]
Interrupt flag 29. As part of command and packet engine (CPE) boot process, it has opened access to RF Core modules and memories. Write zero to clear flag. Write to one has no effect."]
pub type MODULES_UNLOCKED_R = crate::BitReader<bool>;
#[doc = "Field `MODULES_UNLOCKED` writer - 29:29\\]
Interrupt flag 29. As part of command and packet engine (CPE) boot process, it has opened access to RF Core modules and memories. Write zero to clear flag. Write to one has no effect."]
pub type MODULES_UNLOCKED_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `BOOT_DONE` reader - 30:30\\]
Interrupt flag 30. The command and packet engine (CPE) boot is finished. Write zero to clear flag. Write to one has no effect."]
pub type BOOT_DONE_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_DONE` writer - 30:30\\]
Interrupt flag 30. The command and packet engine (CPE) boot is finished. Write zero to clear flag. Write to one has no effect."]
pub type BOOT_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
#[doc = "Field `INTERNAL_ERROR` reader - 31:31\\]
Interrupt flag 31. The command and packet engine (CPE) has observed an unexpected error. A reset of the CPE is needed. This can be done by switching the RF Core power domain off and on in PRCM:PDCTL1RFC. Write zero to clear flag. Write to one has no effect."]
pub type INTERNAL_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `INTERNAL_ERROR` writer - 31:31\\]
Interrupt flag 31. The command and packet engine (CPE) has observed an unexpected error. A reset of the CPE is needed. This can be done by switching the RF Core power domain off and on in PRCM:PDCTL1RFC. Write zero to clear flag. Write to one has no effect."]
pub type INTERNAL_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFCPEIFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt flag 0. A radio operation has finished. (IEEE 802.15.4 mode: A background level radio operation command has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt flag 1. The last radio operation command in a chain of commands has finished. (IEEE 802.15.4 mode: The last background level radio operation command in a chain of commands has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn last_command_done(&self) -> LAST_COMMAND_DONE_R {
        LAST_COMMAND_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt flag 2. IEEE 802.15.4 mode only: A foreground radio operation command has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn fg_command_done(&self) -> FG_COMMAND_DONE_R {
        FG_COMMAND_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt flag 3. IEEE 802.15.4 mode only: The last foreground radio operation command in a chain of commands has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn last_fg_command_done(&self) -> LAST_FG_COMMAND_DONE_R {
        LAST_FG_COMMAND_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt flag 4. Packet transmitted. (BLE mode: A packet has been transmitted.) (IEEE 802.15.4 mode: A frame has been transmitted). Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt flag 5. BLE mode: Acknowledgement received on a transmitted packet. IEEE 802.15.4 mode: Transmitted automatic ACK frame. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ack(&self) -> TX_ACK_R {
        TX_ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt flag 6. BLE mode: Transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ctrl(&self) -> TX_CTRL_R {
        TX_CTRL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt flag 7. BLE mode: Acknowledgement received on a transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ctrl_ack(&self) -> TX_CTRL_ACK_R {
        TX_CTRL_ACK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt flag 8. BLE mode only: Acknowledgement received on a transmitted LL control packet, and acknowledgement transmitted for that packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&self) -> TX_CTRL_ACK_ACK_R {
        TX_CTRL_ACK_ACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt flag 9. BLE mode only: Packet retransmitted. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_retrans(&self) -> TX_RETRANS_R {
        TX_RETRANS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt flag 10. Tx queue data entry state changed to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_entry_done(&self) -> TX_ENTRY_DONE_R {
        TX_ENTRY_DONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt flag 11. BLE mode only: A buffer change is complete after CMD_BLE_ADV_PAYLOAD. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_buffer_changed(&self) -> TX_BUFFER_CHANGED_R {
        TX_BUFFER_CHANGED_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt flag 12. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq12(&self) -> IRQ12_R {
        IRQ12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt flag 13. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq13(&self) -> IRQ13_R {
        IRQ13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt flag 14. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq14(&self) -> IRQ14_R {
        IRQ14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt flag 15. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq15(&self) -> IRQ15_R {
        IRQ15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt flag 16. Packet received correctly. BLE mode: Packet received with CRC OK, payload, and not to be ignored. IEEE 802.15.4 mode: Frame received with CRC OK. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ok(&self) -> RX_OK_R {
        RX_OK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt flag 17. Packet received with CRC error. BLE mode: Packet received with CRC error. IEEE 802.15.4 mode: Frame received with CRC error. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_nok(&self) -> RX_NOK_R {
        RX_NOK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt flag 18. Packet received, but can be ignored. BLE mode: Packet received with CRC OK, but to be ignored. IEEE 802.15.4 mode: Frame received with ignore flag set. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ignored(&self) -> RX_IGNORED_R {
        RX_IGNORED_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt flag 19. BLE mode only: Packet received with CRC OK, not to be ignored, no payload. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_empty(&self) -> RX_EMPTY_R {
        RX_EMPTY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Interrupt flag 20. BLE mode only: LL control packet received with CRC OK, not to be ignored. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ctrl(&self) -> RX_CTRL_R {
        RX_CTRL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Interrupt flag 21. BLE mode only: LL control packet received with CRC OK, not to be ignored, then acknowledgement sent. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ctrl_ack(&self) -> RX_CTRL_ACK_R {
        RX_CTRL_ACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt flag 22. Packet received that did not fit in Rx queue. BLE mode: Packet received that did not fit in the Rx queue. IEEE 802.15.4 mode: Frame received that did not fit in the Rx queue. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_buf_full(&self) -> RX_BUF_FULL_R {
        RX_BUF_FULL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Interrupt flag 23. Rx queue data entry changing state to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_entry_done(&self) -> RX_ENTRY_DONE_R {
        RX_ENTRY_DONE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Interrupt flag 24. Data written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_data_written(&self) -> RX_DATA_WRITTEN_R {
        RX_DATA_WRITTEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Interrupt flag 25. Specified number of bytes written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_n_data_written(&self) -> RX_N_DATA_WRITTEN_R {
        RX_N_DATA_WRITTEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Interrupt flag 26. Packet reception stopped before packet was done. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_aborted(&self) -> RX_ABORTED_R {
        RX_ABORTED_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Interrupt flag 27. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq27(&self) -> IRQ27_R {
        IRQ27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Interrupt flag 28. The phase-locked loop in frequency synthesizer has reported loss of lock. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn synth_no_lock(&self) -> SYNTH_NO_LOCK_R {
        SYNTH_NO_LOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Interrupt flag 29. As part of command and packet engine (CPE) boot process, it has opened access to RF Core modules and memories. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn modules_unlocked(&self) -> MODULES_UNLOCKED_R {
        MODULES_UNLOCKED_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Interrupt flag 30. The command and packet engine (CPE) boot is finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn boot_done(&self) -> BOOT_DONE_R {
        BOOT_DONE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Interrupt flag 31. The command and packet engine (CPE) has observed an unexpected error. A reset of the CPE is needed. This can be done by switching the RF Core power domain off and on in PRCM:PDCTL1RFC. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn internal_error(&self) -> INTERNAL_ERROR_R {
        INTERNAL_ERROR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt flag 0. A radio operation has finished. (IEEE 802.15.4 mode: A background level radio operation command has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn command_done(&mut self) -> COMMAND_DONE_W<0> {
        COMMAND_DONE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt flag 1. The last radio operation command in a chain of commands has finished. (IEEE 802.15.4 mode: The last background level radio operation command in a chain of commands has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn last_command_done(&mut self) -> LAST_COMMAND_DONE_W<1> {
        LAST_COMMAND_DONE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt flag 2. IEEE 802.15.4 mode only: A foreground radio operation command has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn fg_command_done(&mut self) -> FG_COMMAND_DONE_W<2> {
        FG_COMMAND_DONE_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt flag 3. IEEE 802.15.4 mode only: The last foreground radio operation command in a chain of commands has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn last_fg_command_done(&mut self) -> LAST_FG_COMMAND_DONE_W<3> {
        LAST_FG_COMMAND_DONE_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt flag 4. Packet transmitted. (BLE mode: A packet has been transmitted.) (IEEE 802.15.4 mode: A frame has been transmitted). Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_done(&mut self) -> TX_DONE_W<4> {
        TX_DONE_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt flag 5. BLE mode: Acknowledgement received on a transmitted packet. IEEE 802.15.4 mode: Transmitted automatic ACK frame. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ack(&mut self) -> TX_ACK_W<5> {
        TX_ACK_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt flag 6. BLE mode: Transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl(&mut self) -> TX_CTRL_W<6> {
        TX_CTRL_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt flag 7. BLE mode: Acknowledgement received on a transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl_ack(&mut self) -> TX_CTRL_ACK_W<7> {
        TX_CTRL_ACK_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt flag 8. BLE mode only: Acknowledgement received on a transmitted LL control packet, and acknowledgement transmitted for that packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl_ack_ack(&mut self) -> TX_CTRL_ACK_ACK_W<8> {
        TX_CTRL_ACK_ACK_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt flag 9. BLE mode only: Packet retransmitted. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_retrans(&mut self) -> TX_RETRANS_W<9> {
        TX_RETRANS_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt flag 10. Tx queue data entry state changed to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_entry_done(&mut self) -> TX_ENTRY_DONE_W<10> {
        TX_ENTRY_DONE_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt flag 11. BLE mode only: A buffer change is complete after CMD_BLE_ADV_PAYLOAD. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tx_buffer_changed(&mut self) -> TX_BUFFER_CHANGED_W<11> {
        TX_BUFFER_CHANGED_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt flag 12. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn irq12(&mut self) -> IRQ12_W<12> {
        IRQ12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt flag 13. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn irq13(&mut self) -> IRQ13_W<13> {
        IRQ13_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt flag 14. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn irq14(&mut self) -> IRQ14_W<14> {
        IRQ14_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt flag 15. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn irq15(&mut self) -> IRQ15_W<15> {
        IRQ15_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt flag 16. Packet received correctly. BLE mode: Packet received with CRC OK, payload, and not to be ignored. IEEE 802.15.4 mode: Frame received with CRC OK. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ok(&mut self) -> RX_OK_W<16> {
        RX_OK_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt flag 17. Packet received with CRC error. BLE mode: Packet received with CRC error. IEEE 802.15.4 mode: Frame received with CRC error. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_nok(&mut self) -> RX_NOK_W<17> {
        RX_NOK_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt flag 18. Packet received, but can be ignored. BLE mode: Packet received with CRC OK, but to be ignored. IEEE 802.15.4 mode: Frame received with ignore flag set. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ignored(&mut self) -> RX_IGNORED_W<18> {
        RX_IGNORED_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt flag 19. BLE mode only: Packet received with CRC OK, not to be ignored, no payload. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_empty(&mut self) -> RX_EMPTY_W<19> {
        RX_EMPTY_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Interrupt flag 20. BLE mode only: LL control packet received with CRC OK, not to be ignored. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ctrl(&mut self) -> RX_CTRL_W<20> {
        RX_CTRL_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Interrupt flag 21. BLE mode only: LL control packet received with CRC OK, not to be ignored, then acknowledgement sent. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ctrl_ack(&mut self) -> RX_CTRL_ACK_W<21> {
        RX_CTRL_ACK_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt flag 22. Packet received that did not fit in Rx queue. BLE mode: Packet received that did not fit in the Rx queue. IEEE 802.15.4 mode: Frame received that did not fit in the Rx queue. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_buf_full(&mut self) -> RX_BUF_FULL_W<22> {
        RX_BUF_FULL_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Interrupt flag 23. Rx queue data entry changing state to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_entry_done(&mut self) -> RX_ENTRY_DONE_W<23> {
        RX_ENTRY_DONE_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Interrupt flag 24. Data written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_written(&mut self) -> RX_DATA_WRITTEN_W<24> {
        RX_DATA_WRITTEN_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Interrupt flag 25. Specified number of bytes written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_n_data_written(&mut self) -> RX_N_DATA_WRITTEN_W<25> {
        RX_N_DATA_WRITTEN_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Interrupt flag 26. Packet reception stopped before packet was done. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rx_aborted(&mut self) -> RX_ABORTED_W<26> {
        RX_ABORTED_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Interrupt flag 27. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn irq27(&mut self) -> IRQ27_W<27> {
        IRQ27_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Interrupt flag 28. The phase-locked loop in frequency synthesizer has reported loss of lock. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn synth_no_lock(&mut self) -> SYNTH_NO_LOCK_W<28> {
        SYNTH_NO_LOCK_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Interrupt flag 29. As part of command and packet engine (CPE) boot process, it has opened access to RF Core modules and memories. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn modules_unlocked(&mut self) -> MODULES_UNLOCKED_W<29> {
        MODULES_UNLOCKED_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Interrupt flag 30. The command and packet engine (CPE) boot is finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn boot_done(&mut self) -> BOOT_DONE_W<30> {
        BOOT_DONE_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Interrupt flag 31. The command and packet engine (CPE) has observed an unexpected error. A reset of the CPE is needed. This can be done by switching the RF Core power domain off and on in PRCM:PDCTL1RFC. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn internal_error(&mut self) -> INTERNAL_ERROR_W<31> {
        INTERNAL_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags For Command and Packet Engine Generated Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcpeifg](index.html) module"]
pub struct RFCPEIFG_SPEC;
impl crate::RegisterSpec for RFCPEIFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcpeifg::R](R) reader structure"]
impl crate::Readable for RFCPEIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcpeifg::W](W) writer structure"]
impl crate::Writable for RFCPEIFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFCPEIFG to value 0"]
impl crate::Resettable for RFCPEIFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
