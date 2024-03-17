#[doc = "Register `DENALI_PHY_336` reader"]
pub type R = crate::R<DenaliPhy336Spec>;
#[doc = "Register `DENALI_PHY_336` writer"]
pub type W = crate::W<DenaliPhy336Spec>;
#[doc = "Field `PHY_GTLVL_RDDQS_SLV_DLY_START_2` reader - Initial read DQS gate slave delay setting during gate training for slice 2."]
pub type PhyGtlvlRddqsSlvDlyStart2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_RDDQS_SLV_DLY_START_2` writer - Initial read DQS gate slave delay setting during gate training for slice 2."]
pub type PhyGtlvlRddqsSlvDlyStart2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_GTLVL_LAT_ADJ_START_2` reader - Initial read DQS gate cycle delay from dfi_rddata_en during gate training for slice 2."]
pub type PhyGtlvlLatAdjStart2R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_LAT_ADJ_START_2` writer - Initial read DQS gate cycle delay from dfi_rddata_en during gate training for slice 2."]
pub type PhyGtlvlLatAdjStart2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:9 - Initial read DQS gate slave delay setting during gate training for slice 2."]
    #[inline(always)]
    pub fn phy_gtlvl_rddqs_slv_dly_start_2(&self) -> PhyGtlvlRddqsSlvDlyStart2R {
        PhyGtlvlRddqsSlvDlyStart2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:19 - Initial read DQS gate cycle delay from dfi_rddata_en during gate training for slice 2."]
    #[inline(always)]
    pub fn phy_gtlvl_lat_adj_start_2(&self) -> PhyGtlvlLatAdjStart2R {
        PhyGtlvlLatAdjStart2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Initial read DQS gate slave delay setting during gate training for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_rddqs_slv_dly_start_2(
        &mut self,
    ) -> PhyGtlvlRddqsSlvDlyStart2W<DenaliPhy336Spec> {
        PhyGtlvlRddqsSlvDlyStart2W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Initial read DQS gate cycle delay from dfi_rddata_en during gate training for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_lat_adj_start_2(&mut self) -> PhyGtlvlLatAdjStart2W<DenaliPhy336Spec> {
        PhyGtlvlLatAdjStart2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_336::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_336::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy336Spec;
impl crate::RegisterSpec for DenaliPhy336Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_336::R`](R) reader structure"]
impl crate::Readable for DenaliPhy336Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_336::W`](W) writer structure"]
impl crate::Writable for DenaliPhy336Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_336 to value 0"]
impl crate::Resettable for DenaliPhy336Spec {
    const RESET_VALUE: u32 = 0;
}
