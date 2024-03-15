#[doc = "Register `VF_OFFSET_STRIDE` reader"]
pub type R = crate::R<VfOffsetStrideSpec>;
#[doc = "Field `FVFO` reader - First VF Offset \\[FVFO\\]
Offset of First VF relative to its PF. This field can be re-written independently for each PF from the local management bus."]
pub type FvfoR = crate::FieldReader<u16>;
#[doc = "Field `VFS` reader - VF Stride \\[VFS\\]
Stride value used to assign RIDs for VFs. The stride value is hardwired to 1 for all Physical Functions."]
pub type VfsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - First VF Offset \\[FVFO\\]
Offset of First VF relative to its PF. This field can be re-written independently for each PF from the local management bus."]
    #[inline(always)]
    pub fn fvfo(&self) -> FvfoR {
        FvfoR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VF Stride \\[VFS\\]
Stride value used to assign RIDs for VFs. The stride value is hardwired to 1 for all Physical Functions."]
    #[inline(always)]
    pub fn vfs(&self) -> VfsR {
        VfsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "VF Offset/Stride Register Stride value used to assign RIDs for VFs. The stride value is hardwired to 1 for all Physical Functions.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_offset_stride::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfOffsetStrideSpec;
impl crate::RegisterSpec for VfOffsetStrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vf_offset_stride::R`](R) reader structure"]
impl crate::Readable for VfOffsetStrideSpec {}
#[doc = "`reset()` method sets VF_OFFSET_STRIDE to value 0x0001_0001"]
impl crate::Resettable for VfOffsetStrideSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
