#[doc = "Register `PROBE_CfgCtl` reader"]
pub type R = crate::R<ProbeCfgCtlSpec>;
#[doc = "Register `PROBE_CfgCtl` writer"]
pub type W = crate::W<ProbeCfgCtlSpec>;
#[doc = "Field `GLOBALEN` reader - Set register field GlobalEn to 1 enable the tracing and statistics\n\ncollection sub-systems of the packet probe."]
pub type GlobalenR = crate::BitReader;
#[doc = "Field `GLOBALEN` writer - Set register field GlobalEn to 1 enable the tracing and statistics\n\ncollection sub-systems of the packet probe."]
pub type GlobalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE` reader - Register field Active is used to inform software that the probe is\n\nactive. Probe configuration is not allowed during the active state.\n\nThis bit is raised when bit GlobalEn is set, and is cleared a few\n\ncycles after setting GlobalEn to zero (probe is Idle)."]
pub type ActiveR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set register field GlobalEn to 1 enable the tracing and statistics\n\ncollection sub-systems of the packet probe."]
    #[inline(always)]
    pub fn globalen(&self) -> GlobalenR {
        GlobalenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Register field Active is used to inform software that the probe is\n\nactive. Probe configuration is not allowed during the active state.\n\nThis bit is raised when bit GlobalEn is set, and is cleared a few\n\ncycles after setting GlobalEn to zero (probe is Idle)."]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set register field GlobalEn to 1 enable the tracing and statistics\n\ncollection sub-systems of the packet probe."]
    #[inline(always)]
    #[must_use]
    pub fn globalen(&mut self) -> GlobalenW<ProbeCfgCtlSpec> {
        GlobalenW::new(self, 0)
    }
}
#[doc = "Register CfgCtl contains global enable and active bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_cfg_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probe_cfg_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProbeCfgCtlSpec;
impl crate::RegisterSpec for ProbeCfgCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`probe_cfg_ctl::R`](R) reader structure"]
impl crate::Readable for ProbeCfgCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`probe_cfg_ctl::W`](W) writer structure"]
impl crate::Writable for ProbeCfgCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROBE_CfgCtl to value 0"]
impl crate::Resettable for ProbeCfgCtlSpec {
    const RESET_VALUE: u32 = 0;
}
