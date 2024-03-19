#[doc = "Register `DDR_DENALI_CTL_255` reader"]
pub type R = crate::R<DdrDenaliCtl255Spec>;
#[doc = "Register `DDR_DENALI_CTL_255` writer"]
pub type W = crate::W<DdrDenaliCtl255Spec>;
#[doc = "Field `RDLVL_GATE_DFI_PROMOTE_THRESHOLD_F2` reader - Gate training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type RdlvlGateDfiPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_DFI_PROMOTE_THRESHOLD_F2` writer - Gate training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type RdlvlGateDfiPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CALVL_REQ` writer - User request to initiate CA training. Set to 1 to trigger."]
pub type CalvlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALVL_CS` reader - Specifies the target chip select for the CA training operation initiated through the CALVL_REQ parameter."]
pub type CalvlCsR = crate::BitReader;
#[doc = "Field `CALVL_CS` writer - Specifies the target chip select for the CA training operation initiated through the CALVL_REQ parameter."]
pub type CalvlCsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Gate training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    pub fn rdlvl_gate_dfi_promote_threshold_f2(&self) -> RdlvlGateDfiPromoteThresholdF2R {
        RdlvlGateDfiPromoteThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Specifies the target chip select for the CA training operation initiated through the CALVL_REQ parameter."]
    #[inline(always)]
    pub fn calvl_cs(&self) -> CalvlCsR {
        CalvlCsR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Gate training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_dfi_promote_threshold_f2(
        &mut self,
    ) -> RdlvlGateDfiPromoteThresholdF2W<DdrDenaliCtl255Spec> {
        RdlvlGateDfiPromoteThresholdF2W::new(self, 0)
    }
    #[doc = "Bit 16 - User request to initiate CA training. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_req(&mut self) -> CalvlReqW<DdrDenaliCtl255Spec> {
        CalvlReqW::new(self, 16)
    }
    #[doc = "Bit 24 - Specifies the target chip select for the CA training operation initiated through the CALVL_REQ parameter."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_cs(&mut self) -> CalvlCsW<DdrDenaliCtl255Spec> {
        CalvlCsW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_255::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_255::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl255Spec;
impl crate::RegisterSpec for DdrDenaliCtl255Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_255::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl255Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_255::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl255Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_255 to value 0"]
impl crate::Resettable for DdrDenaliCtl255Spec {
    const RESET_VALUE: u32 = 0;
}
