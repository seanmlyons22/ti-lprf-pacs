#[doc = "Register `CLR` reader"]
pub type R = crate::R<ClrSpec>;
#[doc = "Register `CLR` writer"]
pub type W = crate::W<ClrSpec>;
#[doc = "Field `BUF` reader - 0:0\\]
Write 1 to this field to clear BUF."]
pub type BufR = crate::BitReader;
#[doc = "Field `BUF` writer - 0:0\\]
Write 1 to this field to clear BUF."]
pub type BufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXT` reader - 1:1\\]
Write 1 to this field to clear TXT."]
pub type TxtR = crate::BitReader;
#[doc = "Field `TXT` writer - 1:1\\]
Write 1 to this field to clear TXT."]
pub type TxtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write 1 to this field to clear BUF."]
    #[inline(always)]
    pub fn buf(&self) -> BufR {
        BufR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write 1 to this field to clear TXT."]
    #[inline(always)]
    pub fn txt(&self) -> TxtR {
        TxtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write 1 to this field to clear BUF."]
    #[inline(always)]
    #[must_use]
    pub fn buf(&mut self) -> BufW<ClrSpec> {
        BufW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write 1 to this field to clear TXT."]
    #[inline(always)]
    #[must_use]
    pub fn txt(&mut self) -> TxtW<ClrSpec> {
        TxtW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<ClrSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Clear Use this register to clear contents of TXT and BUF when STA.STATE = IDLE. If condition is not met, the contents remain unchanged.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrSpec;
impl crate::RegisterSpec for ClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr::R`](R) reader structure"]
impl crate::Readable for ClrSpec {}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for ClrSpec {
    const RESET_VALUE: u32 = 0;
}
