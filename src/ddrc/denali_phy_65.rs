#[doc = "Register `DENALI_PHY_65` reader"]
pub type R = crate::R<DenaliPhy65Spec>;
#[doc = "Register `DENALI_PHY_65` writer"]
pub type W = crate::W<DenaliPhy65Spec>;
#[doc = "Field `PHY_RDDQ2_SLAVE_DELAY_0` reader - Read DQ2 slave delay setting for slice 0."]
pub type PhyRddq2SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ2_SLAVE_DELAY_0` writer - Read DQ2 slave delay setting for slice 0."]
pub type PhyRddq2SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQ3_SLAVE_DELAY_0` reader - Read DQ3 slave delay setting for slice 0."]
pub type PhyRddq3SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ3_SLAVE_DELAY_0` writer - Read DQ3 slave delay setting for slice 0."]
pub type PhyRddq3SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DQ2 slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_rddq2_slave_delay_0(&self) -> PhyRddq2SlaveDelay0R {
        PhyRddq2SlaveDelay0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQ3 slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_rddq3_slave_delay_0(&self) -> PhyRddq3SlaveDelay0R {
        PhyRddq3SlaveDelay0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQ2 slave delay setting for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq2_slave_delay_0(&mut self) -> PhyRddq2SlaveDelay0W<DenaliPhy65Spec> {
        PhyRddq2SlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQ3 slave delay setting for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq3_slave_delay_0(&mut self) -> PhyRddq3SlaveDelay0W<DenaliPhy65Spec> {
        PhyRddq3SlaveDelay0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_65::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_65::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy65Spec;
impl crate::RegisterSpec for DenaliPhy65Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_65::R`](R) reader structure"]
impl crate::Readable for DenaliPhy65Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_65::W`](W) writer structure"]
impl crate::Writable for DenaliPhy65Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_65 to value 0"]
impl crate::Resettable for DenaliPhy65Spec {
    const RESET_VALUE: u32 = 0;
}
