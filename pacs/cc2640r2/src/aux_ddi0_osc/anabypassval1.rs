#[doc = "Register `ANABYPASSVAL1` reader"]
pub type R = crate::R<Anabypassval1Spec>;
#[doc = "Register `ANABYPASSVAL1` writer"]
pub type W = crate::W<Anabypassval1Spec>;
#[doc = "Field `XOSC_HF_COLUMN_Q12` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type XoscHfColumnQ12R = crate::FieldReader<u16>;
#[doc = "Field `XOSC_HF_COLUMN_Q12` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type XoscHfColumnQ12W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `XOSC_HF_ROW_Q12` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type XoscHfRowQ12R = crate::FieldReader;
#[doc = "Field `XOSC_HF_ROW_Q12` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type XoscHfRowQ12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED20` writer - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_column_q12(&self) -> XoscHfColumnQ12R {
        XoscHfColumnQ12R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_row_q12(&self) -> XoscHfRowQ12R {
        XoscHfRowQ12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_column_q12(&mut self) -> XoscHfColumnQ12W<Anabypassval1Spec> {
        XoscHfColumnQ12W::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_row_q12(&mut self) -> XoscHfRowQ12W<Anabypassval1Spec> {
        XoscHfRowQ12W::new(self, 16)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> Reserved20W<Anabypassval1Spec> {
        Reserved20W::new(self, 20)
    }
}
#[doc = "Analog Bypass Values 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`anabypassval1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`anabypassval1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Anabypassval1Spec;
impl crate::RegisterSpec for Anabypassval1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anabypassval1::R`](R) reader structure"]
impl crate::Readable for Anabypassval1Spec {}
#[doc = "`write(|w| ..)` method takes [`anabypassval1::W`](W) writer structure"]
impl crate::Writable for Anabypassval1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANABYPASSVAL1 to value 0"]
impl crate::Resettable for Anabypassval1Spec {
    const RESET_VALUE: u32 = 0;
}
