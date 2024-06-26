#[doc = "Register `GRXFIFOSIZ%s` reader"]
pub type R = crate::R<GrxfifosizSpec>;
#[doc = "Register `GRXFIFOSIZ%s` writer"]
pub type W = crate::W<GrxfifosizSpec>;
#[doc = "Field `RXFDEP_N` reader - RxFIFO Depth\n\nThis field contains the depth of RxFIFOn in 64-bit words.\n\nMinimum value: 32; Maximum value: 16,384"]
pub type RxfdepNR = crate::FieldReader<u16>;
#[doc = "Field `RXFDEP_N` writer - RxFIFO Depth\n\nThis field contains the depth of RxFIFOn in 64-bit words.\n\nMinimum value: 32; Maximum value: 16,384"]
pub type RxfdepNW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RXFSTADDR_N` reader - RxFIFOn RAM Start Address\n\nThis field contains the memory start address for RxFIFOn in 64-\n\nbit words."]
pub type RxfstaddrNR = crate::FieldReader<u16>;
#[doc = "Field `RXFSTADDR_N` writer - RxFIFOn RAM Start Address\n\nThis field contains the memory start address for RxFIFOn in 64-\n\nbit words."]
pub type RxfstaddrNW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RxFIFO Depth\n\nThis field contains the depth of RxFIFOn in 64-bit words.\n\nMinimum value: 32; Maximum value: 16,384"]
    #[inline(always)]
    pub fn rxfdep_n(&self) -> RxfdepNR {
        RxfdepNR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - RxFIFOn RAM Start Address\n\nThis field contains the memory start address for RxFIFOn in 64-\n\nbit words."]
    #[inline(always)]
    pub fn rxfstaddr_n(&self) -> RxfstaddrNR {
        RxfstaddrNR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RxFIFO Depth\n\nThis field contains the depth of RxFIFOn in 64-bit words.\n\nMinimum value: 32; Maximum value: 16,384"]
    #[inline(always)]
    #[must_use]
    pub fn rxfdep_n(&mut self) -> RxfdepNW<GrxfifosizSpec> {
        RxfdepNW::new(self, 0)
    }
    #[doc = "Bits 16:31 - RxFIFOn RAM Start Address\n\nThis field contains the memory start address for RxFIFOn in 64-\n\nbit words."]
    #[inline(always)]
    #[must_use]
    pub fn rxfstaddr_n(&mut self) -> RxfstaddrNW<GrxfifosizSpec> {
        RxfstaddrNW::new(self, 16)
    }
}
#[doc = "Global Receive FIFO Size Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfifosiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfifosiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrxfifosizSpec;
impl crate::RegisterSpec for GrxfifosizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxfifosiz::R`](R) reader structure"]
impl crate::Readable for GrxfifosizSpec {}
#[doc = "`write(|w| ..)` method takes [`grxfifosiz::W`](W) writer structure"]
impl crate::Writable for GrxfifosizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRXFIFOSIZ%s to value 0x0285"]
impl crate::Resettable for GrxfifosizSpec {
    const RESET_VALUE: u32 = 0x0285;
}
