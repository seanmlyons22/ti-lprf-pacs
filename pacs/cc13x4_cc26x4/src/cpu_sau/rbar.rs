#[doc = "Register `RBAR` reader"]
pub type R = crate::R<RbarSpec>;
#[doc = "Register `RBAR` writer"]
pub type W = crate::W<RbarSpec>;
#[doc = "Field `RESERVED0` reader - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BADDR` reader - 31:5\\]
Holds bits \\[31:5\\]
of the base address for the selected SAU region"]
pub type BaddrR = crate::FieldReader<u32>;
#[doc = "Field `BADDR` writer - 31:5\\]
Holds bits \\[31:5\\]
of the base address for the selected SAU region"]
pub type BaddrW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Holds bits \\[31:5\\]
of the base address for the selected SAU region"]
    #[inline(always)]
    pub fn baddr(&self) -> BaddrR {
        BaddrR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<RbarSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Holds bits \\[31:5\\]
of the base address for the selected SAU region"]
    #[inline(always)]
    #[must_use]
    pub fn baddr(&mut self) -> BaddrW<RbarSpec> {
        BaddrW::new(self, 5)
    }
}
#[doc = "Provides indirect read and write access to the base address of the currently selected SAU region\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbarSpec;
impl crate::RegisterSpec for RbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbar::R`](R) reader structure"]
impl crate::Readable for RbarSpec {}
#[doc = "`write(|w| ..)` method takes [`rbar::W`](W) writer structure"]
impl crate::Writable for RbarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RBAR to value 0"]
impl crate::Resettable for RbarSpec {
    const RESET_VALUE: u32 = 0;
}
