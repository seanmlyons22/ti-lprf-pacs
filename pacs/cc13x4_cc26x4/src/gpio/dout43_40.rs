#[doc = "Register `DOUT43_40` reader"]
pub type R = crate::R<Dout43_40Spec>;
#[doc = "Register `DOUT43_40` writer"]
pub type W = crate::W<Dout43_40Spec>;
#[doc = "Field `DIO40` reader - 0:0\\]
Sets the state of the pin that is configured as DIO#40, if the corresponding DOE47_0 bitfield is set."]
pub type Dio40R = crate::BitReader;
#[doc = "Field `DIO40` writer - 0:0\\]
Sets the state of the pin that is configured as DIO#40, if the corresponding DOE47_0 bitfield is set."]
pub type Dio40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIO41` reader - 8:8\\]
Sets the state of the pin that is configured as DIO#41, if the corresponding DOE47_0 bitfield is set."]
pub type Dio41R = crate::BitReader;
#[doc = "Field `DIO41` writer - 8:8\\]
Sets the state of the pin that is configured as DIO#41, if the corresponding DOE47_0 bitfield is set."]
pub type Dio41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `RESERVED9` writer - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIO42` reader - 16:16\\]
Sets the state of the pin that is configured as DIO#42, if the corresponding DOE47_0 bitfield is set."]
pub type Dio42R = crate::BitReader;
#[doc = "Field `DIO42` writer - 16:16\\]
Sets the state of the pin that is configured as DIO#42, if the corresponding DOE47_0 bitfield is set."]
pub type Dio42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "Field `RESERVED17` writer - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIO43` reader - 24:24\\]
Sets the state of the pin that is configured as DIO#43, if the corresponding DOE47_0 bitfield is set."]
pub type Dio43R = crate::BitReader;
#[doc = "Field `DIO43` writer - 24:24\\]
Sets the state of the pin that is configured as DIO#43, if the corresponding DOE47_0 bitfield is set."]
pub type Dio43W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#40, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio40(&self) -> Dio40R {
        Dio40R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#41, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio41(&self) -> Dio41R {
        Dio41R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#42, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio42(&self) -> Dio42R {
        Dio42R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#43, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio43(&self) -> Dio43R {
        Dio43R::new(((self.bits >> 24) & 1) != 0)
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
Sets the state of the pin that is configured as DIO#40, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio40(&mut self) -> Dio40W<Dout43_40Spec> {
        Dio40W::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Dout43_40Spec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#41, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio41(&mut self) -> Dio41W<Dout43_40Spec> {
        Dio41W::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<Dout43_40Spec> {
        Reserved9W::new(self, 9)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#42, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio42(&mut self) -> Dio42W<Dout43_40Spec> {
        Dio42W::new(self, 16)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<Dout43_40Spec> {
        Reserved17W::new(self, 17)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#43, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio43(&mut self) -> Dio43W<Dout43_40Spec> {
        Dio43W::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> Reserved25W<Dout43_40Spec> {
        Reserved25W::new(self, 25)
    }
}
#[doc = "Data Out 43 to 40 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout43_40::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout43_40::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout43_40Spec;
impl crate::RegisterSpec for Dout43_40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout43_40::R`](R) reader structure"]
impl crate::Readable for Dout43_40Spec {}
#[doc = "`write(|w| ..)` method takes [`dout43_40::W`](W) writer structure"]
impl crate::Writable for Dout43_40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT43_40 to value 0"]
impl crate::Resettable for Dout43_40Spec {
    const RESET_VALUE: u32 = 0;
}
