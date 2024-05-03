#[doc = "Register `JTAGUSERCODE` reader"]
pub type R = crate::R<JtagusercodeSpec>;
#[doc = "Register `JTAGUSERCODE` writer"]
pub type W = crate::W<JtagusercodeSpec>;
#[doc = "Field `USER_CODE` reader - 31:0\\]
32-bit JTAG USERCODE register feeding main JTAG TAP NB: This field can be locked"]
pub type UserCodeR = crate::FieldReader<u32>;
#[doc = "Field `USER_CODE` writer - 31:0\\]
32-bit JTAG USERCODE register feeding main JTAG TAP NB: This field can be locked"]
pub type UserCodeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
32-bit JTAG USERCODE register feeding main JTAG TAP NB: This field can be locked"]
    #[inline(always)]
    pub fn user_code(&self) -> UserCodeR {
        UserCodeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
32-bit JTAG USERCODE register feeding main JTAG TAP NB: This field can be locked"]
    #[inline(always)]
    #[must_use]
    pub fn user_code(&mut self) -> UserCodeW<JtagusercodeSpec> {
        UserCodeW::new(self, 0)
    }
}
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtagusercode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtagusercode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JtagusercodeSpec;
impl crate::RegisterSpec for JtagusercodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtagusercode::R`](R) reader structure"]
impl crate::Readable for JtagusercodeSpec {}
#[doc = "`write(|w| ..)` method takes [`jtagusercode::W`](W) writer structure"]
impl crate::Writable for JtagusercodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JTAGUSERCODE to value 0x0b99_a02f"]
impl crate::Resettable for JtagusercodeSpec {
    const RESET_VALUE: u32 = 0x0b99_a02f;
}
