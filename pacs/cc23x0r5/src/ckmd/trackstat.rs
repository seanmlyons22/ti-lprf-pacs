#[doc = "Register `TRACKSTAT` reader"]
pub type R = crate::R<TrackstatSpec>;
#[doc = "Register `TRACKSTAT` writer"]
pub type W = crate::W<TrackstatSpec>;
#[doc = "Field `FINETRIM` reader - 12:0\\]
Current HFOSC Fine-trim value This field uses the internal fractional representation (sign, 4 integer bits, 8 fractional bits). The actual trim value applied to the oscillator is delta-sigma modulated 5 bits non-signed (inverted sign bit + integer bits)."]
pub type FinetrimR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED13` reader - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13R = crate::FieldReader;
#[doc = "Field `LOOPERR` reader - 29:16\\]
Current HFOSC tracking error"]
pub type LooperrR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED30` reader - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30R = crate::BitReader;
#[doc = "Field `LOOPERRVLD` reader - 31:31\\]
Current HFOSC tracking error valid This bit is one if the tracking loop is running and the error value is valid."]
pub type LooperrvldR = crate::BitReader;
impl R {
    #[doc = "Bits 0:12 - 12:0\\]
Current HFOSC Fine-trim value This field uses the internal fractional representation (sign, 4 integer bits, 8 fractional bits). The actual trim value applied to the oscillator is delta-sigma modulated 5 bits non-signed (inverted sign bit + integer bits)."]
    #[inline(always)]
    pub fn finetrim(&self) -> FinetrimR {
        FinetrimR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:29 - 29:16\\]
Current HFOSC tracking error"]
    #[inline(always)]
    pub fn looperr(&self) -> LooperrR {
        LooperrR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> Reserved30R {
        Reserved30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Current HFOSC tracking error valid This bit is one if the tracking loop is running and the error value is valid."]
    #[inline(always)]
    pub fn looperrvld(&self) -> LooperrvldR {
        LooperrvldR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "HFOSC tracking loop status information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trackstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trackstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrackstatSpec;
impl crate::RegisterSpec for TrackstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trackstat::R`](R) reader structure"]
impl crate::Readable for TrackstatSpec {}
#[doc = "`write(|w| ..)` method takes [`trackstat::W`](W) writer structure"]
impl crate::Writable for TrackstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRACKSTAT to value 0"]
impl crate::Resettable for TrackstatSpec {
    const RESET_VALUE: u32 = 0;
}
