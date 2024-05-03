#[doc = "Register `RFCPEISL` reader"]
pub type R = crate::R<RfcpeislSpec>;
#[doc = "Register `RFCPEISL` writer"]
pub type W = crate::W<RfcpeislSpec>;
#[doc = "0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandDone {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<CommandDone> for bool {
    #[inline(always)]
    fn from(variant: CommandDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMMAND_DONE` reader - 0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
pub type CommandDoneR = crate::BitReader<CommandDone>;
impl CommandDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CommandDone {
        match self.bits {
            true => CommandDone::Cpe1,
            false => CommandDone::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == CommandDone::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == CommandDone::Cpe0
    }
}
#[doc = "Field `COMMAND_DONE` writer - 0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
pub type CommandDoneW<'a, REG> = crate::BitWriter<'a, REG, CommandDone>;
impl<'a, REG> CommandDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(CommandDone::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(CommandDone::Cpe0)
    }
}
#[doc = "1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LastCommandDone {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<LastCommandDone> for bool {
    #[inline(always)]
    fn from(variant: LastCommandDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LAST_COMMAND_DONE` reader - 1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
pub type LastCommandDoneR = crate::BitReader<LastCommandDone>;
impl LastCommandDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LastCommandDone {
        match self.bits {
            true => LastCommandDone::Cpe1,
            false => LastCommandDone::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == LastCommandDone::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == LastCommandDone::Cpe0
    }
}
#[doc = "Field `LAST_COMMAND_DONE` writer - 1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
pub type LastCommandDoneW<'a, REG> = crate::BitWriter<'a, REG, LastCommandDone>;
impl<'a, REG> LastCommandDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(LastCommandDone::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(LastCommandDone::Cpe0)
    }
}
#[doc = "2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FgCommandDone {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<FgCommandDone> for bool {
    #[inline(always)]
    fn from(variant: FgCommandDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FG_COMMAND_DONE` reader - 2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
pub type FgCommandDoneR = crate::BitReader<FgCommandDone>;
impl FgCommandDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FgCommandDone {
        match self.bits {
            true => FgCommandDone::Cpe1,
            false => FgCommandDone::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == FgCommandDone::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == FgCommandDone::Cpe0
    }
}
#[doc = "Field `FG_COMMAND_DONE` writer - 2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
pub type FgCommandDoneW<'a, REG> = crate::BitWriter<'a, REG, FgCommandDone>;
impl<'a, REG> FgCommandDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(FgCommandDone::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(FgCommandDone::Cpe0)
    }
}
#[doc = "3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LastFgCommandDone {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<LastFgCommandDone> for bool {
    #[inline(always)]
    fn from(variant: LastFgCommandDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LAST_FG_COMMAND_DONE` reader - 3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
pub type LastFgCommandDoneR = crate::BitReader<LastFgCommandDone>;
impl LastFgCommandDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LastFgCommandDone {
        match self.bits {
            true => LastFgCommandDone::Cpe1,
            false => LastFgCommandDone::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == LastFgCommandDone::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == LastFgCommandDone::Cpe0
    }
}
#[doc = "Field `LAST_FG_COMMAND_DONE` writer - 3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
pub type LastFgCommandDoneW<'a, REG> = crate::BitWriter<'a, REG, LastFgCommandDone>;
impl<'a, REG> LastFgCommandDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(LastFgCommandDone::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(LastFgCommandDone::Cpe0)
    }
}
#[doc = "4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxDone {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<TxDone> for bool {
    #[inline(always)]
    fn from(variant: TxDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_DONE` reader - 4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
pub type TxDoneR = crate::BitReader<TxDone>;
impl TxDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxDone {
        match self.bits {
            true => TxDone::Cpe1,
            false => TxDone::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TxDone::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TxDone::Cpe0
    }
}
#[doc = "Field `TX_DONE` writer - 4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
pub type TxDoneW<'a, REG> = crate::BitWriter<'a, REG, TxDone>;
impl<'a, REG> TxDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(TxDone::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(TxDone::Cpe0)
    }
}
#[doc = "5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxAck {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<TxAck> for bool {
    #[inline(always)]
    fn from(variant: TxAck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_ACK` reader - 5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
pub type TxAckR = crate::BitReader<TxAck>;
impl TxAckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxAck {
        match self.bits {
            true => TxAck::Cpe1,
            false => TxAck::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TxAck::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TxAck::Cpe0
    }
}
#[doc = "Field `TX_ACK` writer - 5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
pub type TxAckW<'a, REG> = crate::BitWriter<'a, REG, TxAck>;
impl<'a, REG> TxAckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(TxAck::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(TxAck::Cpe0)
    }
}
#[doc = "6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxCtrl {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<TxCtrl> for bool {
    #[inline(always)]
    fn from(variant: TxCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_CTRL` reader - 6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
pub type TxCtrlR = crate::BitReader<TxCtrl>;
impl TxCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxCtrl {
        match self.bits {
            true => TxCtrl::Cpe1,
            false => TxCtrl::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TxCtrl::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TxCtrl::Cpe0
    }
}
#[doc = "Field `TX_CTRL` writer - 6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
pub type TxCtrlW<'a, REG> = crate::BitWriter<'a, REG, TxCtrl>;
impl<'a, REG> TxCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(TxCtrl::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(TxCtrl::Cpe0)
    }
}
#[doc = "7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxCtrlAck {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<TxCtrlAck> for bool {
    #[inline(always)]
    fn from(variant: TxCtrlAck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_CTRL_ACK` reader - 7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
pub type TxCtrlAckR = crate::BitReader<TxCtrlAck>;
impl TxCtrlAckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxCtrlAck {
        match self.bits {
            true => TxCtrlAck::Cpe1,
            false => TxCtrlAck::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TxCtrlAck::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TxCtrlAck::Cpe0
    }
}
#[doc = "Field `TX_CTRL_ACK` writer - 7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
pub type TxCtrlAckW<'a, REG> = crate::BitWriter<'a, REG, TxCtrlAck>;
impl<'a, REG> TxCtrlAckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(TxCtrlAck::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(TxCtrlAck::Cpe0)
    }
}
#[doc = "8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxCtrlAckAck {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<TxCtrlAckAck> for bool {
    #[inline(always)]
    fn from(variant: TxCtrlAckAck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_CTRL_ACK_ACK` reader - 8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
pub type TxCtrlAckAckR = crate::BitReader<TxCtrlAckAck>;
impl TxCtrlAckAckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxCtrlAckAck {
        match self.bits {
            true => TxCtrlAckAck::Cpe1,
            false => TxCtrlAckAck::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TxCtrlAckAck::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TxCtrlAckAck::Cpe0
    }
}
#[doc = "Field `TX_CTRL_ACK_ACK` writer - 8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
pub type TxCtrlAckAckW<'a, REG> = crate::BitWriter<'a, REG, TxCtrlAckAck>;
impl<'a, REG> TxCtrlAckAckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(TxCtrlAckAck::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(TxCtrlAckAck::Cpe0)
    }
}
#[doc = "9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxRetrans {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<TxRetrans> for bool {
    #[inline(always)]
    fn from(variant: TxRetrans) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_RETRANS` reader - 9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
pub type TxRetransR = crate::BitReader<TxRetrans>;
impl TxRetransR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxRetrans {
        match self.bits {
            true => TxRetrans::Cpe1,
            false => TxRetrans::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TxRetrans::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TxRetrans::Cpe0
    }
}
#[doc = "Field `TX_RETRANS` writer - 9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
pub type TxRetransW<'a, REG> = crate::BitWriter<'a, REG, TxRetrans>;
impl<'a, REG> TxRetransW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(TxRetrans::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(TxRetrans::Cpe0)
    }
}
#[doc = "10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxEntryDone {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<TxEntryDone> for bool {
    #[inline(always)]
    fn from(variant: TxEntryDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_ENTRY_DONE` reader - 10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
pub type TxEntryDoneR = crate::BitReader<TxEntryDone>;
impl TxEntryDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxEntryDone {
        match self.bits {
            true => TxEntryDone::Cpe1,
            false => TxEntryDone::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TxEntryDone::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TxEntryDone::Cpe0
    }
}
#[doc = "Field `TX_ENTRY_DONE` writer - 10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
pub type TxEntryDoneW<'a, REG> = crate::BitWriter<'a, REG, TxEntryDone>;
impl<'a, REG> TxEntryDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(TxEntryDone::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(TxEntryDone::Cpe0)
    }
}
#[doc = "11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxBufferChanged {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<TxBufferChanged> for bool {
    #[inline(always)]
    fn from(variant: TxBufferChanged) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_BUFFER_CHANGED` reader - 11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
pub type TxBufferChangedR = crate::BitReader<TxBufferChanged>;
impl TxBufferChangedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxBufferChanged {
        match self.bits {
            true => TxBufferChanged::Cpe1,
            false => TxBufferChanged::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TxBufferChanged::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TxBufferChanged::Cpe0
    }
}
#[doc = "Field `TX_BUFFER_CHANGED` writer - 11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
pub type TxBufferChangedW<'a, REG> = crate::BitWriter<'a, REG, TxBufferChanged>;
impl<'a, REG> TxBufferChangedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(TxBufferChanged::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(TxBufferChanged::Cpe0)
    }
}
#[doc = "12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irq12 {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<Irq12> for bool {
    #[inline(always)]
    fn from(variant: Irq12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ12` reader - 12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
pub type Irq12R = crate::BitReader<Irq12>;
impl Irq12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irq12 {
        match self.bits {
            true => Irq12::Cpe1,
            false => Irq12::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == Irq12::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == Irq12::Cpe0
    }
}
#[doc = "Field `IRQ12` writer - 12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
pub type Irq12W<'a, REG> = crate::BitWriter<'a, REG, Irq12>;
impl<'a, REG> Irq12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(Irq12::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(Irq12::Cpe0)
    }
}
#[doc = "13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irq13 {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<Irq13> for bool {
    #[inline(always)]
    fn from(variant: Irq13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ13` reader - 13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
pub type Irq13R = crate::BitReader<Irq13>;
impl Irq13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irq13 {
        match self.bits {
            true => Irq13::Cpe1,
            false => Irq13::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == Irq13::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == Irq13::Cpe0
    }
}
#[doc = "Field `IRQ13` writer - 13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
pub type Irq13W<'a, REG> = crate::BitWriter<'a, REG, Irq13>;
impl<'a, REG> Irq13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(Irq13::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(Irq13::Cpe0)
    }
}
#[doc = "14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irq14 {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<Irq14> for bool {
    #[inline(always)]
    fn from(variant: Irq14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ14` reader - 14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
pub type Irq14R = crate::BitReader<Irq14>;
impl Irq14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irq14 {
        match self.bits {
            true => Irq14::Cpe1,
            false => Irq14::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == Irq14::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == Irq14::Cpe0
    }
}
#[doc = "Field `IRQ14` writer - 14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
pub type Irq14W<'a, REG> = crate::BitWriter<'a, REG, Irq14>;
impl<'a, REG> Irq14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(Irq14::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(Irq14::Cpe0)
    }
}
#[doc = "15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irq15 {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<Irq15> for bool {
    #[inline(always)]
    fn from(variant: Irq15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ15` reader - 15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
pub type Irq15R = crate::BitReader<Irq15>;
impl Irq15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irq15 {
        match self.bits {
            true => Irq15::Cpe1,
            false => Irq15::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == Irq15::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == Irq15::Cpe0
    }
}
#[doc = "Field `IRQ15` writer - 15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
pub type Irq15W<'a, REG> = crate::BitWriter<'a, REG, Irq15>;
impl<'a, REG> Irq15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(Irq15::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(Irq15::Cpe0)
    }
}
#[doc = "16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxOk {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<RxOk> for bool {
    #[inline(always)]
    fn from(variant: RxOk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_OK` reader - 16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
pub type RxOkR = crate::BitReader<RxOk>;
impl RxOkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxOk {
        match self.bits {
            true => RxOk::Cpe1,
            false => RxOk::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RxOk::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RxOk::Cpe0
    }
}
#[doc = "Field `RX_OK` writer - 16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
pub type RxOkW<'a, REG> = crate::BitWriter<'a, REG, RxOk>;
impl<'a, REG> RxOkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(RxOk::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(RxOk::Cpe0)
    }
}
#[doc = "17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxNok {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<RxNok> for bool {
    #[inline(always)]
    fn from(variant: RxNok) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_NOK` reader - 17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
pub type RxNokR = crate::BitReader<RxNok>;
impl RxNokR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxNok {
        match self.bits {
            true => RxNok::Cpe1,
            false => RxNok::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RxNok::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RxNok::Cpe0
    }
}
#[doc = "Field `RX_NOK` writer - 17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
pub type RxNokW<'a, REG> = crate::BitWriter<'a, REG, RxNok>;
impl<'a, REG> RxNokW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(RxNok::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(RxNok::Cpe0)
    }
}
#[doc = "18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxIgnored {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<RxIgnored> for bool {
    #[inline(always)]
    fn from(variant: RxIgnored) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_IGNORED` reader - 18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
pub type RxIgnoredR = crate::BitReader<RxIgnored>;
impl RxIgnoredR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxIgnored {
        match self.bits {
            true => RxIgnored::Cpe1,
            false => RxIgnored::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RxIgnored::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RxIgnored::Cpe0
    }
}
#[doc = "Field `RX_IGNORED` writer - 18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
pub type RxIgnoredW<'a, REG> = crate::BitWriter<'a, REG, RxIgnored>;
impl<'a, REG> RxIgnoredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(RxIgnored::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(RxIgnored::Cpe0)
    }
}
#[doc = "19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxEmpty {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<RxEmpty> for bool {
    #[inline(always)]
    fn from(variant: RxEmpty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_EMPTY` reader - 19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
pub type RxEmptyR = crate::BitReader<RxEmpty>;
impl RxEmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxEmpty {
        match self.bits {
            true => RxEmpty::Cpe1,
            false => RxEmpty::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RxEmpty::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RxEmpty::Cpe0
    }
}
#[doc = "Field `RX_EMPTY` writer - 19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
pub type RxEmptyW<'a, REG> = crate::BitWriter<'a, REG, RxEmpty>;
impl<'a, REG> RxEmptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(RxEmpty::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(RxEmpty::Cpe0)
    }
}
#[doc = "20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxCtrl {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<RxCtrl> for bool {
    #[inline(always)]
    fn from(variant: RxCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_CTRL` reader - 20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
pub type RxCtrlR = crate::BitReader<RxCtrl>;
impl RxCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxCtrl {
        match self.bits {
            true => RxCtrl::Cpe1,
            false => RxCtrl::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RxCtrl::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RxCtrl::Cpe0
    }
}
#[doc = "Field `RX_CTRL` writer - 20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
pub type RxCtrlW<'a, REG> = crate::BitWriter<'a, REG, RxCtrl>;
impl<'a, REG> RxCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(RxCtrl::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(RxCtrl::Cpe0)
    }
}
#[doc = "21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxCtrlAck {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<RxCtrlAck> for bool {
    #[inline(always)]
    fn from(variant: RxCtrlAck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_CTRL_ACK` reader - 21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
pub type RxCtrlAckR = crate::BitReader<RxCtrlAck>;
impl RxCtrlAckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxCtrlAck {
        match self.bits {
            true => RxCtrlAck::Cpe1,
            false => RxCtrlAck::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RxCtrlAck::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RxCtrlAck::Cpe0
    }
}
#[doc = "Field `RX_CTRL_ACK` writer - 21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
pub type RxCtrlAckW<'a, REG> = crate::BitWriter<'a, REG, RxCtrlAck>;
impl<'a, REG> RxCtrlAckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(RxCtrlAck::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(RxCtrlAck::Cpe0)
    }
}
#[doc = "22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxBufFull {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<RxBufFull> for bool {
    #[inline(always)]
    fn from(variant: RxBufFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_BUF_FULL` reader - 22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
pub type RxBufFullR = crate::BitReader<RxBufFull>;
impl RxBufFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxBufFull {
        match self.bits {
            true => RxBufFull::Cpe1,
            false => RxBufFull::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RxBufFull::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RxBufFull::Cpe0
    }
}
#[doc = "Field `RX_BUF_FULL` writer - 22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
pub type RxBufFullW<'a, REG> = crate::BitWriter<'a, REG, RxBufFull>;
impl<'a, REG> RxBufFullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(RxBufFull::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(RxBufFull::Cpe0)
    }
}
#[doc = "23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxEntryDone {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<RxEntryDone> for bool {
    #[inline(always)]
    fn from(variant: RxEntryDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_ENTRY_DONE` reader - 23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
pub type RxEntryDoneR = crate::BitReader<RxEntryDone>;
impl RxEntryDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxEntryDone {
        match self.bits {
            true => RxEntryDone::Cpe1,
            false => RxEntryDone::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RxEntryDone::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RxEntryDone::Cpe0
    }
}
#[doc = "Field `RX_ENTRY_DONE` writer - 23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
pub type RxEntryDoneW<'a, REG> = crate::BitWriter<'a, REG, RxEntryDone>;
impl<'a, REG> RxEntryDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(RxEntryDone::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(RxEntryDone::Cpe0)
    }
}
#[doc = "24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxDataWritten {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<RxDataWritten> for bool {
    #[inline(always)]
    fn from(variant: RxDataWritten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_DATA_WRITTEN` reader - 24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
pub type RxDataWrittenR = crate::BitReader<RxDataWritten>;
impl RxDataWrittenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxDataWritten {
        match self.bits {
            true => RxDataWritten::Cpe1,
            false => RxDataWritten::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RxDataWritten::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RxDataWritten::Cpe0
    }
}
#[doc = "Field `RX_DATA_WRITTEN` writer - 24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
pub type RxDataWrittenW<'a, REG> = crate::BitWriter<'a, REG, RxDataWritten>;
impl<'a, REG> RxDataWrittenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(RxDataWritten::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(RxDataWritten::Cpe0)
    }
}
#[doc = "25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxNDataWritten {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<RxNDataWritten> for bool {
    #[inline(always)]
    fn from(variant: RxNDataWritten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_N_DATA_WRITTEN` reader - 25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
pub type RxNDataWrittenR = crate::BitReader<RxNDataWritten>;
impl RxNDataWrittenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxNDataWritten {
        match self.bits {
            true => RxNDataWritten::Cpe1,
            false => RxNDataWritten::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RxNDataWritten::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RxNDataWritten::Cpe0
    }
}
#[doc = "Field `RX_N_DATA_WRITTEN` writer - 25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
pub type RxNDataWrittenW<'a, REG> = crate::BitWriter<'a, REG, RxNDataWritten>;
impl<'a, REG> RxNDataWrittenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(RxNDataWritten::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(RxNDataWritten::Cpe0)
    }
}
#[doc = "26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxAborted {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<RxAborted> for bool {
    #[inline(always)]
    fn from(variant: RxAborted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_ABORTED` reader - 26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
pub type RxAbortedR = crate::BitReader<RxAborted>;
impl RxAbortedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxAborted {
        match self.bits {
            true => RxAborted::Cpe1,
            false => RxAborted::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RxAborted::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RxAborted::Cpe0
    }
}
#[doc = "Field `RX_ABORTED` writer - 26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
pub type RxAbortedW<'a, REG> = crate::BitWriter<'a, REG, RxAborted>;
impl<'a, REG> RxAbortedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(RxAborted::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(RxAborted::Cpe0)
    }
}
#[doc = "27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irq27 {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<Irq27> for bool {
    #[inline(always)]
    fn from(variant: Irq27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ27` reader - 27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
pub type Irq27R = crate::BitReader<Irq27>;
impl Irq27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irq27 {
        match self.bits {
            true => Irq27::Cpe1,
            false => Irq27::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == Irq27::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == Irq27::Cpe0
    }
}
#[doc = "Field `IRQ27` writer - 27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
pub type Irq27W<'a, REG> = crate::BitWriter<'a, REG, Irq27>;
impl<'a, REG> Irq27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(Irq27::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(Irq27::Cpe0)
    }
}
#[doc = "28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SynthNoLock {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<SynthNoLock> for bool {
    #[inline(always)]
    fn from(variant: SynthNoLock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNTH_NO_LOCK` reader - 28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
pub type SynthNoLockR = crate::BitReader<SynthNoLock>;
impl SynthNoLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SynthNoLock {
        match self.bits {
            true => SynthNoLock::Cpe1,
            false => SynthNoLock::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == SynthNoLock::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == SynthNoLock::Cpe0
    }
}
#[doc = "Field `SYNTH_NO_LOCK` writer - 28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
pub type SynthNoLockW<'a, REG> = crate::BitWriter<'a, REG, SynthNoLock>;
impl<'a, REG> SynthNoLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(SynthNoLock::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(SynthNoLock::Cpe0)
    }
}
#[doc = "29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModulesUnlocked {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<ModulesUnlocked> for bool {
    #[inline(always)]
    fn from(variant: ModulesUnlocked) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODULES_UNLOCKED` reader - 29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
pub type ModulesUnlockedR = crate::BitReader<ModulesUnlocked>;
impl ModulesUnlockedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ModulesUnlocked {
        match self.bits {
            true => ModulesUnlocked::Cpe1,
            false => ModulesUnlocked::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == ModulesUnlocked::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == ModulesUnlocked::Cpe0
    }
}
#[doc = "Field `MODULES_UNLOCKED` writer - 29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
pub type ModulesUnlockedW<'a, REG> = crate::BitWriter<'a, REG, ModulesUnlocked>;
impl<'a, REG> ModulesUnlockedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(ModulesUnlocked::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(ModulesUnlocked::Cpe0)
    }
}
#[doc = "30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootDone {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<BootDone> for bool {
    #[inline(always)]
    fn from(variant: BootDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOT_DONE` reader - 30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
pub type BootDoneR = crate::BitReader<BootDone>;
impl BootDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BootDone {
        match self.bits {
            true => BootDone::Cpe1,
            false => BootDone::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == BootDone::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == BootDone::Cpe0
    }
}
#[doc = "Field `BOOT_DONE` writer - 30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
pub type BootDoneW<'a, REG> = crate::BitWriter<'a, REG, BootDone>;
impl<'a, REG> BootDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(BootDone::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(BootDone::Cpe0)
    }
}
#[doc = "31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InternalError {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    Cpe1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    Cpe0 = 0,
}
impl From<InternalError> for bool {
    #[inline(always)]
    fn from(variant: InternalError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTERNAL_ERROR` reader - 31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
pub type InternalErrorR = crate::BitReader<InternalError>;
impl InternalErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InternalError {
        match self.bits {
            true => InternalError::Cpe1,
            false => InternalError::Cpe0,
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == InternalError::Cpe1
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == InternalError::Cpe0
    }
}
#[doc = "Field `INTERNAL_ERROR` writer - 31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
pub type InternalErrorW<'a, REG> = crate::BitWriter<'a, REG, InternalError>;
impl<'a, REG> InternalErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut crate::W<REG> {
        self.variant(InternalError::Cpe1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut crate::W<REG> {
        self.variant(InternalError::Cpe0)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn command_done(&self) -> CommandDoneR {
        CommandDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn last_command_done(&self) -> LastCommandDoneR {
        LastCommandDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn fg_command_done(&self) -> FgCommandDoneR {
        FgCommandDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn last_fg_command_done(&self) -> LastFgCommandDoneR {
        LastFgCommandDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
    #[inline(always)]
    pub fn tx_done(&self) -> TxDoneR {
        TxDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ack(&self) -> TxAckR {
        TxAckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl(&self) -> TxCtrlR {
        TxCtrlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl_ack(&self) -> TxCtrlAckR {
        TxCtrlAckR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&self) -> TxCtrlAckAckR {
        TxCtrlAckAckR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
    #[inline(always)]
    pub fn tx_retrans(&self) -> TxRetransR {
        TxRetransR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    pub fn tx_entry_done(&self) -> TxEntryDoneR {
        TxEntryDoneR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
    #[inline(always)]
    pub fn tx_buffer_changed(&self) -> TxBufferChangedR {
        TxBufferChangedR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
    #[inline(always)]
    pub fn irq12(&self) -> Irq12R {
        Irq12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
    #[inline(always)]
    pub fn irq13(&self) -> Irq13R {
        Irq13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
    #[inline(always)]
    pub fn irq14(&self) -> Irq14R {
        Irq14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
    #[inline(always)]
    pub fn irq15(&self) -> Irq15R {
        Irq15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
    #[inline(always)]
    pub fn rx_ok(&self) -> RxOkR {
        RxOkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
    #[inline(always)]
    pub fn rx_nok(&self) -> RxNokR {
        RxNokR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
    #[inline(always)]
    pub fn rx_ignored(&self) -> RxIgnoredR {
        RxIgnoredR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
    #[inline(always)]
    pub fn rx_empty(&self) -> RxEmptyR {
        RxEmptyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
    #[inline(always)]
    pub fn rx_ctrl(&self) -> RxCtrlR {
        RxCtrlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    pub fn rx_ctrl_ack(&self) -> RxCtrlAckR {
        RxCtrlAckR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
    #[inline(always)]
    pub fn rx_buf_full(&self) -> RxBufFullR {
        RxBufFullR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    pub fn rx_entry_done(&self) -> RxEntryDoneR {
        RxEntryDoneR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    pub fn rx_data_written(&self) -> RxDataWrittenR {
        RxDataWrittenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    pub fn rx_n_data_written(&self) -> RxNDataWrittenR {
        RxNDataWrittenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
    #[inline(always)]
    pub fn rx_aborted(&self) -> RxAbortedR {
        RxAbortedR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
    #[inline(always)]
    pub fn irq27(&self) -> Irq27R {
        Irq27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
    #[inline(always)]
    pub fn synth_no_lock(&self) -> SynthNoLockR {
        SynthNoLockR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
    #[inline(always)]
    pub fn modules_unlocked(&self) -> ModulesUnlockedR {
        ModulesUnlockedR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
    #[inline(always)]
    pub fn boot_done(&self) -> BootDoneR {
        BootDoneR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
    #[inline(always)]
    pub fn internal_error(&self) -> InternalErrorR {
        InternalErrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn command_done(&mut self) -> CommandDoneW<RfcpeislSpec> {
        CommandDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn last_command_done(&mut self) -> LastCommandDoneW<RfcpeislSpec> {
        LastCommandDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn fg_command_done(&mut self) -> FgCommandDoneW<RfcpeislSpec> {
        FgCommandDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn last_fg_command_done(&mut self) -> LastFgCommandDoneW<RfcpeislSpec> {
        LastFgCommandDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_done(&mut self) -> TxDoneW<RfcpeislSpec> {
        TxDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ack(&mut self) -> TxAckW<RfcpeislSpec> {
        TxAckW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl(&mut self) -> TxCtrlW<RfcpeislSpec> {
        TxCtrlW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl_ack(&mut self) -> TxCtrlAckW<RfcpeislSpec> {
        TxCtrlAckW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ctrl_ack_ack(&mut self) -> TxCtrlAckAckW<RfcpeislSpec> {
        TxCtrlAckAckW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_retrans(&mut self) -> TxRetransW<RfcpeislSpec> {
        TxRetransW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_entry_done(&mut self) -> TxEntryDoneW<RfcpeislSpec> {
        TxEntryDoneW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn tx_buffer_changed(&mut self) -> TxBufferChangedW<RfcpeislSpec> {
        TxBufferChangedW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn irq12(&mut self) -> Irq12W<RfcpeislSpec> {
        Irq12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn irq13(&mut self) -> Irq13W<RfcpeislSpec> {
        Irq13W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn irq14(&mut self) -> Irq14W<RfcpeislSpec> {
        Irq14W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn irq15(&mut self) -> Irq15W<RfcpeislSpec> {
        Irq15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ok(&mut self) -> RxOkW<RfcpeislSpec> {
        RxOkW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_nok(&mut self) -> RxNokW<RfcpeislSpec> {
        RxNokW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ignored(&mut self) -> RxIgnoredW<RfcpeislSpec> {
        RxIgnoredW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_empty(&mut self) -> RxEmptyW<RfcpeislSpec> {
        RxEmptyW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ctrl(&mut self) -> RxCtrlW<RfcpeislSpec> {
        RxCtrlW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ctrl_ack(&mut self) -> RxCtrlAckW<RfcpeislSpec> {
        RxCtrlAckW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_buf_full(&mut self) -> RxBufFullW<RfcpeislSpec> {
        RxBufFullW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_entry_done(&mut self) -> RxEntryDoneW<RfcpeislSpec> {
        RxEntryDoneW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_written(&mut self) -> RxDataWrittenW<RfcpeislSpec> {
        RxDataWrittenW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_n_data_written(&mut self) -> RxNDataWrittenW<RfcpeislSpec> {
        RxNDataWrittenW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn rx_aborted(&mut self) -> RxAbortedW<RfcpeislSpec> {
        RxAbortedW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn irq27(&mut self) -> Irq27W<RfcpeislSpec> {
        Irq27W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn synth_no_lock(&mut self) -> SynthNoLockW<RfcpeislSpec> {
        SynthNoLockW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn modules_unlocked(&mut self) -> ModulesUnlockedW<RfcpeislSpec> {
        ModulesUnlockedW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn boot_done(&mut self) -> BootDoneW<RfcpeislSpec> {
        BootDoneW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
    #[inline(always)]
    #[must_use]
    pub fn internal_error(&mut self) -> InternalErrorW<RfcpeislSpec> {
        InternalErrorW::new(self, 31)
    }
}
#[doc = "Interrupt Vector Selection For Command and Packet Engine Generated Interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcpeisl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcpeisl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcpeislSpec;
impl crate::RegisterSpec for RfcpeislSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcpeisl::R`](R) reader structure"]
impl crate::Readable for RfcpeislSpec {}
#[doc = "`write(|w| ..)` method takes [`rfcpeisl::W`](W) writer structure"]
impl crate::Writable for RfcpeislSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFCPEISL to value 0xffff_0000"]
impl crate::Resettable for RfcpeislSpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
