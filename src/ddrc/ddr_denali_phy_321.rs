#[doc = "Register `DDR_DENALI_PHY_321` reader"]
pub type R = crate::R<DdrDenaliPhy321Spec>;
#[doc = "Register `DDR_DENALI_PHY_321` writer"]
pub type W = crate::W<DdrDenaliPhy321Spec>;
#[doc = "Field `PHY_RDDQ2_SLAVE_DELAY_2` reader - Read DQ2 slave delay setting for slice 2."]
pub type PhyRddq2SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ2_SLAVE_DELAY_2` writer - Read DQ2 slave delay setting for slice 2."]
pub type PhyRddq2SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQ3_SLAVE_DELAY_2` reader - Read DQ3 slave delay setting for slice 2."]
pub type PhyRddq3SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ3_SLAVE_DELAY_2` writer - Read DQ3 slave delay setting for slice 2."]
pub type PhyRddq3SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DQ2 slave delay setting for slice 2."]
    #[inline(always)]
    pub fn phy_rddq2_slave_delay_2(&self) -> PhyRddq2SlaveDelay2R {
        PhyRddq2SlaveDelay2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQ3 slave delay setting for slice 2."]
    #[inline(always)]
    pub fn phy_rddq3_slave_delay_2(&self) -> PhyRddq3SlaveDelay2R {
        PhyRddq3SlaveDelay2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQ2 slave delay setting for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq2_slave_delay_2(&mut self) -> PhyRddq2SlaveDelay2W<DdrDenaliPhy321Spec> {
        PhyRddq2SlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQ3 slave delay setting for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq3_slave_delay_2(&mut self) -> PhyRddq3SlaveDelay2W<DdrDenaliPhy321Spec> {
        PhyRddq3SlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_321::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_321::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy321Spec;
impl crate::RegisterSpec for DdrDenaliPhy321Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_321::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy321Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_321::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy321Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_321 to value 0"]
impl crate::Resettable for DdrDenaliPhy321Spec {
    const RESET_VALUE: u32 = 0;
}
