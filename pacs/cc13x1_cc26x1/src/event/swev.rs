#[doc = "Register `SWEV` reader"]
pub type R = crate::R<SwevSpec>;
#[doc = "Register `SWEV` writer"]
pub type W = crate::W<SwevSpec>;
#[doc = "Field `SWEV0` reader - 0:0\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 0 event."]
pub type Swev0R = crate::BitReader;
#[doc = "Field `SWEV0` writer - 0:0\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 0 event."]
pub type Swev0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED0` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `SWEV1` reader - 8:8\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 1 event."]
pub type Swev1R = crate::BitReader;
#[doc = "Field `SWEV1` writer - 8:8\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 1 event."]
pub type Swev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SWEV2` reader - 16:16\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 2 event."]
pub type Swev2R = crate::BitReader;
#[doc = "Field `SWEV2` writer - 16:16\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 2 event."]
pub type Swev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SWEV3` reader - 24:24\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 3 event."]
pub type Swev3R = crate::BitReader;
#[doc = "Field `SWEV3` writer - 24:24\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 3 event."]
pub type Swev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 0 event."]
    #[inline(always)]
    pub fn swev0(&self) -> Swev0R {
        Swev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 1 event."]
    #[inline(always)]
    pub fn swev1(&self) -> Swev1R {
        Swev1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 2 event."]
    #[inline(always)]
    pub fn swev2(&self) -> Swev2R {
        Swev2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 3 event."]
    #[inline(always)]
    pub fn swev3(&self) -> Swev3R {
        Swev3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 0 event."]
    #[inline(always)]
    #[must_use]
    pub fn swev0(&mut self) -> Swev0W<SwevSpec> {
        Swev0W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 1 event."]
    #[inline(always)]
    #[must_use]
    pub fn swev1(&mut self) -> Swev1W<SwevSpec> {
        Swev1W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 2 event."]
    #[inline(always)]
    #[must_use]
    pub fn swev2(&mut self) -> Swev2W<SwevSpec> {
        Swev2W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 3 event."]
    #[inline(always)]
    #[must_use]
    pub fn swev3(&mut self) -> Swev3W<SwevSpec> {
        Swev3W::new(self, 24)
    }
}
#[doc = "Set or Clear Software Events\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwevSpec;
impl crate::RegisterSpec for SwevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swev::R`](R) reader structure"]
impl crate::Readable for SwevSpec {}
#[doc = "`write(|w| ..)` method takes [`swev::W`](W) writer structure"]
impl crate::Writable for SwevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWEV to value 0"]
impl crate::Resettable for SwevSpec {
    const RESET_VALUE: u32 = 0;
}
