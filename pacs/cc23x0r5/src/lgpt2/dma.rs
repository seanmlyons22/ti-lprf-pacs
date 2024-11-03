#[doc = "Register `DMA` reader"]
pub type R = crate::R<DmaSpec>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DmaSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Req {
    #[doc = "15: Setting of RIS.C11CC generates a DMA request."]
    C11cc = 15,
    #[doc = "14: Setting of RIS.C10CC generates a DMA request."]
    C10cc = 14,
    #[doc = "13: Setting of RIS.C9CC generates a DMA request."]
    C9cc = 13,
    #[doc = "12: Setting of RIS.C8CC generates a DMA request."]
    C8cc = 12,
    #[doc = "11: Setting of RIS.C7CC generates a DMA request."]
    C7cc = 11,
    #[doc = "10: Setting of RIS.C6CC generates a DMA request."]
    C6cc = 10,
    #[doc = "9: Setting of RIS.C5CC generates a DMA request."]
    C5cc = 9,
    #[doc = "8: Setting of RIS.C4CC generates a DMA request."]
    C4cc = 8,
    #[doc = "7: Setting of RIS.C3CC generates a DMA request."]
    C3cc = 7,
    #[doc = "6: Setting of RIS.C2CC generates a DMA request."]
    C2cc = 6,
    #[doc = "5: Setting of RIS.C1CC generates a DMA request."]
    C1cc = 5,
    #[doc = "4: Setting of RIS.C0CC generates a DMA request."]
    C0cc = 4,
    #[doc = "3: Setting of RIS.FAULT generates a DMA request."]
    Fault = 3,
    #[doc = "2: Setting of RIS.ZERO generates a DMA request."]
    Zero = 2,
    #[doc = "1: Setting of RIS.TGT generates a DMA request."]
    Tgt = 1,
    #[doc = "0: Disabled"]
    Dis = 0,
}
impl From<Req> for u8 {
    #[inline(always)]
    fn from(variant: Req) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Req {
    type Ux = u8;
}
impl crate::IsEnum for Req {}
#[doc = "Field `REQ` reader - "]
pub type ReqR = crate::FieldReader<Req>;
impl ReqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Req {
        match self.bits {
            15 => Req::C11cc,
            14 => Req::C10cc,
            13 => Req::C9cc,
            12 => Req::C8cc,
            11 => Req::C7cc,
            10 => Req::C6cc,
            9 => Req::C5cc,
            8 => Req::C4cc,
            7 => Req::C3cc,
            6 => Req::C2cc,
            5 => Req::C1cc,
            4 => Req::C0cc,
            3 => Req::Fault,
            2 => Req::Zero,
            1 => Req::Tgt,
            0 => Req::Dis,
            _ => unreachable!(),
        }
    }
    #[doc = "Setting of RIS.C11CC generates a DMA request."]
    #[inline(always)]
    pub fn is_c11cc(&self) -> bool {
        *self == Req::C11cc
    }
    #[doc = "Setting of RIS.C10CC generates a DMA request."]
    #[inline(always)]
    pub fn is_c10cc(&self) -> bool {
        *self == Req::C10cc
    }
    #[doc = "Setting of RIS.C9CC generates a DMA request."]
    #[inline(always)]
    pub fn is_c9cc(&self) -> bool {
        *self == Req::C9cc
    }
    #[doc = "Setting of RIS.C8CC generates a DMA request."]
    #[inline(always)]
    pub fn is_c8cc(&self) -> bool {
        *self == Req::C8cc
    }
    #[doc = "Setting of RIS.C7CC generates a DMA request."]
    #[inline(always)]
    pub fn is_c7cc(&self) -> bool {
        *self == Req::C7cc
    }
    #[doc = "Setting of RIS.C6CC generates a DMA request."]
    #[inline(always)]
    pub fn is_c6cc(&self) -> bool {
        *self == Req::C6cc
    }
    #[doc = "Setting of RIS.C5CC generates a DMA request."]
    #[inline(always)]
    pub fn is_c5cc(&self) -> bool {
        *self == Req::C5cc
    }
    #[doc = "Setting of RIS.C4CC generates a DMA request."]
    #[inline(always)]
    pub fn is_c4cc(&self) -> bool {
        *self == Req::C4cc
    }
    #[doc = "Setting of RIS.C3CC generates a DMA request."]
    #[inline(always)]
    pub fn is_c3cc(&self) -> bool {
        *self == Req::C3cc
    }
    #[doc = "Setting of RIS.C2CC generates a DMA request."]
    #[inline(always)]
    pub fn is_c2cc(&self) -> bool {
        *self == Req::C2cc
    }
    #[doc = "Setting of RIS.C1CC generates a DMA request."]
    #[inline(always)]
    pub fn is_c1cc(&self) -> bool {
        *self == Req::C1cc
    }
    #[doc = "Setting of RIS.C0CC generates a DMA request."]
    #[inline(always)]
    pub fn is_c0cc(&self) -> bool {
        *self == Req::C0cc
    }
    #[doc = "Setting of RIS.FAULT generates a DMA request."]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == Req::Fault
    }
    #[doc = "Setting of RIS.ZERO generates a DMA request."]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Req::Zero
    }
    #[doc = "Setting of RIS.TGT generates a DMA request."]
    #[inline(always)]
    pub fn is_tgt(&self) -> bool {
        *self == Req::Tgt
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Req::Dis
    }
}
#[doc = "Field `REQ` writer - "]
pub type ReqW<'a, REG> = crate::FieldWriter<'a, REG, 4, Req, crate::Safe>;
impl<'a, REG> ReqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Setting of RIS.C11CC generates a DMA request."]
    #[inline(always)]
    pub fn c11cc(self) -> &'a mut crate::W<REG> {
        self.variant(Req::C11cc)
    }
    #[doc = "Setting of RIS.C10CC generates a DMA request."]
    #[inline(always)]
    pub fn c10cc(self) -> &'a mut crate::W<REG> {
        self.variant(Req::C10cc)
    }
    #[doc = "Setting of RIS.C9CC generates a DMA request."]
    #[inline(always)]
    pub fn c9cc(self) -> &'a mut crate::W<REG> {
        self.variant(Req::C9cc)
    }
    #[doc = "Setting of RIS.C8CC generates a DMA request."]
    #[inline(always)]
    pub fn c8cc(self) -> &'a mut crate::W<REG> {
        self.variant(Req::C8cc)
    }
    #[doc = "Setting of RIS.C7CC generates a DMA request."]
    #[inline(always)]
    pub fn c7cc(self) -> &'a mut crate::W<REG> {
        self.variant(Req::C7cc)
    }
    #[doc = "Setting of RIS.C6CC generates a DMA request."]
    #[inline(always)]
    pub fn c6cc(self) -> &'a mut crate::W<REG> {
        self.variant(Req::C6cc)
    }
    #[doc = "Setting of RIS.C5CC generates a DMA request."]
    #[inline(always)]
    pub fn c5cc(self) -> &'a mut crate::W<REG> {
        self.variant(Req::C5cc)
    }
    #[doc = "Setting of RIS.C4CC generates a DMA request."]
    #[inline(always)]
    pub fn c4cc(self) -> &'a mut crate::W<REG> {
        self.variant(Req::C4cc)
    }
    #[doc = "Setting of RIS.C3CC generates a DMA request."]
    #[inline(always)]
    pub fn c3cc(self) -> &'a mut crate::W<REG> {
        self.variant(Req::C3cc)
    }
    #[doc = "Setting of RIS.C2CC generates a DMA request."]
    #[inline(always)]
    pub fn c2cc(self) -> &'a mut crate::W<REG> {
        self.variant(Req::C2cc)
    }
    #[doc = "Setting of RIS.C1CC generates a DMA request."]
    #[inline(always)]
    pub fn c1cc(self) -> &'a mut crate::W<REG> {
        self.variant(Req::C1cc)
    }
    #[doc = "Setting of RIS.C0CC generates a DMA request."]
    #[inline(always)]
    pub fn c0cc(self) -> &'a mut crate::W<REG> {
        self.variant(Req::C0cc)
    }
    #[doc = "Setting of RIS.FAULT generates a DMA request."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut crate::W<REG> {
        self.variant(Req::Fault)
    }
    #[doc = "Setting of RIS.ZERO generates a DMA request."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Req::Zero)
    }
    #[doc = "Setting of RIS.TGT generates a DMA request."]
    #[inline(always)]
    pub fn tgt(self) -> &'a mut crate::W<REG> {
        self.variant(Req::Tgt)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Req::Dis)
    }
}
#[doc = "Field `RESERVED4` reader - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RWADDR` reader - 14:8\\]
The base address which the DMA access when reading/writing DMARW. The base address is set by taking the 9 LSB of the physical address and divide by 4. For example, if you wanted the RWADDR to point to the PTGT register you should set RWADDR = 0x0FC/4."]
pub type RwaddrR = crate::FieldReader;
#[doc = "Field `RWADDR` writer - 14:8\\]
The base address which the DMA access when reading/writing DMARW. The base address is set by taking the 9 LSB of the physical address and divide by 4. For example, if you wanted the RWADDR to point to the PTGT register you should set RWADDR = 0x0FC/4."]
pub type RwaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESERVED15` reader - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `RESERVED15` writer - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWCNTR` reader - 19:16\\]
The read/write counter. RWCNTR+1 is the number of times the DMA can access (read/write) the DMARW register. For each DMA access to DMARW an internal counter is incremented, writing to the next address field. RWADDR + 4*RWCNTR is the final register address which can be accessed by the DMA."]
pub type RwcntrR = crate::FieldReader;
#[doc = "Field `RWCNTR` writer - 19:16\\]
The read/write counter. RWCNTR+1 is the number of times the DMA can access (read/write) the DMARW register. For each DMA access to DMARW an internal counter is incremented, writing to the next address field. RWADDR + 4*RWCNTR is the final register address which can be accessed by the DMA."]
pub type RwcntrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED20` writer - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn req(&self) -> ReqR {
        ReqR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
The base address which the DMA access when reading/writing DMARW. The base address is set by taking the 9 LSB of the physical address and divide by 4. For example, if you wanted the RWADDR to point to the PTGT register you should set RWADDR = 0x0FC/4."]
    #[inline(always)]
    pub fn rwaddr(&self) -> RwaddrR {
        RwaddrR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The read/write counter. RWCNTR+1 is the number of times the DMA can access (read/write) the DMARW register. For each DMA access to DMARW an internal counter is incremented, writing to the next address field. RWADDR + 4*RWCNTR is the final register address which can be accessed by the DMA."]
    #[inline(always)]
    pub fn rwcntr(&self) -> RwcntrR {
        RwcntrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> ReqW<DmaSpec> {
        ReqW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<DmaSpec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bits 8:14 - 14:8\\]
The base address which the DMA access when reading/writing DMARW. The base address is set by taking the 9 LSB of the physical address and divide by 4. For example, if you wanted the RWADDR to point to the PTGT register you should set RWADDR = 0x0FC/4."]
    #[inline(always)]
    #[must_use]
    pub fn rwaddr(&mut self) -> RwaddrW<DmaSpec> {
        RwaddrW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<DmaSpec> {
        Reserved15W::new(self, 15)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The read/write counter. RWCNTR+1 is the number of times the DMA can access (read/write) the DMARW register. For each DMA access to DMARW an internal counter is incremented, writing to the next address field. RWADDR + 4*RWCNTR is the final register address which can be accessed by the DMA."]
    #[inline(always)]
    #[must_use]
    pub fn rwcntr(&mut self) -> RwcntrW<DmaSpec> {
        RwcntrW::new(self, 16)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> Reserved20W<DmaSpec> {
        Reserved20W::new(self, 20)
    }
}
#[doc = "Direct Memory Accsess This register is used to enable DMA requests from the timer and set the register addresses which the DMA will access (read/write). Choose DMA request source by setting the REQ field. The setting of the corresponding interrupt in the RIS registers also sets the DMA request. Upon a DMA request defined by REQ an internal address pointer is set to RWADDR*4. Every access to DMARW will increment the internal pointer by 4 such that the next DMA access will be to the next register. The internal pointer will stop after RWCNTR increments. Further access will be ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSpec;
impl crate::RegisterSpec for DmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DmaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DmaSpec {
    const RESET_VALUE: u32 = 0;
}
