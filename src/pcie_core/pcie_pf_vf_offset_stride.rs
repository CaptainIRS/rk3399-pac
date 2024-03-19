#[doc = "Register `PCIE_PF_VF_OFFSET_STRIDE` reader"]
pub type R = crate::R<PciePfVfOffsetStrideSpec>;
#[doc = "Field `FVFO` reader - First VF Offset \\[FVFO\\]\n\nOffset of First VF relative to its PF.\n\nThis field can be re-written\n\nindependently for each PF from the\n\nlocal management bus."]
pub type FvfoR = crate::FieldReader<u16>;
#[doc = "Field `VFS` reader - VF Stride \\[VFS\\]\n\nStride value used to assign RIDs for\n\nVFs. The stride value is hardwired to\n\n1 for all Physical Functions."]
pub type VfsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - First VF Offset \\[FVFO\\]\n\nOffset of First VF relative to its PF.\n\nThis field can be re-written\n\nindependently for each PF from the\n\nlocal management bus."]
    #[inline(always)]
    pub fn fvfo(&self) -> FvfoR {
        FvfoR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VF Stride \\[VFS\\]\n\nStride value used to assign RIDs for\n\nVFs. The stride value is hardwired to\n\n1 for all Physical Functions."]
    #[inline(always)]
    pub fn vfs(&self) -> VfsR {
        VfsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "VF Offset/Stride Register\n\nStride value used to assign RIDs for\n\nVFs. The stride value is hardwired to\n\n1 for all Physical Functions.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vf_offset_stride::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfVfOffsetStrideSpec;
impl crate::RegisterSpec for PciePfVfOffsetStrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_vf_offset_stride::R`](R) reader structure"]
impl crate::Readable for PciePfVfOffsetStrideSpec {}
#[doc = "`reset()` method sets PCIE_PF_VF_OFFSET_STRIDE to value 0x0001_0001"]
impl crate::Resettable for PciePfVfOffsetStrideSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
