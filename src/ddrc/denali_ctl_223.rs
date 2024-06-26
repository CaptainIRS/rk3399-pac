#[doc = "Register `DENALI_CTL_223` reader"]
pub type R = crate::R<DenaliCtl223Spec>;
#[doc = "Register `DENALI_CTL_223` writer"]
pub type W = crate::W<DenaliCtl223Spec>;
#[doc = "Field `SWLVL_LOAD` writer - User request to load delays and execute software leveling. Set to 1 to trigger."]
pub type SwlvlLoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWLVL_START` writer - User request to initiate software leveling of type in the SW_LEVELING_MODE parameter. Set to 1 to trigger."]
pub type SwlvlStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWLVL_EXIT` writer - User request to exit software leveling. Set to 1 to exit."]
pub type SwlvlExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWLVL_OP_DONE` reader - Signals that software leveling is currently in progress. Value of 1 indicates operation complete."]
pub type SwlvlOpDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 24 - Signals that software leveling is currently in progress. Value of 1 indicates operation complete."]
    #[inline(always)]
    pub fn swlvl_op_done(&self) -> SwlvlOpDoneR {
        SwlvlOpDoneR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - User request to load delays and execute software leveling. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn swlvl_load(&mut self) -> SwlvlLoadW<DenaliCtl223Spec> {
        SwlvlLoadW::new(self, 0)
    }
    #[doc = "Bit 8 - User request to initiate software leveling of type in the SW_LEVELING_MODE parameter. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn swlvl_start(&mut self) -> SwlvlStartW<DenaliCtl223Spec> {
        SwlvlStartW::new(self, 8)
    }
    #[doc = "Bit 16 - User request to exit software leveling. Set to 1 to exit."]
    #[inline(always)]
    #[must_use]
    pub fn swlvl_exit(&mut self) -> SwlvlExitW<DenaliCtl223Spec> {
        SwlvlExitW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_223::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_223::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl223Spec;
impl crate::RegisterSpec for DenaliCtl223Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_223::R`](R) reader structure"]
impl crate::Readable for DenaliCtl223Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_223::W`](W) writer structure"]
impl crate::Writable for DenaliCtl223Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_223 to value 0"]
impl crate::Resettable for DenaliCtl223Spec {
    const RESET_VALUE: u32 = 0;
}
