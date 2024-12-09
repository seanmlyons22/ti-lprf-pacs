#[doc = "Register `DOUT15_12` reader"]
pub type R = crate::R<Dout15_12Spec>;
#[doc = "Register `DOUT15_12` writer"]
pub type W = crate::W<Dout15_12Spec>;
#[doc = "Field `DIO12` writer - 0:0\\]
Sets the state of the pin that is configured as DIO#12, if the corresponding DOE31_0 bitfield is set."]
pub type Dio12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DIO13` writer - 8:8\\]
Sets the state of the pin that is configured as DIO#13, if the corresponding DOE31_0 bitfield is set."]
pub type Dio13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `DIO14` writer - 16:16\\]
Sets the state of the pin that is configured as DIO#14, if the corresponding DOE31_0 bitfield is set."]
pub type Dio14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "Field `DIO15` writer - 24:24\\]
Sets the state of the pin that is configured as DIO#15, if the corresponding DOE31_0 bitfield is set."]
pub type Dio15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#12, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio12(&mut self) -> Dio12W<Dout15_12Spec> {
        Dio12W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#13, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio13(&mut self) -> Dio13W<Dout15_12Spec> {
        Dio13W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#14, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio14(&mut self) -> Dio14W<Dout15_12Spec> {
        Dio14W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#15, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio15(&mut self) -> Dio15W<Dout15_12Spec> {
        Dio15W::new(self, 24)
    }
}
#[doc = "Data Out 12 to 15 Alias register for byte access to each bit in DOUT31_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout15_12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout15_12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout15_12Spec;
impl crate::RegisterSpec for Dout15_12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout15_12::R`](R) reader structure"]
impl crate::Readable for Dout15_12Spec {}
#[doc = "`write(|w| ..)` method takes [`dout15_12::W`](W) writer structure"]
impl crate::Writable for Dout15_12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT15_12 to value 0"]
impl crate::Resettable for Dout15_12Spec {
    const RESET_VALUE: u32 = 0;
}
