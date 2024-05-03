#[doc = "Register `IOPOE` reader"]
pub type R = crate::R<IopoeSpec>;
#[doc = "Register `IOPOE` writer"]
pub type W = crate::W<IopoeSpec>;
#[doc = "Field `IO7_0` reader - 7:0\\]
Write 1 to bit index n in this bit vector to configure AUXIO\\[8i+n\\]
to be driven from source given in \\[IOnPSEL.*\\]. Write 0 to bit index n in this bit vector to configure AUXIO\\[8i+n\\]
to be driven from bit n in GPIODOUT."]
pub type Io7_0R = crate::FieldReader;
#[doc = "Field `IO7_0` writer - 7:0\\]
Write 1 to bit index n in this bit vector to configure AUXIO\\[8i+n\\]
to be driven from source given in \\[IOnPSEL.*\\]. Write 0 to bit index n in this bit vector to configure AUXIO\\[8i+n\\]
to be driven from bit n in GPIODOUT."]
pub type Io7_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Write 1 to bit index n in this bit vector to configure AUXIO\\[8i+n\\]
to be driven from source given in \\[IOnPSEL.*\\]. Write 0 to bit index n in this bit vector to configure AUXIO\\[8i+n\\]
to be driven from bit n in GPIODOUT."]
    #[inline(always)]
    pub fn io7_0(&self) -> Io7_0R {
        Io7_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Write 1 to bit index n in this bit vector to configure AUXIO\\[8i+n\\]
to be driven from source given in \\[IOnPSEL.*\\]. Write 0 to bit index n in this bit vector to configure AUXIO\\[8i+n\\]
to be driven from bit n in GPIODOUT."]
    #[inline(always)]
    #[must_use]
    pub fn io7_0(&mut self) -> Io7_0W<IopoeSpec> {
        Io7_0W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<IopoeSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Input Output Peripheral Output Enable This register selects the output source for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iopoe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iopoe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopoeSpec;
impl crate::RegisterSpec for IopoeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopoe::R`](R) reader structure"]
impl crate::Readable for IopoeSpec {}
#[doc = "`write(|w| ..)` method takes [`iopoe::W`](W) writer structure"]
impl crate::Writable for IopoeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOPOE to value 0"]
impl crate::Resettable for IopoeSpec {
    const RESET_VALUE: u32 = 0;
}
