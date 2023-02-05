#[doc = "Register `DACCTL` reader"]
pub struct R(crate::R<DACCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACCTL` writer"]
pub struct W(crate::W<DACCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACCTL_SPEC>;
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
impl From<crate::W<DACCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_VOUT_SEL` reader - 2:0\\]
DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\]."]
pub type DAC_VOUT_SEL_R = crate::FieldReader<u8, DAC_VOUT_SEL_A>;
#[doc = "2:0\\]
DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC_VOUT_SEL_A {
    #[doc = "4: Connect to COMPA_IN analog node. Required setting to drive external load selected in ADI_4_AUX:MUX1.COMPA_IN."]
    COMPA_IN = 4,
    #[doc = "2: Connect to COMPA_REF analog node. It is not possible to drive external loads connected to COMPA_REF I/O mux with this setting."]
    COMPA_REF = 2,
    #[doc = "1: Connect to COMPB_REF analog node. Required setting to use Comparator B."]
    COMPB_REF = 1,
    #[doc = "0: Connect to nothing It is recommended to use NC as intermediate step when you change DAC_VOUT_SEL."]
    NC = 0,
}
impl From<DAC_VOUT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_VOUT_SEL_A) -> Self {
        variant as _
    }
}
impl DAC_VOUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAC_VOUT_SEL_A> {
        match self.bits {
            4 => Some(DAC_VOUT_SEL_A::COMPA_IN),
            2 => Some(DAC_VOUT_SEL_A::COMPA_REF),
            1 => Some(DAC_VOUT_SEL_A::COMPB_REF),
            0 => Some(DAC_VOUT_SEL_A::NC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COMPA_IN`"]
    #[inline(always)]
    pub fn is_compa_in(&self) -> bool {
        *self == DAC_VOUT_SEL_A::COMPA_IN
    }
    #[doc = "Checks if the value of the field is `COMPA_REF`"]
    #[inline(always)]
    pub fn is_compa_ref(&self) -> bool {
        *self == DAC_VOUT_SEL_A::COMPA_REF
    }
    #[doc = "Checks if the value of the field is `COMPB_REF`"]
    #[inline(always)]
    pub fn is_compb_ref(&self) -> bool {
        *self == DAC_VOUT_SEL_A::COMPB_REF
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == DAC_VOUT_SEL_A::NC
    }
}
#[doc = "Field `DAC_VOUT_SEL` writer - 2:0\\]
DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\]."]
pub type DAC_VOUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DACCTL_SPEC, u8, DAC_VOUT_SEL_A, 3, O>;
impl<'a, const O: u8> DAC_VOUT_SEL_W<'a, O> {
    #[doc = "Connect to COMPA_IN analog node. Required setting to drive external load selected in ADI_4_AUX:MUX1.COMPA_IN."]
    #[inline(always)]
    pub fn compa_in(self) -> &'a mut W {
        self.variant(DAC_VOUT_SEL_A::COMPA_IN)
    }
    #[doc = "Connect to COMPA_REF analog node. It is not possible to drive external loads connected to COMPA_REF I/O mux with this setting."]
    #[inline(always)]
    pub fn compa_ref(self) -> &'a mut W {
        self.variant(DAC_VOUT_SEL_A::COMPA_REF)
    }
    #[doc = "Connect to COMPB_REF analog node. Required setting to use Comparator B."]
    #[inline(always)]
    pub fn compb_ref(self) -> &'a mut W {
        self.variant(DAC_VOUT_SEL_A::COMPB_REF)
    }
    #[doc = "Connect to nothing It is recommended to use NC as intermediate step when you change DAC_VOUT_SEL."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(DAC_VOUT_SEL_A::NC)
    }
}
#[doc = "Field `DAC_PRECHARGE_EN` reader - 3:3\\]
DAC precharge enable. Only enable precharge when ADI_4_AUX:MUX2.DAC_VREF_SEL equals DCOUPL and VDDS is higher than 2.65 V. DAC output voltage range: 0: 0 V to 1.28 V. 1: 1.28 V to 2.56 V. Otherwise, see ADI_4_AUX:MUX2.DAC_VREF_SEL for DAC output voltage range. Enable precharge 1 us before you enable the DAC and the buffer."]
pub type DAC_PRECHARGE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DAC_PRECHARGE_EN` writer - 3:3\\]
DAC precharge enable. Only enable precharge when ADI_4_AUX:MUX2.DAC_VREF_SEL equals DCOUPL and VDDS is higher than 2.65 V. DAC output voltage range: 0: 0 V to 1.28 V. 1: 1.28 V to 2.56 V. Otherwise, see ADI_4_AUX:MUX2.DAC_VREF_SEL for DAC output voltage range. Enable precharge 1 us before you enable the DAC and the buffer."]
pub type DAC_PRECHARGE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACCTL_SPEC, bool, O>;
#[doc = "Field `DAC_BUFFER_EN` reader - 4:4\\]
DAC buffer enable. DAC buffer reduces the time required to produce the programmed voltage at the expense of increased current consumption. 0: Disable DAC buffer. 1: Enable DAC buffer. Enable buffer when DAC_VOUT_SEL equals COMPA_IN. Do not enable the buffer when AUX_SYSIF:OPMODEREQ.REQ equals PDA or PDLP."]
pub type DAC_BUFFER_EN_R = crate::BitReader<bool>;
#[doc = "Field `DAC_BUFFER_EN` writer - 4:4\\]
DAC buffer enable. DAC buffer reduces the time required to produce the programmed voltage at the expense of increased current consumption. 0: Disable DAC buffer. 1: Enable DAC buffer. Enable buffer when DAC_VOUT_SEL equals COMPA_IN. Do not enable the buffer when AUX_SYSIF:OPMODEREQ.REQ equals PDA or PDLP."]
pub type DAC_BUFFER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACCTL_SPEC, bool, O>;
#[doc = "Field `DAC_EN` reader - 5:5\\]
DAC module enable. 0: Disable DAC. 1: Enable DAC. The Sensor Controller must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA. The System CPU must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA in Standby mode. The System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE to use the DAC in Active and Idle modes. Standby, Active, and Idle are power modes defined in TI’s Power Manager."]
pub type DAC_EN_R = crate::BitReader<bool>;
#[doc = "Field `DAC_EN` writer - 5:5\\]
DAC module enable. 0: Disable DAC. 1: Enable DAC. The Sensor Controller must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA. The System CPU must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA in Standby mode. The System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE to use the DAC in Active and Idle modes. Standby, Active, and Idle are power modes defined in TI’s Power Manager."]
pub type DAC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DACCTL_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\]."]
    #[inline(always)]
    pub fn dac_vout_sel(&self) -> DAC_VOUT_SEL_R {
        DAC_VOUT_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
DAC precharge enable. Only enable precharge when ADI_4_AUX:MUX2.DAC_VREF_SEL equals DCOUPL and VDDS is higher than 2.65 V. DAC output voltage range: 0: 0 V to 1.28 V. 1: 1.28 V to 2.56 V. Otherwise, see ADI_4_AUX:MUX2.DAC_VREF_SEL for DAC output voltage range. Enable precharge 1 us before you enable the DAC and the buffer."]
    #[inline(always)]
    pub fn dac_precharge_en(&self) -> DAC_PRECHARGE_EN_R {
        DAC_PRECHARGE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
DAC buffer enable. DAC buffer reduces the time required to produce the programmed voltage at the expense of increased current consumption. 0: Disable DAC buffer. 1: Enable DAC buffer. Enable buffer when DAC_VOUT_SEL equals COMPA_IN. Do not enable the buffer when AUX_SYSIF:OPMODEREQ.REQ equals PDA or PDLP."]
    #[inline(always)]
    pub fn dac_buffer_en(&self) -> DAC_BUFFER_EN_R {
        DAC_BUFFER_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
DAC module enable. 0: Disable DAC. 1: Enable DAC. The Sensor Controller must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA. The System CPU must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA in Standby mode. The System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE to use the DAC in Active and Idle modes. Standby, Active, and Idle are power modes defined in TI’s Power Manager."]
    #[inline(always)]
    pub fn dac_en(&self) -> DAC_EN_R {
        DAC_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\]."]
    #[inline(always)]
    #[must_use]
    pub fn dac_vout_sel(&mut self) -> DAC_VOUT_SEL_W<0> {
        DAC_VOUT_SEL_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
DAC precharge enable. Only enable precharge when ADI_4_AUX:MUX2.DAC_VREF_SEL equals DCOUPL and VDDS is higher than 2.65 V. DAC output voltage range: 0: 0 V to 1.28 V. 1: 1.28 V to 2.56 V. Otherwise, see ADI_4_AUX:MUX2.DAC_VREF_SEL for DAC output voltage range. Enable precharge 1 us before you enable the DAC and the buffer."]
    #[inline(always)]
    #[must_use]
    pub fn dac_precharge_en(&mut self) -> DAC_PRECHARGE_EN_W<3> {
        DAC_PRECHARGE_EN_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
DAC buffer enable. DAC buffer reduces the time required to produce the programmed voltage at the expense of increased current consumption. 0: Disable DAC buffer. 1: Enable DAC buffer. Enable buffer when DAC_VOUT_SEL equals COMPA_IN. Do not enable the buffer when AUX_SYSIF:OPMODEREQ.REQ equals PDA or PDLP."]
    #[inline(always)]
    #[must_use]
    pub fn dac_buffer_en(&mut self) -> DAC_BUFFER_EN_W<4> {
        DAC_BUFFER_EN_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
DAC module enable. 0: Disable DAC. 1: Enable DAC. The Sensor Controller must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA. The System CPU must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA in Standby mode. The System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE to use the DAC in Active and Idle modes. Standby, Active, and Idle are power modes defined in TI’s Power Manager."]
    #[inline(always)]
    #[must_use]
    pub fn dac_en(&mut self) -> DAC_EN_W<5> {
        DAC_EN_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Control This register controls the analog part of the DAC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacctl](index.html) module"]
pub struct DACCTL_SPEC;
impl crate::RegisterSpec for DACCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacctl::R](R) reader structure"]
impl crate::Readable for DACCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacctl::W](W) writer structure"]
impl crate::Writable for DACCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACCTL to value 0"]
impl crate::Resettable for DACCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
