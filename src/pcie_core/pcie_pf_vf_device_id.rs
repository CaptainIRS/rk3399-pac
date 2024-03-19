#[doc = "Register `PCIE_PF_VF_DEVICE_ID` reader"]
pub type R = crate::R<PciePfVfDeviceIdSpec>;
#[doc = "Field `R2` reader - Reserved \\[R2\\]
Reserved"]
pub type R2R = crate::FieldReader<u16>;
#[doc = "Field `VFDI` reader - VF Device ID \\[VFDI\\]
VF device id assigned to the device. Its default value is specified in reg_defaults.h, but can be re- written independently for each PF from the local management bus."]
pub type VfdiR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Reserved \\[R2\\]
Reserved"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VF Device ID \\[VFDI\\]
VF device id assigned to the device. Its default value is specified in reg_defaults.h, but can be re- written independently for each PF from the local management bus."]
    #[inline(always)]
    pub fn vfdi(&self) -> VfdiR {
        VfdiR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "VF Device ID Register VF device id assigned to the device. Its default value is specified in reg_defaults.h, but can be re- written independently for each PF from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vf_device_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfVfDeviceIdSpec;
impl crate::RegisterSpec for PciePfVfDeviceIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_vf_device_id::R`](R) reader structure"]
impl crate::Readable for PciePfVfDeviceIdSpec {}
#[doc = "`reset()` method sets PCIE_PF_VF_DEVICE_ID to value 0x0100_0000"]
impl crate::Resettable for PciePfVfDeviceIdSpec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
