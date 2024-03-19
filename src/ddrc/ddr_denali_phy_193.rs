#[doc = "Register `DDR_DENALI_PHY_193` reader"]
pub type R = crate::R<DdrDenaliPhy193Spec>;
#[doc = "Register `DDR_DENALI_PHY_193` writer"]
pub type W = crate::W<DdrDenaliPhy193Spec>;
#[doc = "Field `PHY_RDDQ2_SLAVE_DELAY_1` reader - Read DQ2 slave delay setting for slice 1."]
pub type PhyRddq2SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ2_SLAVE_DELAY_1` writer - Read DQ2 slave delay setting for slice 1."]
pub type PhyRddq2SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQ3_SLAVE_DELAY_1` reader - Read DQ3 slave delay setting for slice 1."]
pub type PhyRddq3SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ3_SLAVE_DELAY_1` writer - Read DQ3 slave delay setting for slice 1."]
pub type PhyRddq3SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DQ2 slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_rddq2_slave_delay_1(&self) -> PhyRddq2SlaveDelay1R {
        PhyRddq2SlaveDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQ3 slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_rddq3_slave_delay_1(&self) -> PhyRddq3SlaveDelay1R {
        PhyRddq3SlaveDelay1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQ2 slave delay setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq2_slave_delay_1(&mut self) -> PhyRddq2SlaveDelay1W<DdrDenaliPhy193Spec> {
        PhyRddq2SlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQ3 slave delay setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq3_slave_delay_1(&mut self) -> PhyRddq3SlaveDelay1W<DdrDenaliPhy193Spec> {
        PhyRddq3SlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_193::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_193::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy193Spec;
impl crate::RegisterSpec for DdrDenaliPhy193Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_193::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy193Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_193::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy193Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_193 to value 0"]
impl crate::Resettable for DdrDenaliPhy193Spec {
    const RESET_VALUE: u32 = 0;
}
