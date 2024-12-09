#[doc = "Register `TXFHDRC` reader"]
pub type R = crate::R<TxfhdrcSpec>;
#[doc = "Register `TXFHDRC` writer"]
pub type W = crate::W<TxfhdrcSpec>;
#[doc = "0:0\\]
Header enable field. When CS_GATE is set to BLK, this bit has to be set by software to enable this feature. When CS_GATE is set to UNBLK, this field is set automatically whenever a write to header update registers occurs MMR_TXFHDRn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdrEn {
    #[doc = "1: Atomic header udpate feature enable"]
    Ena = 1,
    #[doc = "0: Atomic header update feature disable"]
    Dis = 0,
}
impl From<HdrEn> for bool {
    #[inline(always)]
    fn from(variant: HdrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDR_EN` reader - 0:0\\]
Header enable field. When CS_GATE is set to BLK, this bit has to be set by software to enable this feature. When CS_GATE is set to UNBLK, this field is set automatically whenever a write to header update registers occurs MMR_TXFHDRn"]
pub type HdrEnR = crate::BitReader<HdrEn>;
impl HdrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdrEn {
        match self.bits {
            true => HdrEn::Ena,
            false => HdrEn::Dis,
        }
    }
    #[doc = "Atomic header udpate feature enable"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == HdrEn::Ena
    }
    #[doc = "Atomic header update feature disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == HdrEn::Dis
    }
}
#[doc = "Field `HDR_EN` writer - 0:0\\]
Header enable field. When CS_GATE is set to BLK, this bit has to be set by software to enable this feature. When CS_GATE is set to UNBLK, this field is set automatically whenever a write to header update registers occurs MMR_TXFHDRn"]
pub type HdrEnW<'a, REG> = crate::BitWriter<'a, REG, HdrEn>;
impl<'a, REG> HdrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Atomic header udpate feature enable"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(HdrEn::Ena)
    }
    #[doc = "Atomic header update feature disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(HdrEn::Dis)
    }
}
#[doc = "1:1\\]
Header Ignored field. When CS_GATE is set to BLK, this bit is set when the last Header update register(MMR_TXFHDRn.*) is written when CS is low or HDR_CMT is already set. When CS_GATE is set to UNBLK, this bit is set only when the header update register is written when HDR_CMT is already set. This bit remains 0 otherwise. When set, this bit can be written to a value of 0 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdrIgn {
    #[doc = "1: Header update is not ignored"]
    Clr = 1,
}
impl From<HdrIgn> for bool {
    #[inline(always)]
    fn from(variant: HdrIgn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDR_IGN` reader - 1:1\\]
Header Ignored field. When CS_GATE is set to BLK, this bit is set when the last Header update register(MMR_TXFHDRn.*) is written when CS is low or HDR_CMT is already set. When CS_GATE is set to UNBLK, this bit is set only when the header update register is written when HDR_CMT is already set. This bit remains 0 otherwise. When set, this bit can be written to a value of 0 to clear."]
pub type HdrIgnR = crate::BitReader<HdrIgn>;
impl HdrIgnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HdrIgn> {
        match self.bits {
            true => Some(HdrIgn::Clr),
            _ => None,
        }
    }
    #[doc = "Header update is not ignored"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == HdrIgn::Clr
    }
}
#[doc = "Field `HDR_IGN` writer - 1:1\\]
Header Ignored field. When CS_GATE is set to BLK, this bit is set when the last Header update register(MMR_TXFHDRn.*) is written when CS is low or HDR_CMT is already set. When CS_GATE is set to UNBLK, this bit is set only when the header update register is written when HDR_CMT is already set. This bit remains 0 otherwise. When set, this bit can be written to a value of 0 to clear."]
pub type HdrIgnW<'a, REG> = crate::BitWriter<'a, REG, HdrIgn>;
impl<'a, REG> HdrIgnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Header update is not ignored"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(HdrIgn::Clr)
    }
}
#[doc = "2:2\\]
Header Committed field. This bit is set when the HDR_EN bit is set and CS is sampled low. This bit remains 0 otherwise. When set, this bit can be written to a value of 0 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdrCmt {
    #[doc = "1: Header update is committed"]
    Set = 1,
    #[doc = "0: Header update is not committed"]
    Clr = 0,
}
impl From<HdrCmt> for bool {
    #[inline(always)]
    fn from(variant: HdrCmt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDR_CMT` reader - 2:2\\]
Header Committed field. This bit is set when the HDR_EN bit is set and CS is sampled low. This bit remains 0 otherwise. When set, this bit can be written to a value of 0 to clear."]
pub type HdrCmtR = crate::BitReader<HdrCmt>;
impl HdrCmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdrCmt {
        match self.bits {
            true => HdrCmt::Set,
            false => HdrCmt::Clr,
        }
    }
    #[doc = "Header update is committed"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == HdrCmt::Set
    }
    #[doc = "Header update is not committed"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == HdrCmt::Clr
    }
}
#[doc = "Field `HDR_CMT` writer - 2:2\\]
Header Committed field. This bit is set when the HDR_EN bit is set and CS is sampled low. This bit remains 0 otherwise. When set, this bit can be written to a value of 0 to clear."]
pub type HdrCmtW<'a, REG> = crate::BitWriter<'a, REG, HdrCmt>;
impl<'a, REG> HdrCmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Header update is committed"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(HdrCmt::Set)
    }
    #[doc = "Header update is not committed"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(HdrCmt::Clr)
    }
}
#[doc = "3:3\\]
Chip Select Gating control register. If this bit is set header update register writes are blocked when chip select (CS) is active low, and HDR_IGN bit is set This bit resets to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsGate {
    #[doc = "1: Header update register writes are blocked when CS is active (low)"]
    Blk = 1,
    #[doc = "0: Header update register writes are not blocked based on CS status"]
    Unblk = 0,
}
impl From<CsGate> for bool {
    #[inline(always)]
    fn from(variant: CsGate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS_GATE` reader - 3:3\\]
Chip Select Gating control register. If this bit is set header update register writes are blocked when chip select (CS) is active low, and HDR_IGN bit is set This bit resets to 0."]
pub type CsGateR = crate::BitReader<CsGate>;
impl CsGateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CsGate {
        match self.bits {
            true => CsGate::Blk,
            false => CsGate::Unblk,
        }
    }
    #[doc = "Header update register writes are blocked when CS is active (low)"]
    #[inline(always)]
    pub fn is_blk(&self) -> bool {
        *self == CsGate::Blk
    }
    #[doc = "Header update register writes are not blocked based on CS status"]
    #[inline(always)]
    pub fn is_unblk(&self) -> bool {
        *self == CsGate::Unblk
    }
}
#[doc = "Field `CS_GATE` writer - 3:3\\]
Chip Select Gating control register. If this bit is set header update register writes are blocked when chip select (CS) is active low, and HDR_IGN bit is set This bit resets to 0."]
pub type CsGateW<'a, REG> = crate::BitWriter<'a, REG, CsGate>;
impl<'a, REG> CsGateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Header update register writes are blocked when CS is active (low)"]
    #[inline(always)]
    pub fn blk(self) -> &'a mut crate::W<REG> {
        self.variant(CsGate::Blk)
    }
    #[doc = "Header update register writes are not blocked based on CS status"]
    #[inline(always)]
    pub fn unblk(self) -> &'a mut crate::W<REG> {
        self.variant(CsGate::Unblk)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Header enable field. When CS_GATE is set to BLK, this bit has to be set by software to enable this feature. When CS_GATE is set to UNBLK, this field is set automatically whenever a write to header update registers occurs MMR_TXFHDRn"]
    #[inline(always)]
    pub fn hdr_en(&self) -> HdrEnR {
        HdrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Header Ignored field. When CS_GATE is set to BLK, this bit is set when the last Header update register(MMR_TXFHDRn.*) is written when CS is low or HDR_CMT is already set. When CS_GATE is set to UNBLK, this bit is set only when the header update register is written when HDR_CMT is already set. This bit remains 0 otherwise. When set, this bit can be written to a value of 0 to clear."]
    #[inline(always)]
    pub fn hdr_ign(&self) -> HdrIgnR {
        HdrIgnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Header Committed field. This bit is set when the HDR_EN bit is set and CS is sampled low. This bit remains 0 otherwise. When set, this bit can be written to a value of 0 to clear."]
    #[inline(always)]
    pub fn hdr_cmt(&self) -> HdrCmtR {
        HdrCmtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Chip Select Gating control register. If this bit is set header update register writes are blocked when chip select (CS) is active low, and HDR_IGN bit is set This bit resets to 0."]
    #[inline(always)]
    pub fn cs_gate(&self) -> CsGateR {
        CsGateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Header enable field. When CS_GATE is set to BLK, this bit has to be set by software to enable this feature. When CS_GATE is set to UNBLK, this field is set automatically whenever a write to header update registers occurs MMR_TXFHDRn"]
    #[inline(always)]
    #[must_use]
    pub fn hdr_en(&mut self) -> HdrEnW<TxfhdrcSpec> {
        HdrEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Header Ignored field. When CS_GATE is set to BLK, this bit is set when the last Header update register(MMR_TXFHDRn.*) is written when CS is low or HDR_CMT is already set. When CS_GATE is set to UNBLK, this bit is set only when the header update register is written when HDR_CMT is already set. This bit remains 0 otherwise. When set, this bit can be written to a value of 0 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn hdr_ign(&mut self) -> HdrIgnW<TxfhdrcSpec> {
        HdrIgnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Header Committed field. This bit is set when the HDR_EN bit is set and CS is sampled low. This bit remains 0 otherwise. When set, this bit can be written to a value of 0 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn hdr_cmt(&mut self) -> HdrCmtW<TxfhdrcSpec> {
        HdrCmtW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Chip Select Gating control register. If this bit is set header update register writes are blocked when chip select (CS) is active low, and HDR_IGN bit is set This bit resets to 0."]
    #[inline(always)]
    #[must_use]
    pub fn cs_gate(&mut self) -> CsGateW<TxfhdrcSpec> {
        CsGateW::new(self, 3)
    }
}
#[doc = "Atomic Header Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfhdrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfhdrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfhdrcSpec;
impl crate::RegisterSpec for TxfhdrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfhdrc::R`](R) reader structure"]
impl crate::Readable for TxfhdrcSpec {}
#[doc = "`write(|w| ..)` method takes [`txfhdrc::W`](W) writer structure"]
impl crate::Writable for TxfhdrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFHDRC to value 0"]
impl crate::Resettable for TxfhdrcSpec {
    const RESET_VALUE: u32 = 0;
}
