#[doc = "Register `DENALI_PHY_136` reader"]
pub type R = crate::R<DenaliPhy136Spec>;
#[doc = "Register `DENALI_PHY_136` writer"]
pub type W = crate::W<DenaliPhy136Spec>;
#[doc = "Field `PHY_DBI_MODE_1` reader - DBI mode for slice 1. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
pub type PhyDbiMode1R = crate::BitReader;
#[doc = "Field `PHY_DBI_MODE_1` writer - DBI mode for slice 1. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
pub type PhyDbiMode1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_PER_RANK_CS_MAP_1` reader - Per-rank CS map for slice 1."]
pub type PhyPerRankCsMap1R = crate::FieldReader;
#[doc = "Field `PHY_PER_RANK_CS_MAP_1` writer - Per-rank CS map for slice 1."]
pub type PhyPerRankCsMap1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_PER_CS_TRAINING_MULTICAST_EN_1` reader - When set, a register write will update parameters for all ranks at the same time in slice 1. Set to 1 to enable."]
pub type PhyPerCsTrainingMulticastEn1R = crate::BitReader;
#[doc = "Field `PHY_PER_CS_TRAINING_MULTICAST_EN_1` writer - When set, a register write will update parameters for all ranks at the same time in slice 1. Set to 1 to enable."]
pub type PhyPerCsTrainingMulticastEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_PER_CS_TRAINING_INDEX_1` reader - For per-rank training, indicates which ranksparamtersareread/ writtenforslice1.'"]
pub type PhyPerCsTrainingIndex1R = crate::BitReader;
#[doc = "Field `PHY_PER_CS_TRAINING_INDEX_1` writer - For per-rank training, indicates which ranksparamtersareread/ writtenforslice1.'"]
pub type PhyPerCsTrainingIndex1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DBI mode for slice 1. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
    #[inline(always)]
    pub fn phy_dbi_mode_1(&self) -> PhyDbiMode1R {
        PhyDbiMode1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Per-rank CS map for slice 1."]
    #[inline(always)]
    pub fn phy_per_rank_cs_map_1(&self) -> PhyPerRankCsMap1R {
        PhyPerRankCsMap1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - When set, a register write will update parameters for all ranks at the same time in slice 1. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_per_cs_training_multicast_en_1(&self) -> PhyPerCsTrainingMulticastEn1R {
        PhyPerCsTrainingMulticastEn1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - For per-rank training, indicates which ranksparamtersareread/ writtenforslice1.'"]
    #[inline(always)]
    pub fn phy_per_cs_training_index_1(&self) -> PhyPerCsTrainingIndex1R {
        PhyPerCsTrainingIndex1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBI mode for slice 1. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dbi_mode_1(&mut self) -> PhyDbiMode1W<DenaliPhy136Spec> {
        PhyDbiMode1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Per-rank CS map for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_rank_cs_map_1(&mut self) -> PhyPerRankCsMap1W<DenaliPhy136Spec> {
        PhyPerRankCsMap1W::new(self, 8)
    }
    #[doc = "Bit 16 - When set, a register write will update parameters for all ranks at the same time in slice 1. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_cs_training_multicast_en_1(
        &mut self,
    ) -> PhyPerCsTrainingMulticastEn1W<DenaliPhy136Spec> {
        PhyPerCsTrainingMulticastEn1W::new(self, 16)
    }
    #[doc = "Bit 24 - For per-rank training, indicates which ranksparamtersareread/ writtenforslice1.'"]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_cs_training_index_1(&mut self) -> PhyPerCsTrainingIndex1W<DenaliPhy136Spec> {
        PhyPerCsTrainingIndex1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_136::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_136::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy136Spec;
impl crate::RegisterSpec for DenaliPhy136Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_136::R`](R) reader structure"]
impl crate::Readable for DenaliPhy136Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_136::W`](W) writer structure"]
impl crate::Writable for DenaliPhy136Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_136 to value 0x0001_0000"]
impl crate::Resettable for DenaliPhy136Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
