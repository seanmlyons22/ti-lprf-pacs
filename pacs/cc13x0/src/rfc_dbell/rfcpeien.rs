#[doc = "Register `RFCPEIEN` reader"]
pub type R = crate::R<RfcpeienSpec>;
#[doc = "Register `RFCPEIEN` writer"]
pub type W = crate::W<RfcpeienSpec>;
#[doc = "Field `COMMAND_DONE` reader - 0:0\\]
Interrupt enable for RFCPEIFG.COMMAND_DONE."]
pub type CommandDoneR = crate::BitReader;
#[doc = "Field `COMMAND_DONE` writer - 0:0\\]
Interrupt enable for RFCPEIFG.COMMAND_DONE."]
pub type CommandDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAST_COMMAND_DONE` reader - 1:1\\]
Interrupt enable for RFCPEIFG.LAST_COMMAND_DONE."]
pub type LastCommandDoneR = crate::BitReader;
#[doc = "Field `LAST_COMMAND_DONE` writer - 1:1\\]
Interrupt enable for RFCPEIFG.LAST_COMMAND_DONE."]
pub type LastCommandDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FG_COMMAND_DONE` reader - 2:2\\]
Interrupt enable for RFCPEIFG.FG_COMMAND_DONE."]
pub type FgCommandDoneR = crate::BitReader;
#[doc = "Field `FG_COMMAND_DONE` writer - 2:2\\]
Interrupt enable for RFCPEIFG.FG_COMMAND_DONE."]
pub type FgCommandDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAST_FG_COMMAND_DONE` reader - 3:3\\]
Interrupt enable for RFCPEIFG.LAST_FG_COMMAND_DONE."]
pub type LastFgCommandDoneR = crate::BitReader;
#[doc = "Field `LAST_FG_COMMAND_DONE` writer - 3:3\\]
Interrupt enable for RFCPEIFG.LAST_FG_COMMAND_DONE."]
pub type LastFgCommandDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DONE` reader - 4:4\\]
Interrupt enable for RFCPEIFG.TX_DONE."]
pub type TxDoneR = crate::BitReader;
#[doc = "Field `TX_DONE` writer - 4:4\\]
Interrupt enable for RFCPEIFG.TX_DONE."]
pub type TxDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ACK` reader - 5:5\\]
Interrupt enable for RFCPEIFG.TX_ACK."]
pub type TxAckR = crate::BitReader;
#[doc = "Field `TX_ACK` writer - 5:5\\]
Interrupt enable for RFCPEIFG.TX_ACK."]
pub type TxAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CTRL` reader - 6:6\\]
Interrupt enable for RFCPEIFG.TX_CTRL."]
pub type TxCtrlR = crate::BitReader;
#[doc = "Field `TX_CTRL` writer - 6:6\\]
Interrupt enable for RFCPEIFG.TX_CTRL."]
pub type TxCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CTRL_ACK` reader - 7:7\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK."]
pub type TxCtrlAckR = crate::BitReader;
#[doc = "Field `TX_CTRL_ACK` writer - 7:7\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK."]
pub type TxCtrlAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CTRL_ACK_ACK` reader - 8:8\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK_ACK."]
pub type TxCtrlAckAckR = crate::BitReader;
#[doc = "Field `TX_CTRL_ACK_ACK` writer - 8:8\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK_ACK."]
pub type TxCtrlAckAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_RETRANS` reader - 9:9\\]
Interrupt enable for RFCPEIFG.TX_RETRANS."]
pub type TxRetransR = crate::BitReader;
#[doc = "Field `TX_RETRANS` writer - 9:9\\]
Interrupt enable for RFCPEIFG.TX_RETRANS."]
pub type TxRetransW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ENTRY_DONE` reader - 10:10\\]
Interrupt enable for RFCPEIFG.TX_ENTRY_DONE."]
pub type TxEntryDoneR = crate::BitReader;
#[doc = "Field `TX_ENTRY_DONE` writer - 10:10\\]
Interrupt enable for RFCPEIFG.TX_ENTRY_DONE."]
pub type TxEntryDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_BUFFER_CHANGED` reader - 11:11\\]
Interrupt enable for RFCPEIFG.TX_BUFFER_CHANGED."]
pub type TxBufferChangedR = crate::BitReader;
#[doc = "Field `TX_BUFFER_CHANGED` writer - 11:11\\]
Interrupt enable for RFCPEIFG.TX_BUFFER_CHANGED."]
pub type TxBufferChangedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ12` reader - 12:12\\]
Interrupt enable for RFCPEIFG.IRQ12."]
pub type Irq12R = crate::BitReader;
#[doc = "Field `IRQ12` writer - 12:12\\]
Interrupt enable for RFCPEIFG.IRQ12."]
pub type Irq12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ13` reader - 13:13\\]
Interrupt enable for RFCPEIFG.IRQ13."]
pub type Irq13R = crate::BitReader;
#[doc = "Field `IRQ13` writer - 13:13\\]
Interrupt enable for RFCPEIFG.IRQ13."]
pub type Irq13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ14` reader - 14:14\\]
Interrupt enable for RFCPEIFG.IRQ14."]
pub type Irq14R = crate::BitReader;
#[doc = "Field `IRQ14` writer - 14:14\\]
Interrupt enable for RFCPEIFG.IRQ14."]
pub type Irq14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ15` reader - 15:15\\]
Interrupt enable for RFCPEIFG.IRQ15."]
pub type Irq15R = crate::BitReader;
#[doc = "Field `IRQ15` writer - 15:15\\]
Interrupt enable for RFCPEIFG.IRQ15."]
pub type Irq15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OK` reader - 16:16\\]
Interrupt enable for RFCPEIFG.RX_OK."]
pub type RxOkR = crate::BitReader;
#[doc = "Field `RX_OK` writer - 16:16\\]
Interrupt enable for RFCPEIFG.RX_OK."]
pub type RxOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_NOK` reader - 17:17\\]
Interrupt enable for RFCPEIFG.RX_NOK."]
pub type RxNokR = crate::BitReader;
#[doc = "Field `RX_NOK` writer - 17:17\\]
Interrupt enable for RFCPEIFG.RX_NOK."]
pub type RxNokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_IGNORED` reader - 18:18\\]
Interrupt enable for RFCPEIFG.RX_IGNORED."]
pub type RxIgnoredR = crate::BitReader;
#[doc = "Field `RX_IGNORED` writer - 18:18\\]
Interrupt enable for RFCPEIFG.RX_IGNORED."]
pub type RxIgnoredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EMPTY` reader - 19:19\\]
Interrupt enable for RFCPEIFG.RX_EMPTY."]
pub type RxEmptyR = crate::BitReader;
#[doc = "Field `RX_EMPTY` writer - 19:19\\]
Interrupt enable for RFCPEIFG.RX_EMPTY."]
pub type RxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CTRL` reader - 20:20\\]
Interrupt enable for RFCPEIFG.RX_CTRL."]
pub type RxCtrlR = crate::BitReader;
#[doc = "Field `RX_CTRL` writer - 20:20\\]
Interrupt enable for RFCPEIFG.RX_CTRL."]
pub type RxCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CTRL_ACK` reader - 21:21\\]
Interrupt enable for RFCPEIFG.RX_CTRL_ACK."]
pub type RxCtrlAckR = crate::BitReader;
#[doc = "Field `RX_CTRL_ACK` writer - 21:21\\]
Interrupt enable for RFCPEIFG.RX_CTRL_ACK."]
pub type RxCtrlAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_BUF_FULL` reader - 22:22\\]
Interrupt enable for RFCPEIFG.RX_BUF_FULL."]
pub type RxBufFullR = crate::BitReader;
#[doc = "Field `RX_BUF_FULL` writer - 22:22\\]
Interrupt enable for RFCPEIFG.RX_BUF_FULL."]
pub type RxBufFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ENTRY_DONE` reader - 23:23\\]
Interrupt enable for RFCPEIFG.RX_ENTRY_DONE."]
pub type RxEntryDoneR = crate::BitReader;
#[doc = "Field `RX_ENTRY_DONE` writer - 23:23\\]
Interrupt enable for RFCPEIFG.RX_ENTRY_DONE."]
pub type RxEntryDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_WRITTEN` reader - 24:24\\]
Interrupt enable for RFCPEIFG.RX_DATA_WRITTEN."]
pub type RxDataWrittenR = crate::BitReader;
#[doc = "Field `RX_DATA_WRITTEN` writer - 24:24\\]
Interrupt enable for RFCPEIFG.RX_DATA_WRITTEN."]
pub type RxDataWrittenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_N_DATA_WRITTEN` reader - 25:25\\]
Interrupt enable for RFCPEIFG.RX_N_DATA_WRITTEN."]
pub type RxNDataWrittenR = crate::BitReader;
#[doc = "Field `RX_N_DATA_WRITTEN` writer - 25:25\\]
Interrupt enable for RFCPEIFG.RX_N_DATA_WRITTEN."]
pub type RxNDataWrittenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ABORTED` reader - 26:26\\]
Interrupt enable for RFCPEIFG.RX_ABORTED."]
pub type RxAbortedR = crate::BitReader;
#[doc = "Field `RX_ABORTED` writer - 26:26\\]
Interrupt enable for RFCPEIFG.RX_ABORTED."]
pub type RxAbortedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ27` reader - 27:27\\]
Interrupt enable for RFCPEIFG.IRQ27."]
pub type Irq27R = crate::BitReader;
#[doc = "Field `IRQ27` writer - 27:27\\]
Interrupt enable for RFCPEIFG.IRQ27."]
pub type Irq27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNTH_NO_LOCK` reader - 28:28\\]
Interrupt enable for RFCPEIFG.SYNTH_NO_LOCK."]
pub type SynthNoLockR = crate::BitReader;
#[doc = "Field `SYNTH_NO_LOCK` writer - 28:28\\]
Interrupt enable for RFCPEIFG.SYNTH_NO_LOCK."]
pub type SynthNoLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODULES_UNLOCKED` reader - 29:29\\]
Interrupt enable for RFCPEIFG.MODULES_UNLOCKED."]
pub type ModulesUnlockedR = crate::BitReader;
#[doc = "Field `MODULES_UNLOCKED` writer - 29:29\\]
Interrupt enable for RFCPEIFG.MODULES_UNLOCKED."]
pub type ModulesUnlockedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_DONE` reader - 30:30\\]
Interrupt enable for RFCPEIFG.BOOT_DONE."]
pub type BootDoneR = crate::BitReader;
#[doc = "Field `BOOT_DONE` writer - 30:30\\]
Interrupt enable for RFCPEIFG.BOOT_DONE."]
pub type BootDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERNAL_ERROR` reader - 31:31\\]
Interrupt enable for RFCPEIFG.INTERNAL_ERROR."]
pub type InternalErrorR = crate::BitReader;
#[doc = "Field `INTERNAL_ERROR` writer - 31:31\\]
Interrupt enable for RFCPEIFG.INTERNAL_ERROR."]
pub type InternalErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt enable for RFCPEIFG.COMMAND_DONE."]
    #[inline(always)]
    pub fn command_done(&self) -> CommandDoneR {
        CommandDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt enable for RFCPEIFG.LAST_COMMAND_DONE."]
    #[inline(always)]
    pub fn last_command_done(&self) -> LastCommandDoneR {
        LastCommandDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt enable for RFCPEIFG.FG_COMMAND_DONE."]
    #[inline(always)]
    pub fn fg_command_done(&self) -> FgCommandDoneR {
        FgCommandDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt enable for RFCPEIFG.LAST_FG_COMMAND_DONE."]
    #[inline(always)]
    pub fn last_fg_command_done(&self) -> LastFgCommandDoneR {
        LastFgCommandDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt enable for RFCPEIFG.TX_DONE."]
    #[inline(always)]
    pub fn tx_done(&self) -> TxDoneR {
        TxDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt enable for RFCPEIFG.TX_ACK."]
    #[inline(always)]
    pub fn tx_ack(&self) -> TxAckR {
        TxAckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt enable for RFCPEIFG.TX_CTRL."]
    #[inline(always)]
    pub fn tx_ctrl(&self) -> TxCtrlR {
        TxCtrlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK."]
    #[inline(always)]
    pub fn tx_ctrl_ack(&self) -> TxCtrlAckR {
        TxCtrlAckR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK_ACK."]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&self) -> TxCtrlAckAckR {
        TxCtrlAckAckR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt enable for RFCPEIFG.TX_RETRANS."]
    #[inline(always)]
    pub fn tx_retrans(&self) -> TxRetransR {
        TxRetransR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt enable for RFCPEIFG.TX_ENTRY_DONE."]
    #[inline(always)]
    pub fn tx_entry_done(&self) -> TxEntryDoneR {
        TxEntryDoneR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt enable for RFCPEIFG.TX_BUFFER_CHANGED."]
    #[inline(always)]
    pub fn tx_buffer_changed(&self) -> TxBufferChangedR {
        TxBufferChangedR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt enable for RFCPEIFG.IRQ12."]
    #[inline(always)]
    pub fn irq12(&self) -> Irq12R {
        Irq12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt enable for RFCPEIFG.IRQ13."]
    #[inline(always)]
    pub fn irq13(&self) -> Irq13R {
        Irq13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt enable for RFCPEIFG.IRQ14."]
    #[inline(always)]
    pub fn irq14(&self) -> Irq14R {
        Irq14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt enable for RFCPEIFG.IRQ15."]
    #[inline(always)]
    pub fn irq15(&self) -> Irq15R {
        Irq15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt enable for RFCPEIFG.RX_OK."]
    #[inline(always)]
    pub fn rx_ok(&self) -> RxOkR {
        RxOkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt enable for RFCPEIFG.RX_NOK."]
    #[inline(always)]
    pub fn rx_nok(&self) -> RxNokR {
        RxNokR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt enable for RFCPEIFG.RX_IGNORED."]
    #[inline(always)]
    pub fn rx_ignored(&self) -> RxIgnoredR {
        RxIgnoredR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt enable for RFCPEIFG.RX_EMPTY."]
    #[inline(always)]
    pub fn rx_empty(&self) -> RxEmptyR {
        RxEmptyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Interrupt enable for RFCPEIFG.RX_CTRL."]
    #[inline(always)]
    pub fn rx_ctrl(&self) -> RxCtrlR {
        RxCtrlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Interrupt enable for RFCPEIFG.RX_CTRL_ACK."]
    #[inline(always)]
    pub fn rx_ctrl_ack(&self) -> RxCtrlAckR {
        RxCtrlAckR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt enable for RFCPEIFG.RX_BUF_FULL."]
    #[inline(always)]
    pub fn rx_buf_full(&self) -> RxBufFullR {
        RxBufFullR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Interrupt enable for RFCPEIFG.RX_ENTRY_DONE."]
    #[inline(always)]
    pub fn rx_entry_done(&self) -> RxEntryDoneR {
        RxEntryDoneR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Interrupt enable for RFCPEIFG.RX_DATA_WRITTEN."]
    #[inline(always)]
    pub fn rx_data_written(&self) -> RxDataWrittenR {
        RxDataWrittenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Interrupt enable for RFCPEIFG.RX_N_DATA_WRITTEN."]
    #[inline(always)]
    pub fn rx_n_data_written(&self) -> RxNDataWrittenR {
        RxNDataWrittenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Interrupt enable for RFCPEIFG.RX_ABORTED."]
    #[inline(always)]
    pub fn rx_aborted(&self) -> RxAbortedR {
        RxAbortedR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Interrupt enable for RFCPEIFG.IRQ27."]
    #[inline(always)]
    pub fn irq27(&self) -> Irq27R {
        Irq27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Interrupt enable for RFCPEIFG.SYNTH_NO_LOCK."]
    #[inline(always)]
    pub fn synth_no_lock(&self) -> SynthNoLockR {
        SynthNoLockR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Interrupt enable for RFCPEIFG.MODULES_UNLOCKED."]
    #[inline(always)]
    pub fn modules_unlocked(&self) -> ModulesUnlockedR {
        ModulesUnlockedR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Interrupt enable for RFCPEIFG.BOOT_DONE."]
    #[inline(always)]
    pub fn boot_done(&self) -> BootDoneR {
        BootDoneR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Interrupt enable for RFCPEIFG.INTERNAL_ERROR."]
    #[inline(always)]
    pub fn internal_error(&self) -> InternalErrorR {
        InternalErrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt enable for RFCPEIFG.COMMAND_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn command_done(&mut self) -> CommandDoneW<RfcpeienSpec> {
        CommandDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt enable for RFCPEIFG.LAST_COMMAND_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn last_command_done(&mut self) -> LastCommandDoneW<RfcpeienSpec> {
        LastCommandDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt enable for RFCPEIFG.FG_COMMAND_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn fg_command_done(&mut self) -> FgCommandDoneW<RfcpeienSpec> {
        FgCommandDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt enable for RFCPEIFG.LAST_FG_COMMAND_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn last_fg_command_done(&mut self) -> LastFgCommandDoneW<RfcpeienSpec> {
        LastFgCommandDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt enable for RFCPEIFG.TX_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn tx_done(&mut self) -> TxDoneW<RfcpeienSpec> {
        TxDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt enable for RFCPEIFG.TX_ACK."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ack(&mut self) -> TxAckW<RfcpeienSpec> {
        TxAckW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt enable for RFCPEIFG.TX_CTRL."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl(&mut self) -> TxCtrlW<RfcpeienSpec> {
        TxCtrlW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl_ack(&mut self) -> TxCtrlAckW<RfcpeienSpec> {
        TxCtrlAckW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK_ACK."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl_ack_ack(&mut self) -> TxCtrlAckAckW<RfcpeienSpec> {
        TxCtrlAckAckW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt enable for RFCPEIFG.TX_RETRANS."]
    #[inline(always)]
    #[must_use]
    pub fn tx_retrans(&mut self) -> TxRetransW<RfcpeienSpec> {
        TxRetransW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt enable for RFCPEIFG.TX_ENTRY_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn tx_entry_done(&mut self) -> TxEntryDoneW<RfcpeienSpec> {
        TxEntryDoneW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt enable for RFCPEIFG.TX_BUFFER_CHANGED."]
    #[inline(always)]
    #[must_use]
    pub fn tx_buffer_changed(&mut self) -> TxBufferChangedW<RfcpeienSpec> {
        TxBufferChangedW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt enable for RFCPEIFG.IRQ12."]
    #[inline(always)]
    #[must_use]
    pub fn irq12(&mut self) -> Irq12W<RfcpeienSpec> {
        Irq12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt enable for RFCPEIFG.IRQ13."]
    #[inline(always)]
    #[must_use]
    pub fn irq13(&mut self) -> Irq13W<RfcpeienSpec> {
        Irq13W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt enable for RFCPEIFG.IRQ14."]
    #[inline(always)]
    #[must_use]
    pub fn irq14(&mut self) -> Irq14W<RfcpeienSpec> {
        Irq14W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt enable for RFCPEIFG.IRQ15."]
    #[inline(always)]
    #[must_use]
    pub fn irq15(&mut self) -> Irq15W<RfcpeienSpec> {
        Irq15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt enable for RFCPEIFG.RX_OK."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ok(&mut self) -> RxOkW<RfcpeienSpec> {
        RxOkW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt enable for RFCPEIFG.RX_NOK."]
    #[inline(always)]
    #[must_use]
    pub fn rx_nok(&mut self) -> RxNokW<RfcpeienSpec> {
        RxNokW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt enable for RFCPEIFG.RX_IGNORED."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ignored(&mut self) -> RxIgnoredW<RfcpeienSpec> {
        RxIgnoredW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt enable for RFCPEIFG.RX_EMPTY."]
    #[inline(always)]
    #[must_use]
    pub fn rx_empty(&mut self) -> RxEmptyW<RfcpeienSpec> {
        RxEmptyW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Interrupt enable for RFCPEIFG.RX_CTRL."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ctrl(&mut self) -> RxCtrlW<RfcpeienSpec> {
        RxCtrlW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Interrupt enable for RFCPEIFG.RX_CTRL_ACK."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ctrl_ack(&mut self) -> RxCtrlAckW<RfcpeienSpec> {
        RxCtrlAckW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt enable for RFCPEIFG.RX_BUF_FULL."]
    #[inline(always)]
    #[must_use]
    pub fn rx_buf_full(&mut self) -> RxBufFullW<RfcpeienSpec> {
        RxBufFullW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Interrupt enable for RFCPEIFG.RX_ENTRY_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn rx_entry_done(&mut self) -> RxEntryDoneW<RfcpeienSpec> {
        RxEntryDoneW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Interrupt enable for RFCPEIFG.RX_DATA_WRITTEN."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_written(&mut self) -> RxDataWrittenW<RfcpeienSpec> {
        RxDataWrittenW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Interrupt enable for RFCPEIFG.RX_N_DATA_WRITTEN."]
    #[inline(always)]
    #[must_use]
    pub fn rx_n_data_written(&mut self) -> RxNDataWrittenW<RfcpeienSpec> {
        RxNDataWrittenW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Interrupt enable for RFCPEIFG.RX_ABORTED."]
    #[inline(always)]
    #[must_use]
    pub fn rx_aborted(&mut self) -> RxAbortedW<RfcpeienSpec> {
        RxAbortedW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Interrupt enable for RFCPEIFG.IRQ27."]
    #[inline(always)]
    #[must_use]
    pub fn irq27(&mut self) -> Irq27W<RfcpeienSpec> {
        Irq27W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Interrupt enable for RFCPEIFG.SYNTH_NO_LOCK."]
    #[inline(always)]
    #[must_use]
    pub fn synth_no_lock(&mut self) -> SynthNoLockW<RfcpeienSpec> {
        SynthNoLockW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Interrupt enable for RFCPEIFG.MODULES_UNLOCKED."]
    #[inline(always)]
    #[must_use]
    pub fn modules_unlocked(&mut self) -> ModulesUnlockedW<RfcpeienSpec> {
        ModulesUnlockedW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Interrupt enable for RFCPEIFG.BOOT_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn boot_done(&mut self) -> BootDoneW<RfcpeienSpec> {
        BootDoneW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Interrupt enable for RFCPEIFG.INTERNAL_ERROR."]
    #[inline(always)]
    #[must_use]
    pub fn internal_error(&mut self) -> InternalErrorW<RfcpeienSpec> {
        InternalErrorW::new(self, 31)
    }
}
#[doc = "Interrupt Enable For Command and Packet Engine Generated Interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcpeien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcpeien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcpeienSpec;
impl crate::RegisterSpec for RfcpeienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcpeien::R`](R) reader structure"]
impl crate::Readable for RfcpeienSpec {}
#[doc = "`write(|w| ..)` method takes [`rfcpeien::W`](W) writer structure"]
impl crate::Writable for RfcpeienSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFCPEIEN to value 0xffff_ffff"]
impl crate::Resettable for RfcpeienSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
