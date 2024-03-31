#[doc = "Register `DENALI_PHY_194` reader"]
pub type R = crate::R<DenaliPhy194Spec>;
#[doc = "Register `DENALI_PHY_194` writer"]
pub type W = crate::W<DenaliPhy194Spec>;
#[doc = "Field `PHY_RDDQ4_SLAVE_DELAY_1` reader - Read DQ4 slave delay setting for slice 1."]
pub type PhyRddq4SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ4_SLAVE_DELAY_1` writer - Read DQ4 slave delay setting for slice 1."]
pub type PhyRddq4SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQ5_SLAVE_DELAY_1` reader - Read DQ5 slave delay setting for slice 1."]
pub type PhyRddq5SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ5_SLAVE_DELAY_1` writer - Read DQ5 slave delay setting for slice 1."]
pub type PhyRddq5SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DQ4 slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_rddq4_slave_delay_1(&self) -> PhyRddq4SlaveDelay1R {
        PhyRddq4SlaveDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQ5 slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_rddq5_slave_delay_1(&self) -> PhyRddq5SlaveDelay1R {
        PhyRddq5SlaveDelay1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQ4 slave delay setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq4_slave_delay_1(&mut self) -> PhyRddq4SlaveDelay1W<DenaliPhy194Spec> {
        PhyRddq4SlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQ5 slave delay setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq5_slave_delay_1(&mut self) -> PhyRddq5SlaveDelay1W<DenaliPhy194Spec> {
        PhyRddq5SlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_194::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_194::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy194Spec;
impl crate::RegisterSpec for DenaliPhy194Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_194::R`](R) reader structure"]
impl crate::Readable for DenaliPhy194Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_194::W`](W) writer structure"]
impl crate::Writable for DenaliPhy194Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_194 to value 0"]
impl crate::Resettable for DenaliPhy194Spec {
    const RESET_VALUE: u32 = 0;
}
