#[doc = "Register `DTBCTL` reader"]
pub type R = crate::R<DtbctlSpec>;
#[doc = "Register `DTBCTL` writer"]
pub type W = crate::W<DtbctlSpec>;
#[doc = "Field `EN` reader - 0:0\\]
Enable DTB output"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Enable DTB output"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `CLKSEL` reader - 7:4\\]
Select clock to output on DTB\\[0\\]"]
pub type ClkselR = crate::FieldReader;
#[doc = "Field `CLKSEL` writer - 7:4\\]
Select clock to output on DTB\\[0\\]"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DSEL0` reader - 12:8\\]
Select data to output on DTB\\[5:1\\]"]
pub type Dsel0R = crate::FieldReader;
#[doc = "Field `DSEL0` writer - 12:8\\]
Select data to output on DTB\\[5:1\\]"]
pub type Dsel0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DSEL1` reader - 17:13\\]
Select data to output on DTB\\[10:6\\]"]
pub type Dsel1R = crate::FieldReader;
#[doc = "Field `DSEL1` writer - 17:13\\]
Select data to output on DTB\\[10:6\\]"]
pub type Dsel1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DSEL2` reader - 22:18\\]
Select data to output on DTB\\[15:11\\]"]
pub type Dsel2R = crate::FieldReader;
#[doc = "Field `DSEL2` writer - 22:18\\]
Select data to output on DTB\\[15:11\\]"]
pub type Dsel2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED23` reader - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved23R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable DTB output"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Select clock to output on DTB\\[0\\]"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select data to output on DTB\\[5:1\\]"]
    #[inline(always)]
    pub fn dsel0(&self) -> Dsel0R {
        Dsel0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:17 - 17:13\\]
Select data to output on DTB\\[10:6\\]"]
    #[inline(always)]
    pub fn dsel1(&self) -> Dsel1R {
        Dsel1R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - 22:18\\]
Select data to output on DTB\\[15:11\\]"]
    #[inline(always)]
    pub fn dsel2(&self) -> Dsel2R {
        Dsel2R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> Reserved23R {
        Reserved23R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable DTB output"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<DtbctlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Select clock to output on DTB\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<DtbctlSpec> {
        ClkselW::new(self, 4)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select data to output on DTB\\[5:1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dsel0(&mut self) -> Dsel0W<DtbctlSpec> {
        Dsel0W::new(self, 8)
    }
    #[doc = "Bits 13:17 - 17:13\\]
Select data to output on DTB\\[10:6\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dsel1(&mut self) -> Dsel1W<DtbctlSpec> {
        Dsel1W::new(self, 13)
    }
    #[doc = "Bits 18:22 - 22:18\\]
Select data to output on DTB\\[15:11\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dsel2(&mut self) -> Dsel2W<DtbctlSpec> {
        Dsel2W::new(self, 18)
    }
}
#[doc = "Digital test bus mux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtbctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtbctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtbctlSpec;
impl crate::RegisterSpec for DtbctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtbctl::R`](R) reader structure"]
impl crate::Readable for DtbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dtbctl::W`](W) writer structure"]
impl crate::Writable for DtbctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTBCTL to value 0"]
impl crate::Resettable for DtbctlSpec {
    const RESET_VALUE: u32 = 0;
}
