#[doc = "Register `PCIE_VF_ARI_CAPABILITY_AND_ARI_CONTROL` reader"]
pub type R = crate::R<PcieVfAriCapabilityAndAriControlSpec>;
#[doc = "Field `R13` reader - Reserved \\[R13\\]\n\nReserved"]
pub type R13R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved \\[R13\\]\n\nReserved"]
    #[inline(always)]
    pub fn r13(&self) -> R13R {
        R13R::new(self.bits)
    }
}
#[doc = "ARI Capability Register and ARI Control Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_ari_capability_and_ari_control::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfAriCapabilityAndAriControlSpec;
impl crate::RegisterSpec for PcieVfAriCapabilityAndAriControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_ari_capability_and_ari_control::R`](R) reader structure"]
impl crate::Readable for PcieVfAriCapabilityAndAriControlSpec {}
#[doc = "`reset()` method sets PCIE_VF_ARI_CAPABILITY_AND_ARI_CONTROL to value 0"]
impl crate::Resettable for PcieVfAriCapabilityAndAriControlSpec {
    const RESET_VALUE: u32 = 0;
}
