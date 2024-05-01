#[doc = "Register `AFBCD0_HDR_PTR` reader"]
pub type R = crate::R<Afbcd0HdrPtrSpec>;
#[doc = "Register `AFBCD0_HDR_PTR` writer"]
pub type W = crate::W<Afbcd0HdrPtrSpec>;
#[doc = "Field `AFBCD_HREG_HDR_PTR` reader - afbcd_hreg_hdr_ptr"]
pub type AfbcdHregHdrPtrR = crate::FieldReader<u32>;
#[doc = "Field `AFBCD_HREG_HDR_PTR` writer - afbcd_hreg_hdr_ptr"]
pub type AfbcdHregHdrPtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - afbcd_hreg_hdr_ptr"]
    #[inline(always)]
    pub fn afbcd_hreg_hdr_ptr(&self) -> AfbcdHregHdrPtrR {
        AfbcdHregHdrPtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - afbcd_hreg_hdr_ptr"]
    #[inline(always)]
    #[must_use]
    pub fn afbcd_hreg_hdr_ptr(&mut self) -> AfbcdHregHdrPtrW<Afbcd0HdrPtrSpec> {
        AfbcdHregHdrPtrW::new(self, 0)
    }
}
#[doc = "AFBCD0 memory start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afbcd0_hdr_ptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afbcd0_hdr_ptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afbcd0HdrPtrSpec;
impl crate::RegisterSpec for Afbcd0HdrPtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afbcd0_hdr_ptr::R`](R) reader structure"]
impl crate::Readable for Afbcd0HdrPtrSpec {}
#[doc = "`write(|w| ..)` method takes [`afbcd0_hdr_ptr::W`](W) writer structure"]
impl crate::Writable for Afbcd0HdrPtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFBCD0_HDR_PTR to value 0"]
impl crate::Resettable for Afbcd0HdrPtrSpec {
    const RESET_VALUE: u32 = 0;
}
