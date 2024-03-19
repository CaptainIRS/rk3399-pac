#[doc = "Register `PCIE_PF_INITIAL_VFS_TOTAL_VFS` reader"]
pub type R = crate::R<PciePfInitialVfsTotalVfsSpec>;
#[doc = "Field `IVF` reader - Initial VFs \\[IVF\\]
This field contains the initial number of VFs configured for each PF. This field can be modified using local management registers."]
pub type IvfR = crate::FieldReader<u16>;
#[doc = "Field `TVF` reader - Total VFs \\[TVF\\]
This field contains the total number of VFs per PF. Its default setting is identical to that of InitialVFs. This field can be modified using local management registers."]
pub type TvfR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Initial VFs \\[IVF\\]
This field contains the initial number of VFs configured for each PF. This field can be modified using local management registers."]
    #[inline(always)]
    pub fn ivf(&self) -> IvfR {
        IvfR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Total VFs \\[TVF\\]
This field contains the total number of VFs per PF. Its default setting is identical to that of InitialVFs. This field can be modified using local management registers."]
    #[inline(always)]
    pub fn tvf(&self) -> TvfR {
        TvfR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Initial VFs/Total VFs Register This field contains the total number of VFs per PF. Its default setting is identical to that of InitialVFs. This field can be modified using local management registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_initial_vfs_total_vfs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfInitialVfsTotalVfsSpec;
impl crate::RegisterSpec for PciePfInitialVfsTotalVfsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_initial_vfs_total_vfs::R`](R) reader structure"]
impl crate::Readable for PciePfInitialVfsTotalVfsSpec {}
#[doc = "`reset()` method sets PCIE_PF_INITIAL_VFS_TOTAL_VFS to value 0x0008_0008"]
impl crate::Resettable for PciePfInitialVfsTotalVfsSpec {
    const RESET_VALUE: u32 = 0x0008_0008;
}
