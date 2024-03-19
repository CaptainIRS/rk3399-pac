#[doc = "Register `DDR_DENALI_PHY_264` reader"]
pub type R = crate::R<DdrDenaliPhy264Spec>;
#[doc = "Register `DDR_DENALI_PHY_264` writer"]
pub type W = crate::W<DdrDenaliPhy264Spec>;
#[doc = "Field `PHY_DBI_MODE_2` reader - DBI mode for slice 2. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
pub type PhyDbiMode2R = crate::BitReader;
#[doc = "Field `PHY_DBI_MODE_2` writer - DBI mode for slice 2. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
pub type PhyDbiMode2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_PER_RANK_CS_MAP_2` reader - Per-rank CS map for slice 2."]
pub type PhyPerRankCsMap2R = crate::FieldReader;
#[doc = "Field `PHY_PER_RANK_CS_MAP_2` writer - Per-rank CS map for slice 2."]
pub type PhyPerRankCsMap2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_PER_CS_TRAINING_MULTICAST_EN_2` reader - When set, a register write will update parameters for all ranks at the same time in slice 2. Set to 1 to enable."]
pub type PhyPerCsTrainingMulticastEn2R = crate::BitReader;
#[doc = "Field `PHY_PER_CS_TRAINING_MULTICAST_EN_2` writer - When set, a register write will update parameters for all ranks at the same time in slice 2. Set to 1 to enable."]
pub type PhyPerCsTrainingMulticastEn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_PER_CS_TRAINING_INDEX_2` reader - For per-rank training, indicates which ranksparamtersareread/ writtenforslice2.'"]
pub type PhyPerCsTrainingIndex2R = crate::BitReader;
#[doc = "Field `PHY_PER_CS_TRAINING_INDEX_2` writer - For per-rank training, indicates which ranksparamtersareread/ writtenforslice2.'"]
pub type PhyPerCsTrainingIndex2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DBI mode for slice 2. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
    #[inline(always)]
    pub fn phy_dbi_mode_2(&self) -> PhyDbiMode2R {
        PhyDbiMode2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Per-rank CS map for slice 2."]
    #[inline(always)]
    pub fn phy_per_rank_cs_map_2(&self) -> PhyPerRankCsMap2R {
        PhyPerRankCsMap2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - When set, a register write will update parameters for all ranks at the same time in slice 2. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_per_cs_training_multicast_en_2(&self) -> PhyPerCsTrainingMulticastEn2R {
        PhyPerCsTrainingMulticastEn2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - For per-rank training, indicates which ranksparamtersareread/ writtenforslice2.'"]
    #[inline(always)]
    pub fn phy_per_cs_training_index_2(&self) -> PhyPerCsTrainingIndex2R {
        PhyPerCsTrainingIndex2R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBI mode for slice 2. Bit (0) enables return of DBI read data. Bit (1) enables read leveling of the DBI signal only."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dbi_mode_2(&mut self) -> PhyDbiMode2W<DdrDenaliPhy264Spec> {
        PhyDbiMode2W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Per-rank CS map for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_rank_cs_map_2(&mut self) -> PhyPerRankCsMap2W<DdrDenaliPhy264Spec> {
        PhyPerRankCsMap2W::new(self, 8)
    }
    #[doc = "Bit 16 - When set, a register write will update parameters for all ranks at the same time in slice 2. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_cs_training_multicast_en_2(
        &mut self,
    ) -> PhyPerCsTrainingMulticastEn2W<DdrDenaliPhy264Spec> {
        PhyPerCsTrainingMulticastEn2W::new(self, 16)
    }
    #[doc = "Bit 24 - For per-rank training, indicates which ranksparamtersareread/ writtenforslice2.'"]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_cs_training_index_2(&mut self) -> PhyPerCsTrainingIndex2W<DdrDenaliPhy264Spec> {
        PhyPerCsTrainingIndex2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_264::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_264::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy264Spec;
impl crate::RegisterSpec for DdrDenaliPhy264Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_264::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy264Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_264::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy264Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_264 to value 0x0001_0000"]
impl crate::Resettable for DdrDenaliPhy264Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
