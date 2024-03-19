#[doc = "Register `DDR_DENALI_CTL_200` reader"]
pub type R = crate::R<DdrDenaliCtl200Spec>;
#[doc = "Register `DDR_DENALI_CTL_200` writer"]
pub type W = crate::W<DdrDenaliCtl200Spec>;
#[doc = "Field `CONTROLLER_BUSY` reader - Indicator that the controller is processing a command. Evaluates all ports for outstanding transactions. Value of 1 indicates controller busy."]
pub type ControllerBusyR = crate::BitReader;
#[doc = "Field `CTRLUPD_REQ` writer - Assert the DFI controller-initiated update request signal dfi_ctrlupd_req. Set to 1 to trigger."]
pub type CtrlupdReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRLUPD_REQ_PER_AREF_EN` reader - Enable an automatic controller- initiated update (dfi_ctrlupd_req) after every refresh. Set to 1 to enable."]
pub type CtrlupdReqPerArefEnR = crate::BitReader;
#[doc = "Field `CTRLUPD_REQ_PER_AREF_EN` writer - Enable an automatic controller- initiated update (dfi_ctrlupd_req) after every refresh. Set to 1 to enable."]
pub type CtrlupdReqPerArefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREAMBLE_SUPPORT` reader - Selection of one or two cycle preamble for read and write burst transfers."]
pub type PreambleSupportR = crate::FieldReader;
#[doc = "Field `PREAMBLE_SUPPORT` writer - Selection of one or two cycle preamble for read and write burst transfers."]
pub type PreambleSupportW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Indicator that the controller is processing a command. Evaluates all ports for outstanding transactions. Value of 1 indicates controller busy."]
    #[inline(always)]
    pub fn controller_busy(&self) -> ControllerBusyR {
        ControllerBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Enable an automatic controller- initiated update (dfi_ctrlupd_req) after every refresh. Set to 1 to enable."]
    #[inline(always)]
    pub fn ctrlupd_req_per_aref_en(&self) -> CtrlupdReqPerArefEnR {
        CtrlupdReqPerArefEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Selection of one or two cycle preamble for read and write burst transfers."]
    #[inline(always)]
    pub fn preamble_support(&self) -> PreambleSupportR {
        PreambleSupportR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Assert the DFI controller-initiated update request signal dfi_ctrlupd_req. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn ctrlupd_req(&mut self) -> CtrlupdReqW<DdrDenaliCtl200Spec> {
        CtrlupdReqW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable an automatic controller- initiated update (dfi_ctrlupd_req) after every refresh. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn ctrlupd_req_per_aref_en(&mut self) -> CtrlupdReqPerArefEnW<DdrDenaliCtl200Spec> {
        CtrlupdReqPerArefEnW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Selection of one or two cycle preamble for read and write burst transfers."]
    #[inline(always)]
    #[must_use]
    pub fn preamble_support(&mut self) -> PreambleSupportW<DdrDenaliCtl200Spec> {
        PreambleSupportW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_200::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_200::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl200Spec;
impl crate::RegisterSpec for DdrDenaliCtl200Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_200::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl200Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_200::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl200Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_200 to value 0"]
impl crate::Resettable for DdrDenaliCtl200Spec {
    const RESET_VALUE: u32 = 0;
}
