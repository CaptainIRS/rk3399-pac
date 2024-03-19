#[doc = "Register `SDMMC_PLDMND` writer"]
pub type W = crate::W<SdmmcPldmndSpec>;
#[doc = "Field `PD` writer - Poll Demand. If the OWN bit of a descriptor is not set, the FSM\n\ngoes to the Suspend state. The host needs to write any value into\n\nthis register for the IDMAC FSM to resume normal descriptor\n\nfetch operation. This is a write only register."]
pub type PdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Poll Demand. If the OWN bit of a descriptor is not set, the FSM\n\ngoes to the Suspend state. The host needs to write any value into\n\nthis register for the IDMAC FSM to resume normal descriptor\n\nfetch operation. This is a write only register."]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PdW<SdmmcPldmndSpec> {
        PdW::new(self, 0)
    }
}
#[doc = "Poll demand register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_pldmnd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcPldmndSpec;
impl crate::RegisterSpec for SdmmcPldmndSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sdmmc_pldmnd::W`](W) writer structure"]
impl crate::Writable for SdmmcPldmndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_PLDMND to value 0"]
impl crate::Resettable for SdmmcPldmndSpec {
    const RESET_VALUE: u32 = 0;
}
