#[doc = "Register `DENALI_PHY_915` reader"]
pub type R = crate::R<DenaliPhy915Spec>;
#[doc = "Register `DENALI_PHY_915` writer"]
pub type W = crate::W<DenaliPhy915Spec>;
#[doc = "Field `PHY_PAD_VREF_CTRL_DQ_3` reader - Pad VREF control settings for DQ slice 3."]
pub type PhyPadVrefCtrlDq3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PAD_VREF_CTRL_DQ_3` writer - Pad VREF control settings for DQ slice 3."]
pub type PhyPadVrefCtrlDq3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_PAD_VREF_CTRL_AC` reader - Pad VREF control settings for the address/control."]
pub type PhyPadVrefCtrlAcR = crate::FieldReader<u16>;
#[doc = "Field `PHY_PAD_VREF_CTRL_AC` writer - Pad VREF control settings for the address/control."]
pub type PhyPadVrefCtrlAcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Pad VREF control settings for DQ slice 3."]
    #[inline(always)]
    pub fn phy_pad_vref_ctrl_dq_3(&self) -> PhyPadVrefCtrlDq3R {
        PhyPadVrefCtrlDq3R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Pad VREF control settings for the address/control."]
    #[inline(always)]
    pub fn phy_pad_vref_ctrl_ac(&self) -> PhyPadVrefCtrlAcR {
        PhyPadVrefCtrlAcR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Pad VREF control settings for DQ slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_vref_ctrl_dq_3(&mut self) -> PhyPadVrefCtrlDq3W<DenaliPhy915Spec> {
        PhyPadVrefCtrlDq3W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Pad VREF control settings for the address/control."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_vref_ctrl_ac(&mut self) -> PhyPadVrefCtrlAcW<DenaliPhy915Spec> {
        PhyPadVrefCtrlAcW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_915::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_915::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy915Spec;
impl crate::RegisterSpec for DenaliPhy915Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_915::R`](R) reader structure"]
impl crate::Readable for DenaliPhy915Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_915::W`](W) writer structure"]
impl crate::Writable for DenaliPhy915Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_915 to value 0"]
impl crate::Resettable for DenaliPhy915Spec {
    const RESET_VALUE: u32 = 0;
}
