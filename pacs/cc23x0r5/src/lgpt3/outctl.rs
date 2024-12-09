#[doc = "Register `OUTCTL` reader"]
pub type R = crate::R<OutctlSpec>;
#[doc = "Register `OUTCTL` writer"]
pub type W = crate::W<OutctlSpec>;
#[doc = "Field `CLROUT0` writer - 0:0\\]
Clear output 0. Write 1 to clear output 0."]
pub type Clrout0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETOUT0` writer - 1:1\\]
Set output 0. Write 1 to set output 0."]
pub type Setout0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLROUT1` writer - 2:2\\]
Clear output 1. Write 1 to clear output 1."]
pub type Clrout1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETOUT1` writer - 3:3\\]
Set output 1. Write 1 to set output 1."]
pub type Setout1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLROUT2` writer - 4:4\\]
Clear output 2. Write 1 to clear output 2."]
pub type Clrout2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETOUT2` writer - 5:5\\]
Set output 2. Write 1 to set output 2."]
pub type Setout2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear output 0. Write 1 to clear output 0."]
    #[inline(always)]
    #[must_use]
    pub fn clrout0(&mut self) -> Clrout0W<OutctlSpec> {
        Clrout0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set output 0. Write 1 to set output 0."]
    #[inline(always)]
    #[must_use]
    pub fn setout0(&mut self) -> Setout0W<OutctlSpec> {
        Setout0W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear output 1. Write 1 to clear output 1."]
    #[inline(always)]
    #[must_use]
    pub fn clrout1(&mut self) -> Clrout1W<OutctlSpec> {
        Clrout1W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Set output 1. Write 1 to set output 1."]
    #[inline(always)]
    #[must_use]
    pub fn setout1(&mut self) -> Setout1W<OutctlSpec> {
        Setout1W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear output 2. Write 1 to clear output 2."]
    #[inline(always)]
    #[must_use]
    pub fn clrout2(&mut self) -> Clrout2W<OutctlSpec> {
        Clrout2W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Set output 2. Write 1 to set output 2."]
    #[inline(always)]
    #[must_use]
    pub fn setout2(&mut self) -> Setout2W<OutctlSpec> {
        Setout2W::new(self, 5)
    }
}
#[doc = "Output Control Set and clear individual outputs manually. Manual update of an output takes priority over automatic channel updates to the same output. It is not possible to set and clear an output at the same time, such requests will be neglected. An output can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an output at the same time. All outputs are connected to the event fabric and the IO controller. The outputs going to the IO controller have an aditional complementary output, this output is the inverted IO output. Both the IO and the IO complementary outputs are passed through an IO Controller, see IOCTL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutctlSpec;
impl crate::RegisterSpec for OutctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outctl::R`](R) reader structure"]
impl crate::Readable for OutctlSpec {}
#[doc = "`write(|w| ..)` method takes [`outctl::W`](W) writer structure"]
impl crate::Writable for OutctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTCTL to value 0"]
impl crate::Resettable for OutctlSpec {
    const RESET_VALUE: u32 = 0;
}
