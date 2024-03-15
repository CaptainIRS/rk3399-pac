#[doc = "Register `VF_MIGRATION_STATE_ARRAY_OFFSET` reader"]
pub type R = crate::R<VfMigrationStateArrayOffsetSpec>;
#[doc = "Field `MSAOR` reader - MSAOR (no description)"]
pub type MsaorR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MSAOR (no description)"]
    #[inline(always)]
    pub fn msaor(&self) -> MsaorR {
        MsaorR::new(self.bits)
    }
}
#[doc = "VF Migration State Array Offset Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_migration_state_array_offset::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfMigrationStateArrayOffsetSpec;
impl crate::RegisterSpec for VfMigrationStateArrayOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vf_migration_state_array_offset::R`](R) reader structure"]
impl crate::Readable for VfMigrationStateArrayOffsetSpec {}
#[doc = "`reset()` method sets VF_MIGRATION_STATE_ARRAY_OFFSET to value 0"]
impl crate::Resettable for VfMigrationStateArrayOffsetSpec {
    const RESET_VALUE: u32 = 0;
}
