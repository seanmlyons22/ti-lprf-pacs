#[doc = "Register `BATMONTEMP` reader"]
pub type R = crate::R<BatmontempSpec>;
#[doc = "Register `BATMONTEMP` writer"]
pub type W = crate::W<BatmontempSpec>;
#[doc = "Field `FRAC` reader - 1:0\\]
See AON_BATMON:TEMP.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
pub type FracR = crate::FieldReader;
#[doc = "Field `FRAC` writer - 1:0\\]
See AON_BATMON:TEMP.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
pub type FracW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INT` reader - 10:2\\]
See AON_BATMON:TEMP.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
pub type IntR = crate::FieldReader<u16>;
#[doc = "Field `INT` writer - 10:2\\]
See AON_BATMON:TEMP.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
pub type IntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SIGN` reader - 15:11\\]
Sign extension of INT. Follow this procedure to get the correct value: - Do two dummy reads of SIGN. - Then read SIGN until two consecutive reads are equal."]
pub type SignR = crate::FieldReader;
#[doc = "Field `SIGN` writer - 15:11\\]
Sign extension of INT. Follow this procedure to get the correct value: - Do two dummy reads of SIGN. - Then read SIGN until two consecutive reads are equal."]
pub type SignW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
See AON_BATMON:TEMP.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
    #[inline(always)]
    pub fn frac(&self) -> FracR {
        FracR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:10 - 10:2\\]
See AON_BATMON:TEMP.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Sign extension of INT. Follow this procedure to get the correct value: - Do two dummy reads of SIGN. - Then read SIGN until two consecutive reads are equal."]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
See AON_BATMON:TEMP.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
    #[inline(always)]
    #[must_use]
    pub fn frac(&mut self) -> FracW<BatmontempSpec> {
        FracW::new(self, 0)
    }
    #[doc = "Bits 2:10 - 10:2\\]
See AON_BATMON:TEMP.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<BatmontempSpec> {
        IntW::new(self, 2)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Sign extension of INT. Follow this procedure to get the correct value: - Do two dummy reads of SIGN. - Then read SIGN until two consecutive reads are equal."]
    #[inline(always)]
    #[must_use]
    pub fn sign(&mut self) -> SignW<BatmontempSpec> {
        SignW::new(self, 11)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<BatmontempSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "AON_BATMON Temperature Value Read access to AON_BATMON:TEMP. System CPU must not access this register. Instead, system CPU must access AON_BATMON:TEMP directly. AON_BATMON:TEMP updates during VDDR recharge or active operational mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batmontemp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batmontemp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BatmontempSpec;
impl crate::RegisterSpec for BatmontempSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`batmontemp::R`](R) reader structure"]
impl crate::Readable for BatmontempSpec {}
#[doc = "`write(|w| ..)` method takes [`batmontemp::W`](W) writer structure"]
impl crate::Writable for BatmontempSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BATMONTEMP to value 0"]
impl crate::Resettable for BatmontempSpec {
    const RESET_VALUE: u32 = 0;
}
