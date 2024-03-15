#[doc = "Register `ARI_CAPABILITY_AND_ARI_CONTROL` reader"]
pub type R = crate::R<AriCapabilityAndAriControlSpec>;
#[doc = "Field `R13` reader - Reserved \\[R13\\]
Reserved"]
pub type R13R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved \\[R13\\]
Reserved"]
    #[inline(always)]
    pub fn r13(&self) -> R13R {
        R13R::new(self.bits)
    }
}
#[doc = "ARI Capability Register and ARI Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ari_capability_and_ari_control::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AriCapabilityAndAriControlSpec;
impl crate::RegisterSpec for AriCapabilityAndAriControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ari_capability_and_ari_control::R`](R) reader structure"]
impl crate::Readable for AriCapabilityAndAriControlSpec {}
#[doc = "`reset()` method sets ARI_CAPABILITY_AND_ARI_CONTROL to value 0"]
impl crate::Resettable for AriCapabilityAndAriControlSpec {
    const RESET_VALUE: u32 = 0;
}
