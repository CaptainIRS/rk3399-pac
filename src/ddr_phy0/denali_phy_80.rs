#[doc = "Register `DENALI_PHY_80` reader"]
pub type R = crate::R<DenaliPhy80Spec>;
#[doc = "Register `DENALI_PHY_80` writer"]
pub type W = crate::W<DenaliPhy80Spec>;
#[doc = "Field `PHY_GTLVL_RDDQS_SLV_DLY_START_0` reader - Initial read DQS gate slave delay setting during gate training for slice 0."]
pub type PhyGtlvlRddqsSlvDlyStart0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_RDDQS_SLV_DLY_START_0` writer - Initial read DQS gate slave delay setting during gate training for slice 0."]
pub type PhyGtlvlRddqsSlvDlyStart0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_GTLVL_LAT_ADJ_START_0` reader - Initial read DQS gate cycle delay from dfi_rddata_en during gate training for slice 0."]
pub type PhyGtlvlLatAdjStart0R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_LAT_ADJ_START_0` writer - Initial read DQS gate cycle delay from dfi_rddata_en during gate training for slice 0."]
pub type PhyGtlvlLatAdjStart0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:9 - Initial read DQS gate slave delay setting during gate training for slice 0."]
    #[inline(always)]
    pub fn phy_gtlvl_rddqs_slv_dly_start_0(&self) -> PhyGtlvlRddqsSlvDlyStart0R {
        PhyGtlvlRddqsSlvDlyStart0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:19 - Initial read DQS gate cycle delay from dfi_rddata_en during gate training for slice 0."]
    #[inline(always)]
    pub fn phy_gtlvl_lat_adj_start_0(&self) -> PhyGtlvlLatAdjStart0R {
        PhyGtlvlLatAdjStart0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Initial read DQS gate slave delay setting during gate training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_rddqs_slv_dly_start_0(
        &mut self,
    ) -> PhyGtlvlRddqsSlvDlyStart0W<DenaliPhy80Spec> {
        PhyGtlvlRddqsSlvDlyStart0W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Initial read DQS gate cycle delay from dfi_rddata_en during gate training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_lat_adj_start_0(&mut self) -> PhyGtlvlLatAdjStart0W<DenaliPhy80Spec> {
        PhyGtlvlLatAdjStart0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_80::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_80::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy80Spec;
impl crate::RegisterSpec for DenaliPhy80Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_80::R`](R) reader structure"]
impl crate::Readable for DenaliPhy80Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_80::W`](W) writer structure"]
impl crate::Writable for DenaliPhy80Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_80 to value 0"]
impl crate::Resettable for DenaliPhy80Spec {
    const RESET_VALUE: u32 = 0;
}
