#[doc = "Register `HEADER_LOG_0` reader"]
pub type R = crate::R<HeaderLog0Spec>;
#[doc = "Field `HD0` reader - Header Dword 0 \\[HD0\\]
First Dword of captured TLP header. STICKY."]
pub type Hd0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Header Dword 0 \\[HD0\\]
First Dword of captured TLP header. STICKY."]
    #[inline(always)]
    pub fn hd0(&self) -> Hd0R {
        Hd0R::new(self.bits)
    }
}
#[doc = "Header Log Register 0 First Dword of captured TLP header. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`header_log_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeaderLog0Spec;
impl crate::RegisterSpec for HeaderLog0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`header_log_0::R`](R) reader structure"]
impl crate::Readable for HeaderLog0Spec {}
#[doc = "`reset()` method sets HEADER_LOG_0 to value 0"]
impl crate::Resettable for HeaderLog0Spec {
    const RESET_VALUE: u32 = 0;
}
