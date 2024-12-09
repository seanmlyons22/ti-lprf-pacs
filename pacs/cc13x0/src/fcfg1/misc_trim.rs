#[doc = "Register `MISC_TRIM` reader"]
pub type R = crate::R<MiscTrimSpec>;
#[doc = "Register `MISC_TRIM` writer"]
pub type W = crate::W<MiscTrimSpec>;
#[doc = "Field `TEMPVSLOPE` reader - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
pub type TempvslopeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
    #[inline(always)]
    pub fn tempvslope(&self) -> TempvslopeR {
        TempvslopeR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Miscellaneous Trim Parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscTrimSpec;
impl crate::RegisterSpec for MiscTrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_trim::R`](R) reader structure"]
impl crate::Readable for MiscTrimSpec {}
#[doc = "`write(|w| ..)` method takes [`misc_trim::W`](W) writer structure"]
impl crate::Writable for MiscTrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC_TRIM to value 0xffff_ff33"]
impl crate::Resettable for MiscTrimSpec {
    const RESET_VALUE: u32 = 0xffff_ff33;
}
