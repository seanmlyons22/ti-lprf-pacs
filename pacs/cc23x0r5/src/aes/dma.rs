#[doc = "Register `DMA` reader"]
pub type R = crate::R<DmaSpec>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DmaSpec>;
#[doc = "2:0\\]
Channel A Trigger Select the condition that triggers DMA channel A request. Non-enumerated values are not supported and ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgcha {
    #[doc = "4: Reads of TXT3 or TXTXBUF3 trigger request."]
    Rdtxt3 = 4,
    #[doc = "3: Writes to TXT3 or TXTX3 trigger request."]
    Wrtxt3 = 3,
    #[doc = "2: Completion of ECB encryption triggers request."]
    Ecbdone = 2,
    #[doc = "1: Start of ECB encryption triggers request."]
    Ecbstart = 1,
    #[doc = "0: DMA requests are disabled."]
    Dis = 0,
}
impl From<Trgcha> for u8 {
    #[inline(always)]
    fn from(variant: Trgcha) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgcha {
    type Ux = u8;
}
impl crate::IsEnum for Trgcha {}
#[doc = "Field `TRGCHA` reader - 2:0\\]
Channel A Trigger Select the condition that triggers DMA channel A request. Non-enumerated values are not supported and ignored."]
pub type TrgchaR = crate::FieldReader<Trgcha>;
impl TrgchaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trgcha> {
        match self.bits {
            4 => Some(Trgcha::Rdtxt3),
            3 => Some(Trgcha::Wrtxt3),
            2 => Some(Trgcha::Ecbdone),
            1 => Some(Trgcha::Ecbstart),
            0 => Some(Trgcha::Dis),
            _ => None,
        }
    }
    #[doc = "Reads of TXT3 or TXTXBUF3 trigger request."]
    #[inline(always)]
    pub fn is_rdtxt3(&self) -> bool {
        *self == Trgcha::Rdtxt3
    }
    #[doc = "Writes to TXT3 or TXTX3 trigger request."]
    #[inline(always)]
    pub fn is_wrtxt3(&self) -> bool {
        *self == Trgcha::Wrtxt3
    }
    #[doc = "Completion of ECB encryption triggers request."]
    #[inline(always)]
    pub fn is_ecbdone(&self) -> bool {
        *self == Trgcha::Ecbdone
    }
    #[doc = "Start of ECB encryption triggers request."]
    #[inline(always)]
    pub fn is_ecbstart(&self) -> bool {
        *self == Trgcha::Ecbstart
    }
    #[doc = "DMA requests are disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Trgcha::Dis
    }
}
#[doc = "Field `TRGCHA` writer - 2:0\\]
Channel A Trigger Select the condition that triggers DMA channel A request. Non-enumerated values are not supported and ignored."]
pub type TrgchaW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trgcha>;
impl<'a, REG> TrgchaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reads of TXT3 or TXTXBUF3 trigger request."]
    #[inline(always)]
    pub fn rdtxt3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgcha::Rdtxt3)
    }
    #[doc = "Writes to TXT3 or TXTX3 trigger request."]
    #[inline(always)]
    pub fn wrtxt3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgcha::Wrtxt3)
    }
    #[doc = "Completion of ECB encryption triggers request."]
    #[inline(always)]
    pub fn ecbdone(self) -> &'a mut crate::W<REG> {
        self.variant(Trgcha::Ecbdone)
    }
    #[doc = "Start of ECB encryption triggers request."]
    #[inline(always)]
    pub fn ecbstart(self) -> &'a mut crate::W<REG> {
        self.variant(Trgcha::Ecbstart)
    }
    #[doc = "DMA requests are disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Trgcha::Dis)
    }
}
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "5:4\\]
Channel A Read Write Address The DMA accesses DMACHA to read or write contents of TXT and BUF as a response to a burst request. This field specifes the start address of the first DMA transfer that follows the burst request. The internal address gets incremented automatically for subsequent accesses. The DMA shall transfer 8-bit, 16-bit, or 32-bit words, and shall always complete a 16-byte transfer before re-arbitration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adrcha {
    #[doc = "3: Start address is TXTXBUF0."]
    Txtxbuf0 = 3,
    #[doc = "2: Start address is BUF0."]
    Buf0 = 2,
    #[doc = "1: Start address is TXTX0."]
    Txtx0 = 1,
    #[doc = "0: Start address is TXT0."]
    Txt0 = 0,
}
impl From<Adrcha> for u8 {
    #[inline(always)]
    fn from(variant: Adrcha) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adrcha {
    type Ux = u8;
}
impl crate::IsEnum for Adrcha {}
#[doc = "Field `ADRCHA` reader - 5:4\\]
Channel A Read Write Address The DMA accesses DMACHA to read or write contents of TXT and BUF as a response to a burst request. This field specifes the start address of the first DMA transfer that follows the burst request. The internal address gets incremented automatically for subsequent accesses. The DMA shall transfer 8-bit, 16-bit, or 32-bit words, and shall always complete a 16-byte transfer before re-arbitration."]
pub type AdrchaR = crate::FieldReader<Adrcha>;
impl AdrchaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adrcha {
        match self.bits {
            3 => Adrcha::Txtxbuf0,
            2 => Adrcha::Buf0,
            1 => Adrcha::Txtx0,
            0 => Adrcha::Txt0,
            _ => unreachable!(),
        }
    }
    #[doc = "Start address is TXTXBUF0."]
    #[inline(always)]
    pub fn is_txtxbuf0(&self) -> bool {
        *self == Adrcha::Txtxbuf0
    }
    #[doc = "Start address is BUF0."]
    #[inline(always)]
    pub fn is_buf0(&self) -> bool {
        *self == Adrcha::Buf0
    }
    #[doc = "Start address is TXTX0."]
    #[inline(always)]
    pub fn is_txtx0(&self) -> bool {
        *self == Adrcha::Txtx0
    }
    #[doc = "Start address is TXT0."]
    #[inline(always)]
    pub fn is_txt0(&self) -> bool {
        *self == Adrcha::Txt0
    }
}
#[doc = "Field `ADRCHA` writer - 5:4\\]
Channel A Read Write Address The DMA accesses DMACHA to read or write contents of TXT and BUF as a response to a burst request. This field specifes the start address of the first DMA transfer that follows the burst request. The internal address gets incremented automatically for subsequent accesses. The DMA shall transfer 8-bit, 16-bit, or 32-bit words, and shall always complete a 16-byte transfer before re-arbitration."]
pub type AdrchaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adrcha, crate::Safe>;
impl<'a, REG> AdrchaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Start address is TXTXBUF0."]
    #[inline(always)]
    pub fn txtxbuf0(self) -> &'a mut crate::W<REG> {
        self.variant(Adrcha::Txtxbuf0)
    }
    #[doc = "Start address is BUF0."]
    #[inline(always)]
    pub fn buf0(self) -> &'a mut crate::W<REG> {
        self.variant(Adrcha::Buf0)
    }
    #[doc = "Start address is TXTX0."]
    #[inline(always)]
    pub fn txtx0(self) -> &'a mut crate::W<REG> {
        self.variant(Adrcha::Txtx0)
    }
    #[doc = "Start address is TXT0."]
    #[inline(always)]
    pub fn txt0(self) -> &'a mut crate::W<REG> {
        self.variant(Adrcha::Txt0)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "10:8\\]
Channel B Trigger Select the condition that triggers DMA channel B request. Non-enumerated values are not supported and ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgchb {
    #[doc = "4: Reads of TXT3, or TXTXBUF3 trigger request."]
    Rdtxt3 = 4,
    #[doc = "3: Writes to TXT3, TXTX3, or TXTXBUF3 trigger request."]
    Wrtxt3 = 3,
    #[doc = "2: Completion of ECB encryption triggers request."]
    Ecbdone = 2,
    #[doc = "1: Start of ECB encryption triggers request."]
    Ecbstart = 1,
    #[doc = "0: DMA requests are disabled."]
    Dis = 0,
}
impl From<Trgchb> for u8 {
    #[inline(always)]
    fn from(variant: Trgchb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgchb {
    type Ux = u8;
}
impl crate::IsEnum for Trgchb {}
#[doc = "Field `TRGCHB` reader - 10:8\\]
Channel B Trigger Select the condition that triggers DMA channel B request. Non-enumerated values are not supported and ignored."]
pub type TrgchbR = crate::FieldReader<Trgchb>;
impl TrgchbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trgchb> {
        match self.bits {
            4 => Some(Trgchb::Rdtxt3),
            3 => Some(Trgchb::Wrtxt3),
            2 => Some(Trgchb::Ecbdone),
            1 => Some(Trgchb::Ecbstart),
            0 => Some(Trgchb::Dis),
            _ => None,
        }
    }
    #[doc = "Reads of TXT3, or TXTXBUF3 trigger request."]
    #[inline(always)]
    pub fn is_rdtxt3(&self) -> bool {
        *self == Trgchb::Rdtxt3
    }
    #[doc = "Writes to TXT3, TXTX3, or TXTXBUF3 trigger request."]
    #[inline(always)]
    pub fn is_wrtxt3(&self) -> bool {
        *self == Trgchb::Wrtxt3
    }
    #[doc = "Completion of ECB encryption triggers request."]
    #[inline(always)]
    pub fn is_ecbdone(&self) -> bool {
        *self == Trgchb::Ecbdone
    }
    #[doc = "Start of ECB encryption triggers request."]
    #[inline(always)]
    pub fn is_ecbstart(&self) -> bool {
        *self == Trgchb::Ecbstart
    }
    #[doc = "DMA requests are disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Trgchb::Dis
    }
}
#[doc = "Field `TRGCHB` writer - 10:8\\]
Channel B Trigger Select the condition that triggers DMA channel B request. Non-enumerated values are not supported and ignored."]
pub type TrgchbW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trgchb>;
impl<'a, REG> TrgchbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reads of TXT3, or TXTXBUF3 trigger request."]
    #[inline(always)]
    pub fn rdtxt3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgchb::Rdtxt3)
    }
    #[doc = "Writes to TXT3, TXTX3, or TXTXBUF3 trigger request."]
    #[inline(always)]
    pub fn wrtxt3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgchb::Wrtxt3)
    }
    #[doc = "Completion of ECB encryption triggers request."]
    #[inline(always)]
    pub fn ecbdone(self) -> &'a mut crate::W<REG> {
        self.variant(Trgchb::Ecbdone)
    }
    #[doc = "Start of ECB encryption triggers request."]
    #[inline(always)]
    pub fn ecbstart(self) -> &'a mut crate::W<REG> {
        self.variant(Trgchb::Ecbstart)
    }
    #[doc = "DMA requests are disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Trgchb::Dis)
    }
}
#[doc = "Field `RESERVED11` reader - 11:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::BitReader;
#[doc = "Field `RESERVED11` writer - 11:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "13:12\\]
Channel B Read Write Address The DMA accesses DMACHB to read or write contents of TXT and BUF as a response to a burst request. This field specifes the start address of the first DMA transfer that follows the burst request. The internal address gets incremented automatically for subsequent accesses. The DMA shall transfer 8-bit, 16-bit, or 32-bit words, and shall always complete a 16-byte transfer before re-arbitration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adrchb {
    #[doc = "3: Start address is TXTXBUF0."]
    Txtxbuf0 = 3,
    #[doc = "2: Start address is BUF0."]
    Buf0 = 2,
    #[doc = "1: Start address is TXTX0."]
    Txtx0 = 1,
    #[doc = "0: Start address is TXT0."]
    Txt0 = 0,
}
impl From<Adrchb> for u8 {
    #[inline(always)]
    fn from(variant: Adrchb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adrchb {
    type Ux = u8;
}
impl crate::IsEnum for Adrchb {}
#[doc = "Field `ADRCHB` reader - 13:12\\]
Channel B Read Write Address The DMA accesses DMACHB to read or write contents of TXT and BUF as a response to a burst request. This field specifes the start address of the first DMA transfer that follows the burst request. The internal address gets incremented automatically for subsequent accesses. The DMA shall transfer 8-bit, 16-bit, or 32-bit words, and shall always complete a 16-byte transfer before re-arbitration."]
pub type AdrchbR = crate::FieldReader<Adrchb>;
impl AdrchbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adrchb {
        match self.bits {
            3 => Adrchb::Txtxbuf0,
            2 => Adrchb::Buf0,
            1 => Adrchb::Txtx0,
            0 => Adrchb::Txt0,
            _ => unreachable!(),
        }
    }
    #[doc = "Start address is TXTXBUF0."]
    #[inline(always)]
    pub fn is_txtxbuf0(&self) -> bool {
        *self == Adrchb::Txtxbuf0
    }
    #[doc = "Start address is BUF0."]
    #[inline(always)]
    pub fn is_buf0(&self) -> bool {
        *self == Adrchb::Buf0
    }
    #[doc = "Start address is TXTX0."]
    #[inline(always)]
    pub fn is_txtx0(&self) -> bool {
        *self == Adrchb::Txtx0
    }
    #[doc = "Start address is TXT0."]
    #[inline(always)]
    pub fn is_txt0(&self) -> bool {
        *self == Adrchb::Txt0
    }
}
#[doc = "Field `ADRCHB` writer - 13:12\\]
Channel B Read Write Address The DMA accesses DMACHB to read or write contents of TXT and BUF as a response to a burst request. This field specifes the start address of the first DMA transfer that follows the burst request. The internal address gets incremented automatically for subsequent accesses. The DMA shall transfer 8-bit, 16-bit, or 32-bit words, and shall always complete a 16-byte transfer before re-arbitration."]
pub type AdrchbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adrchb, crate::Safe>;
impl<'a, REG> AdrchbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Start address is TXTXBUF0."]
    #[inline(always)]
    pub fn txtxbuf0(self) -> &'a mut crate::W<REG> {
        self.variant(Adrchb::Txtxbuf0)
    }
    #[doc = "Start address is BUF0."]
    #[inline(always)]
    pub fn buf0(self) -> &'a mut crate::W<REG> {
        self.variant(Adrchb::Buf0)
    }
    #[doc = "Start address is TXTX0."]
    #[inline(always)]
    pub fn txtx0(self) -> &'a mut crate::W<REG> {
        self.variant(Adrchb::Txtx0)
    }
    #[doc = "Start address is TXT0."]
    #[inline(always)]
    pub fn txt0(self) -> &'a mut crate::W<REG> {
        self.variant(Adrchb::Txt0)
    }
}
#[doc = "Field `RESERVED14` reader - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader;
#[doc = "Field `RESERVED14` writer - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "19:16\\]
Done Action This field determines the side effects of DMA done. It is allowed to configure this field with an OR-combination of supported enums, with the exception that GATE_TRGECB_ON_CHA and GATE_TRGECB_ON_CHA_DEL shall be mutually exclusive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Doneact {
    #[doc = "8: DMA channel B done event clears TXT0 thru TXT3 if STA.STATE = IDLE. Event is ignored if condition is not met."]
    ClrTxtOnChb = 8,
    #[doc = "4: DMA channel A done event clears TXT0 thru TXT3 if STA.STATE = IDLE. Event is ignored if condition is not met."]
    ClrTxtOnCha = 4,
    #[doc = "2: Due to the pipelining of BUF writes, in certain modes, DMA CHA Done appears before the last but one ECB has completed. Setting this bit, will gate the triggers defined in AUTOCFG.TRGECB only after the last write by CHA is consumed by AES FSM. Used in ECB,CBC,CBC-MAC modes (having multiple blocks encryption) to avoid spurious ECB triggered on last read by CHB. For single mode encryption, DMA.GATE_TRGECB_ON_CHA should be used."]
    GateTrgecbOnChaDel = 2,
    #[doc = "1: Triggers defined in AUTOCFG.TRGECB are gated when RIS.CHADONE = SET."]
    GateTrgecbOnCha = 1,
    #[doc = "0: DMA done has no side effect."]
    Dis = 0,
}
impl From<Doneact> for u8 {
    #[inline(always)]
    fn from(variant: Doneact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Doneact {
    type Ux = u8;
}
impl crate::IsEnum for Doneact {}
#[doc = "Field `DONEACT` reader - 19:16\\]
Done Action This field determines the side effects of DMA done. It is allowed to configure this field with an OR-combination of supported enums, with the exception that GATE_TRGECB_ON_CHA and GATE_TRGECB_ON_CHA_DEL shall be mutually exclusive"]
pub type DoneactR = crate::FieldReader<Doneact>;
impl DoneactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Doneact> {
        match self.bits {
            8 => Some(Doneact::ClrTxtOnChb),
            4 => Some(Doneact::ClrTxtOnCha),
            2 => Some(Doneact::GateTrgecbOnChaDel),
            1 => Some(Doneact::GateTrgecbOnCha),
            0 => Some(Doneact::Dis),
            _ => None,
        }
    }
    #[doc = "DMA channel B done event clears TXT0 thru TXT3 if STA.STATE = IDLE. Event is ignored if condition is not met."]
    #[inline(always)]
    pub fn is_clr_txt_on_chb(&self) -> bool {
        *self == Doneact::ClrTxtOnChb
    }
    #[doc = "DMA channel A done event clears TXT0 thru TXT3 if STA.STATE = IDLE. Event is ignored if condition is not met."]
    #[inline(always)]
    pub fn is_clr_txt_on_cha(&self) -> bool {
        *self == Doneact::ClrTxtOnCha
    }
    #[doc = "Due to the pipelining of BUF writes, in certain modes, DMA CHA Done appears before the last but one ECB has completed. Setting this bit, will gate the triggers defined in AUTOCFG.TRGECB only after the last write by CHA is consumed by AES FSM. Used in ECB,CBC,CBC-MAC modes (having multiple blocks encryption) to avoid spurious ECB triggered on last read by CHB. For single mode encryption, DMA.GATE_TRGECB_ON_CHA should be used."]
    #[inline(always)]
    pub fn is_gate_trgecb_on_cha_del(&self) -> bool {
        *self == Doneact::GateTrgecbOnChaDel
    }
    #[doc = "Triggers defined in AUTOCFG.TRGECB are gated when RIS.CHADONE = SET."]
    #[inline(always)]
    pub fn is_gate_trgecb_on_cha(&self) -> bool {
        *self == Doneact::GateTrgecbOnCha
    }
    #[doc = "DMA done has no side effect."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Doneact::Dis
    }
}
#[doc = "Field `DONEACT` writer - 19:16\\]
Done Action This field determines the side effects of DMA done. It is allowed to configure this field with an OR-combination of supported enums, with the exception that GATE_TRGECB_ON_CHA and GATE_TRGECB_ON_CHA_DEL shall be mutually exclusive"]
pub type DoneactW<'a, REG> = crate::FieldWriter<'a, REG, 4, Doneact>;
impl<'a, REG> DoneactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA channel B done event clears TXT0 thru TXT3 if STA.STATE = IDLE. Event is ignored if condition is not met."]
    #[inline(always)]
    pub fn clr_txt_on_chb(self) -> &'a mut crate::W<REG> {
        self.variant(Doneact::ClrTxtOnChb)
    }
    #[doc = "DMA channel A done event clears TXT0 thru TXT3 if STA.STATE = IDLE. Event is ignored if condition is not met."]
    #[inline(always)]
    pub fn clr_txt_on_cha(self) -> &'a mut crate::W<REG> {
        self.variant(Doneact::ClrTxtOnCha)
    }
    #[doc = "Due to the pipelining of BUF writes, in certain modes, DMA CHA Done appears before the last but one ECB has completed. Setting this bit, will gate the triggers defined in AUTOCFG.TRGECB only after the last write by CHA is consumed by AES FSM. Used in ECB,CBC,CBC-MAC modes (having multiple blocks encryption) to avoid spurious ECB triggered on last read by CHB. For single mode encryption, DMA.GATE_TRGECB_ON_CHA should be used."]
    #[inline(always)]
    pub fn gate_trgecb_on_cha_del(self) -> &'a mut crate::W<REG> {
        self.variant(Doneact::GateTrgecbOnChaDel)
    }
    #[doc = "Triggers defined in AUTOCFG.TRGECB are gated when RIS.CHADONE = SET."]
    #[inline(always)]
    pub fn gate_trgecb_on_cha(self) -> &'a mut crate::W<REG> {
        self.variant(Doneact::GateTrgecbOnCha)
    }
    #[doc = "DMA done has no side effect."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Doneact::Dis)
    }
}
#[doc = "Field `RESERVED19` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED19` writer - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Channel A Trigger Select the condition that triggers DMA channel A request. Non-enumerated values are not supported and ignored."]
    #[inline(always)]
    pub fn trgcha(&self) -> TrgchaR {
        TrgchaR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Channel A Read Write Address The DMA accesses DMACHA to read or write contents of TXT and BUF as a response to a burst request. This field specifes the start address of the first DMA transfer that follows the burst request. The internal address gets incremented automatically for subsequent accesses. The DMA shall transfer 8-bit, 16-bit, or 32-bit words, and shall always complete a 16-byte transfer before re-arbitration."]
    #[inline(always)]
    pub fn adrcha(&self) -> AdrchaR {
        AdrchaR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Channel B Trigger Select the condition that triggers DMA channel B request. Non-enumerated values are not supported and ignored."]
    #[inline(always)]
    pub fn trgchb(&self) -> TrgchbR {
        TrgchbR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Channel B Read Write Address The DMA accesses DMACHB to read or write contents of TXT and BUF as a response to a burst request. This field specifes the start address of the first DMA transfer that follows the burst request. The internal address gets incremented automatically for subsequent accesses. The DMA shall transfer 8-bit, 16-bit, or 32-bit words, and shall always complete a 16-byte transfer before re-arbitration."]
    #[inline(always)]
    pub fn adrchb(&self) -> AdrchbR {
        AdrchbR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Done Action This field determines the side effects of DMA done. It is allowed to configure this field with an OR-combination of supported enums, with the exception that GATE_TRGECB_ON_CHA and GATE_TRGECB_ON_CHA_DEL shall be mutually exclusive"]
    #[inline(always)]
    pub fn doneact(&self) -> DoneactR {
        DoneactR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Channel A Trigger Select the condition that triggers DMA channel A request. Non-enumerated values are not supported and ignored."]
    #[inline(always)]
    #[must_use]
    pub fn trgcha(&mut self) -> TrgchaW<DmaSpec> {
        TrgchaW::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<DmaSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Channel A Read Write Address The DMA accesses DMACHA to read or write contents of TXT and BUF as a response to a burst request. This field specifes the start address of the first DMA transfer that follows the burst request. The internal address gets incremented automatically for subsequent accesses. The DMA shall transfer 8-bit, 16-bit, or 32-bit words, and shall always complete a 16-byte transfer before re-arbitration."]
    #[inline(always)]
    #[must_use]
    pub fn adrcha(&mut self) -> AdrchaW<DmaSpec> {
        AdrchaW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<DmaSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Channel B Trigger Select the condition that triggers DMA channel B request. Non-enumerated values are not supported and ignored."]
    #[inline(always)]
    #[must_use]
    pub fn trgchb(&mut self) -> TrgchbW<DmaSpec> {
        TrgchbW::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<DmaSpec> {
        Reserved11W::new(self, 11)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Channel B Read Write Address The DMA accesses DMACHB to read or write contents of TXT and BUF as a response to a burst request. This field specifes the start address of the first DMA transfer that follows the burst request. The internal address gets incremented automatically for subsequent accesses. The DMA shall transfer 8-bit, 16-bit, or 32-bit words, and shall always complete a 16-byte transfer before re-arbitration."]
    #[inline(always)]
    #[must_use]
    pub fn adrchb(&mut self) -> AdrchbW<DmaSpec> {
        AdrchbW::new(self, 12)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<DmaSpec> {
        Reserved14W::new(self, 14)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Done Action This field determines the side effects of DMA done. It is allowed to configure this field with an OR-combination of supported enums, with the exception that GATE_TRGECB_ON_CHA and GATE_TRGECB_ON_CHA_DEL shall be mutually exclusive"]
    #[inline(always)]
    #[must_use]
    pub fn doneact(&mut self) -> DoneactW<DmaSpec> {
        DoneactW::new(self, 16)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> Reserved19W<DmaSpec> {
        Reserved19W::new(self, 20)
    }
}
#[doc = "Direct Memory Access This register controls the conditions that will generate burst requests on each DMA channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSpec;
impl crate::RegisterSpec for DmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DmaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DmaSpec {
    const RESET_VALUE: u32 = 0;
}
