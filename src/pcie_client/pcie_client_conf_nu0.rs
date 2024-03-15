#[doc = "Register `PCIE_CLIENT_CONF_NU0` reader"]
pub type R = crate::R<PcieClientConfNu0Spec>;
#[doc = "Configuration no used\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_conf_nu0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientConfNu0Spec;
impl crate::RegisterSpec for PcieClientConfNu0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_conf_nu0::R`](R) reader structure"]
impl crate::Readable for PcieClientConfNu0Spec {}
#[doc = "`reset()` method sets PCIE_CLIENT_CONF_NU0 to value 0"]
impl crate::Resettable for PcieClientConfNu0Spec {
    const RESET_VALUE: u32 = 0;
}
