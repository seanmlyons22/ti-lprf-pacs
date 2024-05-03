#[doc = "Register `REMAP` reader"]
pub type R = crate::R<RemapSpec>;
#[doc = "Register `REMAP` writer"]
pub type W = crate::W<RemapSpec>;
#[doc = "Field `RESERVED0` reader - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REMAP` reader - 28:5\\]
Holds the bits\\[28:5\\]
of the Flash Patch remap address"]
pub type RemapR = crate::FieldReader<u32>;
#[doc = "Field `REMAP` writer - 28:5\\]
Holds the bits\\[28:5\\]
of the Flash Patch remap address"]
pub type RemapW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RMPSPT` reader - 29:29\\]
Indicates whether the FPB unit supports the Flash Patch remap function"]
pub type RmpsptR = crate::BitReader;
#[doc = "Field `RMPSPT` writer - 29:29\\]
Indicates whether the FPB unit supports the Flash Patch remap function"]
pub type RmpsptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30R = crate::FieldReader;
#[doc = "Field `RESERVED30` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:28 - 28:5\\]
Holds the bits\\[28:5\\]
of the Flash Patch remap address"]
    #[inline(always)]
    pub fn remap(&self) -> RemapR {
        RemapR::new((self.bits >> 5) & 0x00ff_ffff)
    }
    #[doc = "Bit 29 - 29:29\\]
Indicates whether the FPB unit supports the Flash Patch remap function"]
    #[inline(always)]
    pub fn rmpspt(&self) -> RmpsptR {
        RmpsptR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> Reserved30R {
        Reserved30R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<RemapSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 5:28 - 28:5\\]
Holds the bits\\[28:5\\]
of the Flash Patch remap address"]
    #[inline(always)]
    #[must_use]
    pub fn remap(&mut self) -> RemapW<RemapSpec> {
        RemapW::new(self, 5)
    }
    #[doc = "Bit 29 - 29:29\\]
Indicates whether the FPB unit supports the Flash Patch remap function"]
    #[inline(always)]
    #[must_use]
    pub fn rmpspt(&mut self) -> RmpsptW<RemapSpec> {
        RmpsptW::new(self, 29)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved30(&mut self) -> Reserved30W<RemapSpec> {
        Reserved30W::new(self, 30)
    }
}
#[doc = "Indicates whether the implementation supports Flash Patch remap and, if it does, holds the target address for remap\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemapSpec;
impl crate::RegisterSpec for RemapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap::R`](R) reader structure"]
impl crate::Readable for RemapSpec {}
#[doc = "`write(|w| ..)` method takes [`remap::W`](W) writer structure"]
impl crate::Writable for RemapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP to value 0"]
impl crate::Resettable for RemapSpec {
    const RESET_VALUE: u32 = 0;
}
