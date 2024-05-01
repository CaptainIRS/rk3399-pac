#[doc = "Register `SWREG72` reader"]
pub type R = crate::R<Swreg72Spec>;
#[doc = "Field `DEBUG_SERVICE` reader - debug_service signals\n\nthis value\\[6:0\\]=service_wr\\[2:0\\], service_rd\\[3:0\\]"]
pub type DebugServiceR = crate::FieldReader;
impl R {
    #[doc = "Bits 24:30 - debug_service signals\n\nthis value\\[6:0\\]=service_wr\\[2:0\\], service_rd\\[3:0\\]"]
    #[inline(always)]
    pub fn debug_service(&self) -> DebugServiceR {
        DebugServiceR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "debug0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg72::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg72Spec;
impl crate::RegisterSpec for Swreg72Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg72::R`](R) reader structure"]
impl crate::Readable for Swreg72Spec {}
#[doc = "`reset()` method sets SWREG72 to value 0"]
impl crate::Resettable for Swreg72Spec {
    const RESET_VALUE: u32 = 0;
}
