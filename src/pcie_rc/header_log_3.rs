#[doc = "Register `HEADER_LOG_3` reader"]
pub type R = crate::R<HeaderLog3Spec>;
#[doc = "Field `HD3` reader - Header Dword 3 \\[HD3\\]
Fourth Dword of captured TLP header. STICKY."]
pub type Hd3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Header Dword 3 \\[HD3\\]
Fourth Dword of captured TLP header. STICKY."]
    #[inline(always)]
    pub fn hd3(&self) -> Hd3R {
        Hd3R::new(self.bits)
    }
}
#[doc = "Header Log Register 3 Fourth Dword of captured TLP header. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`header_log_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeaderLog3Spec;
impl crate::RegisterSpec for HeaderLog3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`header_log_3::R`](R) reader structure"]
impl crate::Readable for HeaderLog3Spec {}
#[doc = "`reset()` method sets HEADER_LOG_3 to value 0"]
impl crate::Resettable for HeaderLog3Spec {
    const RESET_VALUE: u32 = 0;
}
