#[doc = "Register `DDR_DENALI_CTL_106` reader"]
pub type R = crate::R<DdrDenaliCtl106Spec>;
#[doc = "Register `DDR_DENALI_CTL_106` writer"]
pub type W = crate::W<DdrDenaliCtl106Spec>;
#[doc = "Field `LPC_PROMOTE_THRESHOLD_F2` reader - LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands."]
pub type LpcPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `LPC_PROMOTE_THRESHOLD_F2` writer - LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands."]
pub type LpcPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LPC_SR_CTRLUPD_EN` reader - Enable LPC to execute a DFI control update on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrCtrlupdEnR = crate::BitReader;
#[doc = "Field `LPC_SR_CTRLUPD_EN` writer - Enable LPC to execute a DFI control update on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrCtrlupdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPC_SR_PHYUPD_EN` reader - Enable LPC to execute a DFI PHY update on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrPhyupdEnR = crate::BitReader;
#[doc = "Field `LPC_SR_PHYUPD_EN` writer - Enable LPC to execute a DFI PHY update on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrPhyupdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands."]
    #[inline(always)]
    pub fn lpc_promote_threshold_f2(&self) -> LpcPromoteThresholdF2R {
        LpcPromoteThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable LPC to execute a DFI control update on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    pub fn lpc_sr_ctrlupd_en(&self) -> LpcSrCtrlupdEnR {
        LpcSrCtrlupdEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable LPC to execute a DFI PHY update on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    pub fn lpc_sr_phyupd_en(&self) -> LpcSrPhyupdEnR {
        LpcSrPhyupdEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands."]
    #[inline(always)]
    #[must_use]
    pub fn lpc_promote_threshold_f2(&mut self) -> LpcPromoteThresholdF2W<DdrDenaliCtl106Spec> {
        LpcPromoteThresholdF2W::new(self, 0)
    }
    #[doc = "Bit 16 - Enable LPC to execute a DFI control update on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lpc_sr_ctrlupd_en(&mut self) -> LpcSrCtrlupdEnW<DdrDenaliCtl106Spec> {
        LpcSrCtrlupdEnW::new(self, 16)
    }
    #[doc = "Bit 24 - Enable LPC to execute a DFI PHY update on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lpc_sr_phyupd_en(&mut self) -> LpcSrPhyupdEnW<DdrDenaliCtl106Spec> {
        LpcSrPhyupdEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_106::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_106::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl106Spec;
impl crate::RegisterSpec for DdrDenaliCtl106Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_106::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl106Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_106::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl106Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_106 to value 0"]
impl crate::Resettable for DdrDenaliCtl106Spec {
    const RESET_VALUE: u32 = 0;
}
