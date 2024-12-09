#[doc = "Register `MISC_CONF_1` reader"]
pub type R = crate::R<MiscConf1Spec>;
#[doc = "Register `MISC_CONF_1` writer"]
pub type W = crate::W<MiscConf1Spec>;
#[doc = "Field `DEVICE_MINOR_REV` reader - 7:0\\]
HW minor revision number (a value of 0xFF shall be treated equally to 0x00). Any test of this field by SW should be implemented as a 'greater or equal' comparison as signed integer. Value may change without warning."]
pub type DeviceMinorRevR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
HW minor revision number (a value of 0xFF shall be treated equally to 0x00). Any test of this field by SW should be implemented as a 'greater or equal' comparison as signed integer. Value may change without warning."]
    #[inline(always)]
    pub fn device_minor_rev(&self) -> DeviceMinorRevR {
        DeviceMinorRevR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Misc configurations\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_conf_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscConf1Spec;
impl crate::RegisterSpec for MiscConf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_conf_1::R`](R) reader structure"]
impl crate::Readable for MiscConf1Spec {}
#[doc = "`write(|w| ..)` method takes [`misc_conf_1::W`](W) writer structure"]
impl crate::Writable for MiscConf1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC_CONF_1 to value 0xffff_ff00"]
impl crate::Resettable for MiscConf1Spec {
    const RESET_VALUE: u32 = 0xffff_ff00;
}
