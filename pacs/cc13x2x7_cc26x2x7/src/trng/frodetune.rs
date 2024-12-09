#[doc = "Register `FRODETUNE` reader"]
pub type R = crate::R<FrodetuneSpec>;
#[doc = "Register `FRODETUNE` writer"]
pub type W = crate::W<FrodetuneSpec>;
#[doc = "Field `FRO_MASK` reader - 23:0\\]
De-tune bits for the individual FROs. A '1' in bit \\[n\\]
lets FRO 'n' run approximately 5% faster. The value of one of these bits may only be changed while the corresponding FRO is turned off (by temporarily writing a '0' in the corresponding bit of the FROEN.FRO_MASK register)."]
pub type FroMaskR = crate::FieldReader<u32>;
#[doc = "Field `FRO_MASK` writer - 23:0\\]
De-tune bits for the individual FROs. A '1' in bit \\[n\\]
lets FRO 'n' run approximately 5% faster. The value of one of these bits may only be changed while the corresponding FRO is turned off (by temporarily writing a '0' in the corresponding bit of the FROEN.FRO_MASK register)."]
pub type FroMaskW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
De-tune bits for the individual FROs. A '1' in bit \\[n\\]
lets FRO 'n' run approximately 5% faster. The value of one of these bits may only be changed while the corresponding FRO is turned off (by temporarily writing a '0' in the corresponding bit of the FROEN.FRO_MASK register)."]
    #[inline(always)]
    pub fn fro_mask(&self) -> FroMaskR {
        FroMaskR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
De-tune bits for the individual FROs. A '1' in bit \\[n\\]
lets FRO 'n' run approximately 5% faster. The value of one of these bits may only be changed while the corresponding FRO is turned off (by temporarily writing a '0' in the corresponding bit of the FROEN.FRO_MASK register)."]
    #[inline(always)]
    #[must_use]
    pub fn fro_mask(&mut self) -> FroMaskW<FrodetuneSpec> {
        FroMaskW::new(self, 0)
    }
}
#[doc = "FRO De-tune Bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frodetune::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frodetune::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrodetuneSpec;
impl crate::RegisterSpec for FrodetuneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frodetune::R`](R) reader structure"]
impl crate::Readable for FrodetuneSpec {}
#[doc = "`write(|w| ..)` method takes [`frodetune::W`](W) writer structure"]
impl crate::Writable for FrodetuneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRODETUNE to value 0"]
impl crate::Resettable for FrodetuneSpec {
    const RESET_VALUE: u32 = 0;
}
