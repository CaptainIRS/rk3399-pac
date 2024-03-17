#[doc = "Register `DENALI_PHY_08` reader"]
pub type R = crate::R<DenaliPhy08Spec>;
#[doc = "Register `DENALI_PHY_08` writer"]
pub type W = crate::W<DenaliPhy08Spec>;
#[doc = "Field `PHY_DBI_MODE_0` reader - DBI mode for slice 0. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
pub type PhyDbiMode0R = crate::BitReader;
#[doc = "Field `PHY_DBI_MODE_0` writer - DBI mode for slice 0. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
pub type PhyDbiMode0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_PER_RANK_CS_MAP_0` reader - Per-rank CS map for slice 0."]
pub type PhyPerRankCsMap0R = crate::FieldReader;
#[doc = "Field `PHY_PER_RANK_CS_MAP_0` writer - Per-rank CS map for slice 0."]
pub type PhyPerRankCsMap0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_PER_CS_TRAINING_MULTICAST_EN_0` reader - When set, a register write will update parameters for all ranks at the same time in slice 0. Set to 1 to enable."]
pub type PhyPerCsTrainingMulticastEn0R = crate::BitReader;
#[doc = "Field `PHY_PER_CS_TRAINING_MULTICAST_EN_0` writer - When set, a register write will update parameters for all ranks at the same time in slice 0. Set to 1 to enable."]
pub type PhyPerCsTrainingMulticastEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_PER_CS_TRAINING_INDEX_0` reader - For per-rank training, indicates which ranksparamtersareread/ writtenforslice0.'"]
pub type PhyPerCsTrainingIndex0R = crate::BitReader;
#[doc = "Field `PHY_PER_CS_TRAINING_INDEX_0` writer - For per-rank training, indicates which ranksparamtersareread/ writtenforslice0.'"]
pub type PhyPerCsTrainingIndex0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DBI mode for slice 0. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
    #[inline(always)]
    pub fn phy_dbi_mode_0(&self) -> PhyDbiMode0R {
        PhyDbiMode0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Per-rank CS map for slice 0."]
    #[inline(always)]
    pub fn phy_per_rank_cs_map_0(&self) -> PhyPerRankCsMap0R {
        PhyPerRankCsMap0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - When set, a register write will update parameters for all ranks at the same time in slice 0. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_per_cs_training_multicast_en_0(&self) -> PhyPerCsTrainingMulticastEn0R {
        PhyPerCsTrainingMulticastEn0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - For per-rank training, indicates which ranksparamtersareread/ writtenforslice0.'"]
    #[inline(always)]
    pub fn phy_per_cs_training_index_0(&self) -> PhyPerCsTrainingIndex0R {
        PhyPerCsTrainingIndex0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBI mode for slice 0. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dbi_mode_0(&mut self) -> PhyDbiMode0W<DenaliPhy08Spec> {
        PhyDbiMode0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Per-rank CS map for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_rank_cs_map_0(&mut self) -> PhyPerRankCsMap0W<DenaliPhy08Spec> {
        PhyPerRankCsMap0W::new(self, 8)
    }
    #[doc = "Bit 16 - When set, a register write will update parameters for all ranks at the same time in slice 0. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_cs_training_multicast_en_0(
        &mut self,
    ) -> PhyPerCsTrainingMulticastEn0W<DenaliPhy08Spec> {
        PhyPerCsTrainingMulticastEn0W::new(self, 16)
    }
    #[doc = "Bit 24 - For per-rank training, indicates which ranksparamtersareread/ writtenforslice0.'"]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_cs_training_index_0(&mut self) -> PhyPerCsTrainingIndex0W<DenaliPhy08Spec> {
        PhyPerCsTrainingIndex0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_08::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_08::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy08Spec;
impl crate::RegisterSpec for DenaliPhy08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_08::R`](R) reader structure"]
impl crate::Readable for DenaliPhy08Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_08::W`](W) writer structure"]
impl crate::Writable for DenaliPhy08Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_08 to value 0x0001_0000"]
impl crate::Resettable for DenaliPhy08Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
