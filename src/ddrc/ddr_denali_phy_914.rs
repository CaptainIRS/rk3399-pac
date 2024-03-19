#[doc = "Register `DDR_DENALI_PHY_914` reader"]
pub type R = crate::R<DdrDenaliPhy914Spec>;
#[doc = "Register `DDR_DENALI_PHY_914` writer"]
pub type W = crate::W<DdrDenaliPhy914Spec>;
#[doc = "Field `PHY_PAD_VREF_CTRL_DQ_1` reader - Pad VREF control settings for DQ slice 1."]
pub type PhyPadVrefCtrlDq1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PAD_VREF_CTRL_DQ_1` writer - Pad VREF control settings for DQ slice 1."]
pub type PhyPadVrefCtrlDq1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_PAD_VREF_CTRL_DQ_2` reader - Pad VREF control settings for DQ slice 2."]
pub type PhyPadVrefCtrlDq2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PAD_VREF_CTRL_DQ_2` writer - Pad VREF control settings for DQ slice 2."]
pub type PhyPadVrefCtrlDq2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Pad VREF control settings for DQ slice 1."]
    #[inline(always)]
    pub fn phy_pad_vref_ctrl_dq_1(&self) -> PhyPadVrefCtrlDq1R {
        PhyPadVrefCtrlDq1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Pad VREF control settings for DQ slice 2."]
    #[inline(always)]
    pub fn phy_pad_vref_ctrl_dq_2(&self) -> PhyPadVrefCtrlDq2R {
        PhyPadVrefCtrlDq2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Pad VREF control settings for DQ slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_vref_ctrl_dq_1(&mut self) -> PhyPadVrefCtrlDq1W<DdrDenaliPhy914Spec> {
        PhyPadVrefCtrlDq1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Pad VREF control settings for DQ slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_vref_ctrl_dq_2(&mut self) -> PhyPadVrefCtrlDq2W<DdrDenaliPhy914Spec> {
        PhyPadVrefCtrlDq2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_914::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_914::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy914Spec;
impl crate::RegisterSpec for DdrDenaliPhy914Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_914::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy914Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_914::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy914Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_914 to value 0"]
impl crate::Resettable for DdrDenaliPhy914Spec {
    const RESET_VALUE: u32 = 0;
}
