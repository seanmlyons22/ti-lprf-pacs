#[doc = "Register `TMUTE2` reader"]
pub type R = crate::R<Tmute2Spec>;
#[doc = "Register `TMUTE2` writer"]
pub type W = crate::W<Tmute2Spec>;
#[doc = "Field `CDACU` reader - 1:0\\]
SOCADC: Upper 2 bits of CDAC trim. These bits are used in DTC."]
pub type CdacuR = crate::FieldReader;
#[doc = "Field `CDACU` writer - 1:0\\]
SOCADC: Upper 2 bits of CDAC trim. These bits are used in DTC."]
pub type CdacuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES` reader - 3:2\\]
SOCADC: Resistor trim bits. These bits are used in the analog IP."]
pub type ResR = crate::FieldReader;
#[doc = "Field `RES` writer - 3:2\\]
SOCADC: Resistor trim bits. These bits are used in the analog IP."]
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFFSET` reader - 15:4\\]
SOCADC: Offset trim bits. These bits are used in DTC."]
pub type OffsetR = crate::FieldReader<u16>;
#[doc = "Field `OFFSET` writer - 15:4\\]
SOCADC: Offset trim bits. These bits are used in DTC."]
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `LATCH` reader - 22:16\\]
SOC ADC: Latch trim bits. These bits are used in the analog IP."]
pub type LatchR = crate::FieldReader;
#[doc = "Field `LATCH` writer - 22:16\\]
SOC ADC: Latch trim bits. These bits are used in the analog IP."]
pub type LatchW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TRIM` reader - 25:23\\]
ADC REFBUF trim bits."]
pub type TrimR = crate::FieldReader;
#[doc = "Field `TRIM` writer - 25:23\\]
ADC REFBUF trim bits."]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IBTRIM` reader - 30:26\\]
LPCOMP: Bias current trim, 250nA to be terminated across I2V, 1M ohm setting. Resulting target trim voltage 250mV."]
pub type IbtrimR = crate::FieldReader;
#[doc = "Field `IBTRIM` writer - 30:26\\]
LPCOMP: Bias current trim, 250nA to be terminated across I2V, 1M ohm setting. Resulting target trim voltage 250mV."]
pub type IbtrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED31` reader - 31:31\\]
RESERVED"]
pub type Reserved31R = crate::BitReader;
#[doc = "Field `RESERVED31` writer - 31:31\\]
RESERVED"]
pub type Reserved31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
SOCADC: Upper 2 bits of CDAC trim. These bits are used in DTC."]
    #[inline(always)]
    pub fn cdacu(&self) -> CdacuR {
        CdacuR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
SOCADC: Resistor trim bits. These bits are used in the analog IP."]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
SOCADC: Offset trim bits. These bits are used in DTC."]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:22 - 22:16\\]
SOC ADC: Latch trim bits. These bits are used in the analog IP."]
    #[inline(always)]
    pub fn latch(&self) -> LatchR {
        LatchR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:25 - 25:23\\]
ADC REFBUF trim bits."]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:30 - 30:26\\]
LPCOMP: Bias current trim, 250nA to be terminated across I2V, 1M ohm setting. Resulting target trim voltage 250mV."]
    #[inline(always)]
    pub fn ibtrim(&self) -> IbtrimR {
        IbtrimR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
RESERVED"]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
SOCADC: Upper 2 bits of CDAC trim. These bits are used in DTC."]
    #[inline(always)]
    #[must_use]
    pub fn cdacu(&mut self) -> CdacuW<Tmute2Spec> {
        CdacuW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
SOCADC: Resistor trim bits. These bits are used in the analog IP."]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> ResW<Tmute2Spec> {
        ResW::new(self, 2)
    }
    #[doc = "Bits 4:15 - 15:4\\]
SOCADC: Offset trim bits. These bits are used in DTC."]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OffsetW<Tmute2Spec> {
        OffsetW::new(self, 4)
    }
    #[doc = "Bits 16:22 - 22:16\\]
SOC ADC: Latch trim bits. These bits are used in the analog IP."]
    #[inline(always)]
    #[must_use]
    pub fn latch(&mut self) -> LatchW<Tmute2Spec> {
        LatchW::new(self, 16)
    }
    #[doc = "Bits 23:25 - 25:23\\]
ADC REFBUF trim bits."]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TrimW<Tmute2Spec> {
        TrimW::new(self, 23)
    }
    #[doc = "Bits 26:30 - 30:26\\]
LPCOMP: Bias current trim, 250nA to be terminated across I2V, 1M ohm setting. Resulting target trim voltage 250mV."]
    #[inline(always)]
    #[must_use]
    pub fn ibtrim(&mut self) -> IbtrimW<Tmute2Spec> {
        IbtrimW::new(self, 26)
    }
    #[doc = "Bit 31 - 31:31\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> Reserved31W<Tmute2Spec> {
        Reserved31W::new(self, 31)
    }
}
#[doc = "TMUTE2 trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmute2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmute2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmute2Spec;
impl crate::RegisterSpec for Tmute2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmute2::R`](R) reader structure"]
impl crate::Readable for Tmute2Spec {}
#[doc = "`write(|w| ..)` method takes [`tmute2::W`](W) writer structure"]
impl crate::Writable for Tmute2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMUTE2 to value 0x0080_0000"]
impl crate::Resettable for Tmute2Spec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
