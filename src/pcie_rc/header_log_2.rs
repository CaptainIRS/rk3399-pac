#[doc = "Register `HEADER_LOG_2` reader"]
pub type R = crate::R<HeaderLog2Spec>;
#[doc = "Field `HD2` reader - Header Dword 2 \\[HD2\\]
Third Dword of captured TLP header. STICKY."]
pub type Hd2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Header Dword 2 \\[HD2\\]
Third Dword of captured TLP header. STICKY."]
    #[inline(always)]
    pub fn hd2(&self) -> Hd2R {
        Hd2R::new(self.bits)
    }
}
#[doc = "Header Log Register 2 Third Dword of captured TLP header. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`header_log_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeaderLog2Spec;
impl crate::RegisterSpec for HeaderLog2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`header_log_2::R`](R) reader structure"]
impl crate::Readable for HeaderLog2Spec {}
#[doc = "`reset()` method sets HEADER_LOG_2 to value 0"]
impl crate::Resettable for HeaderLog2Spec {
    const RESET_VALUE: u32 = 0;
}
