#[doc = "Register `ATESTCTL` reader"]
pub type R = crate::R<AtestctlSpec>;
#[doc = "Register `ATESTCTL` writer"]
pub type W = crate::W<AtestctlSpec>;
#[doc = "Field `RESERVED0` reader - 28:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED0` writer - 28:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `SCLK_LF_AUX_EN` reader - 29:29\\]
Enable 32 kHz clock to AUX_COMPB."]
pub type SclkLfAuxEnR = crate::BitReader;
#[doc = "Field `SCLK_LF_AUX_EN` writer - 29:29\\]
Enable 32 kHz clock to AUX_COMPB."]
pub type SclkLfAuxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare30R = crate::FieldReader;
#[doc = "Field `SPARE30` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare30W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:28 - 28:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - 29:29\\]
Enable 32 kHz clock to AUX_COMPB."]
    #[inline(always)]
    pub fn sclk_lf_aux_en(&self) -> SclkLfAuxEnR {
        SclkLfAuxEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare30(&self) -> Spare30R {
        Spare30R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:28 - 28:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AtestctlSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Enable 32 kHz clock to AUX_COMPB."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_lf_aux_en(&mut self) -> SclkLfAuxEnW<AtestctlSpec> {
        SclkLfAuxEnW::new(self, 29)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare30(&mut self) -> Spare30W<AtestctlSpec> {
        Spare30W::new(self, 30)
    }
}
#[doc = "Analog Test Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atestctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atestctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtestctlSpec;
impl crate::RegisterSpec for AtestctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atestctl::R`](R) reader structure"]
impl crate::Readable for AtestctlSpec {}
#[doc = "`write(|w| ..)` method takes [`atestctl::W`](W) writer structure"]
impl crate::Writable for AtestctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATESTCTL to value 0"]
impl crate::Resettable for AtestctlSpec {
    const RESET_VALUE: u32 = 0;
}
