#[doc = "Register `PCIE_PF_VF_MIGRATION_STATE_ARRAY_OFFSET` reader"]
pub type R = crate::R<PciePfVfMigrationStateArrayOffsetSpec>;
#[doc = "Field `MSAOR` reader - MSAOR\n\n(no description)"]
pub type MsaorR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MSAOR\n\n(no description)"]
    #[inline(always)]
    pub fn msaor(&self) -> MsaorR {
        MsaorR::new(self.bits)
    }
}
#[doc = "VF Migration State Array Offset Register\n\n(no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vf_migration_state_array_offset::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfVfMigrationStateArrayOffsetSpec;
impl crate::RegisterSpec for PciePfVfMigrationStateArrayOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_vf_migration_state_array_offset::R`](R) reader structure"]
impl crate::Readable for PciePfVfMigrationStateArrayOffsetSpec {}
#[doc = "`reset()` method sets PCIE_PF_VF_MIGRATION_STATE_ARRAY_OFFSET to value 0"]
impl crate::Resettable for PciePfVfMigrationStateArrayOffsetSpec {
    const RESET_VALUE: u32 = 0;
}
