#[doc = "Register `HEADER_LOG_1` reader"]
pub type R = crate::R<HeaderLog1Spec>;
#[doc = "Field `HD1` reader - Header Dword 1 \\[HD1\\]
Second Dword of captured TLP header. STICKY."]
pub type Hd1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Header Dword 1 \\[HD1\\]
Second Dword of captured TLP header. STICKY."]
    #[inline(always)]
    pub fn hd1(&self) -> Hd1R {
        Hd1R::new(self.bits)
    }
}
#[doc = "Header Log Register 1 Second Dword of captured TLP header. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`header_log_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeaderLog1Spec;
impl crate::RegisterSpec for HeaderLog1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`header_log_1::R`](R) reader structure"]
impl crate::Readable for HeaderLog1Spec {}
#[doc = "`reset()` method sets HEADER_LOG_1 to value 0"]
impl crate::Resettable for HeaderLog1Spec {
    const RESET_VALUE: u32 = 0;
}
