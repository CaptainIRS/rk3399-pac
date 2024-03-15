#[doc = "Register `DENALI_PHY_320` reader"]
pub type R = crate::R<DenaliPhy320Spec>;
#[doc = "Register `DENALI_PHY_320` writer"]
pub type W = crate::W<DenaliPhy320Spec>;
#[doc = "Field `PHY_RDDQ0_SLAVE_DELAY_2` reader - Read DQ0 slave delay setting for slice 2."]
pub type PhyRddq0SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ0_SLAVE_DELAY_2` writer - Read DQ0 slave delay setting for slice 2."]
pub type PhyRddq0SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQ1_SLAVE_DELAY_2` reader - Read DQ1 slave delay setting for slice 2."]
pub type PhyRddq1SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ1_SLAVE_DELAY_2` writer - Read DQ1 slave delay setting for slice 2."]
pub type PhyRddq1SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DQ0 slave delay setting for slice 2."]
    #[inline(always)]
    pub fn phy_rddq0_slave_delay_2(&self) -> PhyRddq0SlaveDelay2R {
        PhyRddq0SlaveDelay2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQ1 slave delay setting for slice 2."]
    #[inline(always)]
    pub fn phy_rddq1_slave_delay_2(&self) -> PhyRddq1SlaveDelay2R {
        PhyRddq1SlaveDelay2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQ0 slave delay setting for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq0_slave_delay_2(&mut self) -> PhyRddq0SlaveDelay2W<DenaliPhy320Spec> {
        PhyRddq0SlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQ1 slave delay setting for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq1_slave_delay_2(&mut self) -> PhyRddq1SlaveDelay2W<DenaliPhy320Spec> {
        PhyRddq1SlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_320::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_320::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy320Spec;
impl crate::RegisterSpec for DenaliPhy320Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_320::R`](R) reader structure"]
impl crate::Readable for DenaliPhy320Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_320::W`](W) writer structure"]
impl crate::Writable for DenaliPhy320Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_320 to value 0"]
impl crate::Resettable for DenaliPhy320Spec {
    const RESET_VALUE: u32 = 0;
}
