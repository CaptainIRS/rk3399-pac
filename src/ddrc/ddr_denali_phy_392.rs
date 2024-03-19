#[doc = "Register `DDR_DENALI_PHY_392` reader"]
pub type R = crate::R<DdrDenaliPhy392Spec>;
#[doc = "Register `DDR_DENALI_PHY_392` writer"]
pub type W = crate::W<DdrDenaliPhy392Spec>;
#[doc = "Field `PHY_DBI_MODE_3` reader - DBI mode for slice 3. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
pub type PhyDbiMode3R = crate::BitReader;
#[doc = "Field `PHY_DBI_MODE_3` writer - DBI mode for slice 3. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
pub type PhyDbiMode3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_PER_RANK_CS_MAP_3` reader - Per-rank CS map for slice 3."]
pub type PhyPerRankCsMap3R = crate::FieldReader;
#[doc = "Field `PHY_PER_RANK_CS_MAP_3` writer - Per-rank CS map for slice 3."]
pub type PhyPerRankCsMap3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_PER_CS_TRAINING_MULTICAST_EN_3` reader - When set, a register write will update parameters for all ranks at the same time in slice 3. Set to 1 to enable."]
pub type PhyPerCsTrainingMulticastEn3R = crate::BitReader;
#[doc = "Field `PHY_PER_CS_TRAINING_MULTICAST_EN_3` writer - When set, a register write will update parameters for all ranks at the same time in slice 3. Set to 1 to enable."]
pub type PhyPerCsTrainingMulticastEn3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_PER_CS_TRAINING_INDEX_3` reader - For per-rank training, indicates which ranksparamtersareread/ writtenforslice3.'"]
pub type PhyPerCsTrainingIndex3R = crate::BitReader;
#[doc = "Field `PHY_PER_CS_TRAINING_INDEX_3` writer - For per-rank training, indicates which ranksparamtersareread/ writtenforslice3.'"]
pub type PhyPerCsTrainingIndex3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DBI mode for slice 3. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
    #[inline(always)]
    pub fn phy_dbi_mode_3(&self) -> PhyDbiMode3R {
        PhyDbiMode3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Per-rank CS map for slice 3."]
    #[inline(always)]
    pub fn phy_per_rank_cs_map_3(&self) -> PhyPerRankCsMap3R {
        PhyPerRankCsMap3R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - When set, a register write will update parameters for all ranks at the same time in slice 3. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_per_cs_training_multicast_en_3(&self) -> PhyPerCsTrainingMulticastEn3R {
        PhyPerCsTrainingMulticastEn3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - For per-rank training, indicates which ranksparamtersareread/ writtenforslice3.'"]
    #[inline(always)]
    pub fn phy_per_cs_training_index_3(&self) -> PhyPerCsTrainingIndex3R {
        PhyPerCsTrainingIndex3R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBI mode for slice 3. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dbi_mode_3(&mut self) -> PhyDbiMode3W<DdrDenaliPhy392Spec> {
        PhyDbiMode3W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Per-rank CS map for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_rank_cs_map_3(&mut self) -> PhyPerRankCsMap3W<DdrDenaliPhy392Spec> {
        PhyPerRankCsMap3W::new(self, 8)
    }
    #[doc = "Bit 16 - When set, a register write will update parameters for all ranks at the same time in slice 3. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_cs_training_multicast_en_3(
        &mut self,
    ) -> PhyPerCsTrainingMulticastEn3W<DdrDenaliPhy392Spec> {
        PhyPerCsTrainingMulticastEn3W::new(self, 16)
    }
    #[doc = "Bit 24 - For per-rank training, indicates which ranksparamtersareread/ writtenforslice3.'"]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_cs_training_index_3(&mut self) -> PhyPerCsTrainingIndex3W<DdrDenaliPhy392Spec> {
        PhyPerCsTrainingIndex3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_392::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_392::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy392Spec;
impl crate::RegisterSpec for DdrDenaliPhy392Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_392::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy392Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_392::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy392Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_392 to value 0x0001_0000"]
impl crate::Resettable for DdrDenaliPhy392Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
