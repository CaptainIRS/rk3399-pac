#[doc = "Register `DENALI_CTL_107` reader"]
pub type R = crate::R<DenaliCtl107Spec>;
#[doc = "Register `DENALI_CTL_107` writer"]
pub type W = crate::W<DenaliCtl107Spec>;
#[doc = "Field `LPC_SR_PHYMSTR_EN` reader - Enable LPC to execute a DFI PHY Master request on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrPhymstrEnR = crate::BitReader;
#[doc = "Field `LPC_SR_PHYMSTR_EN` writer - Enable LPC to execute a DFI PHY Master request on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrPhymstrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPC_SR_ZQ_EN` reader - Enable LPC to execute a ZQ calibration on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrZqEnR = crate::BitReader;
#[doc = "Field `LPC_SR_ZQ_EN` writer - Enable LPC to execute a ZQ calibration on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrZqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable LPC to execute a DFI PHY Master request on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    pub fn lpc_sr_phymstr_en(&self) -> LpcSrPhymstrEnR {
        LpcSrPhymstrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Enable LPC to execute a ZQ calibration on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    pub fn lpc_sr_zq_en(&self) -> LpcSrZqEnR {
        LpcSrZqEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable LPC to execute a DFI PHY Master request on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lpc_sr_phymstr_en(&mut self) -> LpcSrPhymstrEnW<DenaliCtl107Spec> {
        LpcSrPhymstrEnW::new(self, 0)
    }
    #[doc = "Bit 16 - Enable LPC to execute a ZQ calibration on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lpc_sr_zq_en(&mut self) -> LpcSrZqEnW<DenaliCtl107Spec> {
        LpcSrZqEnW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_107::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_107::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl107Spec;
impl crate::RegisterSpec for DenaliCtl107Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_107::R`](R) reader structure"]
impl crate::Readable for DenaliCtl107Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_107::W`](W) writer structure"]
impl crate::Writable for DenaliCtl107Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_107 to value 0"]
impl crate::Resettable for DenaliCtl107Spec {
    const RESET_VALUE: u32 = 0;
}
