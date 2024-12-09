#[doc = "Register `BATMONBAT` reader"]
pub type R = crate::R<BatmonbatSpec>;
#[doc = "Register `BATMONBAT` writer"]
pub type W = crate::W<BatmonbatSpec>;
#[doc = "Field `FRAC` reader - 7:0\\]
See AON_BATMON:BAT.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
pub type FracR = crate::FieldReader;
#[doc = "Field `INT` reader - 10:8\\]
See AON_BATMON:BAT.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
pub type IntR = crate::FieldReader;
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
See AON_BATMON:BAT.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
    #[inline(always)]
    pub fn frac(&self) -> FracR {
        FracR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
See AON_BATMON:BAT.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {}
#[doc = "AON_BATMON Battery Voltage Value Read access to AON_BATMON:BAT. System CPU must not access this register. Instead, system CPU must access AON_BATMON:BAT directly. AON_BATMON:BAT updates during VDDR recharge or active operational mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batmonbat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batmonbat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BatmonbatSpec;
impl crate::RegisterSpec for BatmonbatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`batmonbat::R`](R) reader structure"]
impl crate::Readable for BatmonbatSpec {}
#[doc = "`write(|w| ..)` method takes [`batmonbat::W`](W) writer structure"]
impl crate::Writable for BatmonbatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BATMONBAT to value 0"]
impl crate::Resettable for BatmonbatSpec {
    const RESET_VALUE: u32 = 0;
}
