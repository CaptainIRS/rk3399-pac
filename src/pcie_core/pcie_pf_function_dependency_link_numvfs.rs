#[doc = "Register `PCIE_PF_FUNCTION_DEPENDENCY_LINK_NUMVFS` reader"]
pub type R = crate::R<PciePfFunctionDependencyLinkNumvfsSpec>;
#[doc = "Register `PCIE_PF_FUNCTION_DEPENDENCY_LINK_NUMVFS` writer"]
pub type W = crate::W<PciePfFunctionDependencyLinkNumvfsSpec>;
#[doc = "Field `NVF` reader - NumVFs \\[NVF\\]
This field must be set by the software to the number of VFs that it wants to enable for each PF. This field can be changed only when the VF Enable bit in the SR-IOV Control Register is 0. Its value should not exceed the setting of TotalVFs for the corresponding Physical Function. This field can also be written from the local management bus."]
pub type NvfR = crate::FieldReader<u16>;
#[doc = "Field `NVF` writer - NumVFs \\[NVF\\]
This field must be set by the software to the number of VFs that it wants to enable for each PF. This field can be changed only when the VF Enable bit in the SR-IOV Control Register is 0. Its value should not exceed the setting of TotalVFs for the corresponding Physical Function. This field can also be written from the local management bus."]
pub type NvfW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FDL` reader - Function Dependency Link \\[FDL\\]
This field is used to specify dependencies between PFs. It can be modified independently for each Function from the local management bus."]
pub type FdlR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - NumVFs \\[NVF\\]
This field must be set by the software to the number of VFs that it wants to enable for each PF. This field can be changed only when the VF Enable bit in the SR-IOV Control Register is 0. Its value should not exceed the setting of TotalVFs for the corresponding Physical Function. This field can also be written from the local management bus."]
    #[inline(always)]
    pub fn nvf(&self) -> NvfR {
        NvfR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Function Dependency Link \\[FDL\\]
This field is used to specify dependencies between PFs. It can be modified independently for each Function from the local management bus."]
    #[inline(always)]
    pub fn fdl(&self) -> FdlR {
        FdlR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NumVFs \\[NVF\\]
This field must be set by the software to the number of VFs that it wants to enable for each PF. This field can be changed only when the VF Enable bit in the SR-IOV Control Register is 0. Its value should not exceed the setting of TotalVFs for the corresponding Physical Function. This field can also be written from the local management bus."]
    #[inline(always)]
    #[must_use]
    pub fn nvf(&mut self) -> NvfW<PciePfFunctionDependencyLinkNumvfsSpec> {
        NvfW::new(self, 0)
    }
}
#[doc = "Function Dependency Link/NumVFs Register This field is used to specify dependencies between PFs. It can be modified independently for each Function from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_function_dependency_link_numvfs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_function_dependency_link_numvfs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfFunctionDependencyLinkNumvfsSpec;
impl crate::RegisterSpec for PciePfFunctionDependencyLinkNumvfsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_function_dependency_link_numvfs::R`](R) reader structure"]
impl crate::Readable for PciePfFunctionDependencyLinkNumvfsSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_function_dependency_link_numvfs::W`](W) writer structure"]
impl crate::Writable for PciePfFunctionDependencyLinkNumvfsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_FUNCTION_DEPENDENCY_LINK_NUMVFS to value 0"]
impl crate::Resettable for PciePfFunctionDependencyLinkNumvfsSpec {
    const RESET_VALUE: u32 = 0;
}
