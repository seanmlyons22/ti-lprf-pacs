#[doc = "Register `AIFFMTCFG` reader"]
pub type R = crate::R<AiffmtcfgSpec>;
#[doc = "Register `AIFFMTCFG` writer"]
pub type W = crate::W<AiffmtcfgSpec>;
#[doc = "Field `WORD_LEN` reader - 4:0\\]
Number of bits per word (8-24): In single-phase format, this is the exact number of bits per word. In dual-phase format, this is the maximum number of bits per word. Values below 8 and above 24 give undefined behavior. Data written to memory is always aligned to 16 or 24 bits as defined by MEM_LEN_24. Bit widths that differ from this alignment will either be truncated or zero padded."]
pub type WordLenR = crate::FieldReader;
#[doc = "Field `WORD_LEN` writer - 4:0\\]
Number of bits per word (8-24): In single-phase format, this is the exact number of bits per word. In dual-phase format, this is the maximum number of bits per word. Values below 8 and above 24 give undefined behavior. Data written to memory is always aligned to 16 or 24 bits as defined by MEM_LEN_24. Bit widths that differ from this alignment will either be truncated or zero padded."]
pub type WordLenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DUAL_PHASE` reader - 5:5\\]
Selects dual- or single-phase format. 0: Single-phase: DSP format 1: Dual-phase: I2S, LJF and RJF formats"]
pub type DualPhaseR = crate::BitReader;
#[doc = "Field `DUAL_PHASE` writer - 5:5\\]
Selects dual- or single-phase format. 0: Single-phase: DSP format 1: Dual-phase: I2S, LJF and RJF formats"]
pub type DualPhaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "6:6\\]
On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmplEdge {
    #[doc = "1: Data is sampled on the positive edge and clocked out on the negative edge."]
    Pos = 1,
    #[doc = "0: Data is sampled on the negative edge and clocked out on the positive edge."]
    Neg = 0,
}
impl From<SmplEdge> for bool {
    #[inline(always)]
    fn from(variant: SmplEdge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPL_EDGE` reader - 6:6\\]
On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK."]
pub type SmplEdgeR = crate::BitReader<SmplEdge>;
impl SmplEdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmplEdge {
        match self.bits {
            true => SmplEdge::Pos,
            false => SmplEdge::Neg,
        }
    }
    #[doc = "Data is sampled on the positive edge and clocked out on the negative edge."]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        *self == SmplEdge::Pos
    }
    #[doc = "Data is sampled on the negative edge and clocked out on the positive edge."]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        *self == SmplEdge::Neg
    }
}
#[doc = "Field `SMPL_EDGE` writer - 6:6\\]
On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK."]
pub type SmplEdgeW<'a, REG> = crate::BitWriter<'a, REG, SmplEdge>;
impl<'a, REG> SmplEdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is sampled on the positive edge and clocked out on the negative edge."]
    #[inline(always)]
    pub fn pos(self) -> &'a mut crate::W<REG> {
        self.variant(SmplEdge::Pos)
    }
    #[doc = "Data is sampled on the negative edge and clocked out on the positive edge."]
    #[inline(always)]
    pub fn neg(self) -> &'a mut crate::W<REG> {
        self.variant(SmplEdge::Neg)
    }
}
#[doc = "7:7\\]
The size of each word stored to or loaded from memory:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemLen24 {
    #[doc = "1: 24-bit (one 8 bit and one 16 bit locked access per sample)"]
    _24bit = 1,
    #[doc = "0: 16-bit (one 16 bit access per sample)"]
    _16bit = 0,
}
impl From<MemLen24> for bool {
    #[inline(always)]
    fn from(variant: MemLen24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM_LEN_24` reader - 7:7\\]
The size of each word stored to or loaded from memory:"]
pub type MemLen24R = crate::BitReader<MemLen24>;
impl MemLen24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MemLen24 {
        match self.bits {
            true => MemLen24::_24bit,
            false => MemLen24::_16bit,
        }
    }
    #[doc = "24-bit (one 8 bit and one 16 bit locked access per sample)"]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        *self == MemLen24::_24bit
    }
    #[doc = "16-bit (one 16 bit access per sample)"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == MemLen24::_16bit
    }
}
#[doc = "Field `MEM_LEN_24` writer - 7:7\\]
The size of each word stored to or loaded from memory:"]
pub type MemLen24W<'a, REG> = crate::BitWriter<'a, REG, MemLen24>;
impl<'a, REG> MemLen24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "24-bit (one 8 bit and one 16 bit locked access per sample)"]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut crate::W<REG> {
        self.variant(MemLen24::_24bit)
    }
    #[doc = "16-bit (one 16 bit access per sample)"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(MemLen24::_16bit)
    }
}
#[doc = "Field `DATA_DELAY` reader - 15:8\\]
The number of BCLK periods between a WCLK edge and MSB of the first word in a phase: 0x00: LJF and DSP format 0x01: I2S and DSP format 0x02: RJF format ... 0xFF: RJF format Note: When 0, MSB of the next word will be output in the idle period between LSB of the previous word and the start of the next word. Otherwise logical 0 will be output until the data delay has expired."]
pub type DataDelayR = crate::FieldReader;
#[doc = "Field `DATA_DELAY` writer - 15:8\\]
The number of BCLK periods between a WCLK edge and MSB of the first word in a phase: 0x00: LJF and DSP format 0x01: I2S and DSP format 0x02: RJF format ... 0xFF: RJF format Note: When 0, MSB of the next word will be output in the idle period between LSB of the previous word and the start of the next word. Otherwise logical 0 will be output until the data delay has expired."]
pub type DataDelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Number of bits per word (8-24): In single-phase format, this is the exact number of bits per word. In dual-phase format, this is the maximum number of bits per word. Values below 8 and above 24 give undefined behavior. Data written to memory is always aligned to 16 or 24 bits as defined by MEM_LEN_24. Bit widths that differ from this alignment will either be truncated or zero padded."]
    #[inline(always)]
    pub fn word_len(&self) -> WordLenR {
        WordLenR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Selects dual- or single-phase format. 0: Single-phase: DSP format 1: Dual-phase: I2S, LJF and RJF formats"]
    #[inline(always)]
    pub fn dual_phase(&self) -> DualPhaseR {
        DualPhaseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK."]
    #[inline(always)]
    pub fn smpl_edge(&self) -> SmplEdgeR {
        SmplEdgeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
The size of each word stored to or loaded from memory:"]
    #[inline(always)]
    pub fn mem_len_24(&self) -> MemLen24R {
        MemLen24R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The number of BCLK periods between a WCLK edge and MSB of the first word in a phase: 0x00: LJF and DSP format 0x01: I2S and DSP format 0x02: RJF format ... 0xFF: RJF format Note: When 0, MSB of the next word will be output in the idle period between LSB of the previous word and the start of the next word. Otherwise logical 0 will be output until the data delay has expired."]
    #[inline(always)]
    pub fn data_delay(&self) -> DataDelayR {
        DataDelayR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Number of bits per word (8-24): In single-phase format, this is the exact number of bits per word. In dual-phase format, this is the maximum number of bits per word. Values below 8 and above 24 give undefined behavior. Data written to memory is always aligned to 16 or 24 bits as defined by MEM_LEN_24. Bit widths that differ from this alignment will either be truncated or zero padded."]
    #[inline(always)]
    #[must_use]
    pub fn word_len(&mut self) -> WordLenW<AiffmtcfgSpec> {
        WordLenW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Selects dual- or single-phase format. 0: Single-phase: DSP format 1: Dual-phase: I2S, LJF and RJF formats"]
    #[inline(always)]
    #[must_use]
    pub fn dual_phase(&mut self) -> DualPhaseW<AiffmtcfgSpec> {
        DualPhaseW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK."]
    #[inline(always)]
    #[must_use]
    pub fn smpl_edge(&mut self) -> SmplEdgeW<AiffmtcfgSpec> {
        SmplEdgeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
The size of each word stored to or loaded from memory:"]
    #[inline(always)]
    #[must_use]
    pub fn mem_len_24(&mut self) -> MemLen24W<AiffmtcfgSpec> {
        MemLen24W::new(self, 7)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The number of BCLK periods between a WCLK edge and MSB of the first word in a phase: 0x00: LJF and DSP format 0x01: I2S and DSP format 0x02: RJF format ... 0xFF: RJF format Note: When 0, MSB of the next word will be output in the idle period between LSB of the previous word and the start of the next word. Otherwise logical 0 will be output until the data delay has expired."]
    #[inline(always)]
    #[must_use]
    pub fn data_delay(&mut self) -> DataDelayW<AiffmtcfgSpec> {
        DataDelayW::new(self, 8)
    }
}
#[doc = "Serial Interface Format Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aiffmtcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aiffmtcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AiffmtcfgSpec;
impl crate::RegisterSpec for AiffmtcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aiffmtcfg::R`](R) reader structure"]
impl crate::Readable for AiffmtcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`aiffmtcfg::W`](W) writer structure"]
impl crate::Writable for AiffmtcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIFFMTCFG to value 0x0170"]
impl crate::Resettable for AiffmtcfgSpec {
    const RESET_VALUE: u32 = 0x0170;
}
