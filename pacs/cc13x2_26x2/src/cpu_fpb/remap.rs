#[doc = "Register `REMAP` reader"]
pub type R = crate::R<RemapSpec>;
#[doc = "Register `REMAP` writer"]
pub type W = crate::W<RemapSpec>;
#[doc = "Field `RESERVED0` reader - 4:0\\]
This field always reads 0. Writing to this field is ignored."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 4:0\\]
This field always reads 0. Writing to this field is ignored."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REMAP` reader - 28:5\\]
Remap base address field."]
pub type RemapR = crate::FieldReader<u32>;
#[doc = "Field `REMAP` writer - 28:5\\]
Remap base address field."]
pub type RemapW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESERVED29` reader - 31:29\\]
This field always reads 3'b001. Writing to this field is ignored."]
pub type Reserved29R = crate::FieldReader;
#[doc = "Field `RESERVED29` writer - 31:29\\]
This field always reads 3'b001. Writing to this field is ignored."]
pub type Reserved29W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
This field always reads 0. Writing to this field is ignored."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:28 - 28:5\\]
Remap base address field."]
    #[inline(always)]
    pub fn remap(&self) -> RemapR {
        RemapR::new((self.bits >> 5) & 0x00ff_ffff)
    }
    #[doc = "Bits 29:31 - 31:29\\]
This field always reads 3'b001. Writing to this field is ignored."]
    #[inline(always)]
    pub fn reserved29(&self) -> Reserved29R {
        Reserved29R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
This field always reads 0. Writing to this field is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<RemapSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 5:28 - 28:5\\]
Remap base address field."]
    #[inline(always)]
    #[must_use]
    pub fn remap(&mut self) -> RemapW<RemapSpec> {
        RemapW::new(self, 5)
    }
    #[doc = "Bits 29:31 - 31:29\\]
This field always reads 3'b001. Writing to this field is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved29(&mut self) -> Reserved29W<RemapSpec> {
        Reserved29W::new(self, 29)
    }
}
#[doc = "Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets REMAP to value 0x2000_0000"]
impl crate::Resettable for RemapSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
