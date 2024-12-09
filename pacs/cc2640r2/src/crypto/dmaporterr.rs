#[doc = "Register `DMAPORTERR` reader"]
pub type R = crate::R<DmaporterrSpec>;
#[doc = "Register `DMAPORTERR` writer"]
pub type W = crate::W<DmaporterrSpec>;
#[doc = "Field `RESERVED0` reader - 8:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `LAST_CH` reader - 9:9\\]
Indicates which channel was serviced last (channel 0 or channel 1) by the AHB master port."]
pub type LastChR = crate::BitReader;
#[doc = "Field `RESERVED10` reader - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader;
#[doc = "Field `AHB_ERR` reader - 12:12\\]
A 1 indicates that the Crypto peripheral has detected an AHB bus error"]
pub type AhbErrR = crate::BitReader;
#[doc = "Field `RESERVED13` reader - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates which channel was serviced last (channel 0 or channel 1) by the AHB master port."]
    #[inline(always)]
    pub fn last_ch(&self) -> LastChR {
        LastChR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
A 1 indicates that the Crypto peripheral has detected an AHB bus error"]
    #[inline(always)]
    pub fn ahb_err(&self) -> AhbErrR {
        AhbErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {}
#[doc = "DMA Controller Port Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaporterr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaporterr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaporterrSpec;
impl crate::RegisterSpec for DmaporterrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaporterr::R`](R) reader structure"]
impl crate::Readable for DmaporterrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaporterr::W`](W) writer structure"]
impl crate::Writable for DmaporterrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAPORTERR to value 0"]
impl crate::Resettable for DmaporterrSpec {
    const RESET_VALUE: u32 = 0;
}
